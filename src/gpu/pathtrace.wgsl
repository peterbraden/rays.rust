struct type_10 {
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

struct type_12 {
    member: type_10,
}

struct type_14 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
    member_4: vec4<f32>,
    member_5: vec4<f32>,
    member_6: vec4<u32>,
}

struct type_17 {
    member: array<type_14>,
}

struct type_19 {
    member: vec3<f32>,
    member_1: u32,
    member_2: vec3<f32>,
    member_3: u32,
}

struct type_22 {
    member: array<type_19>,
}

struct type_24 {
    member: u32,
    member_1: u32,
    member_2: u32,
    member_3: u32,
    member_4: vec4<f32>,
    member_5: vec4<f32>,
    member_6: vec4<f32>,
}

struct type_27 {
    member: array<type_24>,
}

struct type_29 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
}

struct type_32 {
    member: array<type_29>,
}

struct type_36 {
    member: array<f32>,
}

struct type_38 {
    member: vec4<f32>,
    member_1: vec4<f32>,
}

struct type_40 {
    member: array<type_38>,
}

struct type_43 {
    member: array<vec4<f32>>,
}

struct type_49 {
    member: f32,
    member_1: u32,
    member_2: vec3<f32>,
    member_3: vec3<f32>,
}

struct type_52 {
    member: f32,
    member_1: f32,
}

struct type_53 {
    member: vec3<f32>,
    member_1: vec3<f32>,
}

struct type_55 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: vec3<f32>,
}

struct type_56 {
    member: u32,
    member_1: type_55,
}

struct type_57 {
    member: u32,
    member_1: type_52,
}

struct type_58 {
    member: u32,
    member_1: vec3<f32>,
}

struct type_59 {
    member: vec3<f32>,
    member_1: f32,
}

struct type_64 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: f32,
}

var<private> global: vec3<u32>;
@group(0) @binding(0) 
var<uniform> global_1: type_12;
@group(0) @binding(1) 
var<storage> global_2: type_17;
@group(0) @binding(2) 
var<storage> global_3: type_22;
@group(0) @binding(3) 
var<storage> global_4: type_27;
@group(0) @binding(4) 
var<storage> global_5: type_32;
@group(0) @binding(5) 
var<storage> global_6: type_36;
@group(0) @binding(6) 
var<storage> global_7: type_40;
@group(0) @binding(7) 
var<storage, read_write> global_8: type_43;

fn function_() {
    var local: array<u32, 100>;
    var local_1: array<u32, 100>;
    var phi_30823_: u32;
    var phi_30223_: bool;
    var phi_489_: u32;
    var phi_492_: vec3<f32>;
    var phi_494_: vec3<f32>;
    var phi_496_: vec3<f32>;
    var phi_498_: vec3<f32>;
    var phi_500_: bool;
    var phi_502_: vec3<f32>;
    var phi_504_: vec3<f32>;
    var phi_506_: u32;
    var phi_508_: bool;
    var phi_510_: f32;
    var phi_512_: bool;
    var phi_514_: vec3<f32>;
    var phi_551_: u32;
    var phi_554_: u32;
    var phi_556_: u32;
    var phi_558_: vec4<f32>;
    var phi_560_: type_49;
    var phi_20035_: bool;
    var phi_20065_: bool;
    var phi_20012_: type_52;
    var phi_20015_: type_52;
    var phi_20016_: bool;
    var phi_20021_: type_52;
    var phi_20122_: bool;
    var phi_20137_: bool;
    var phi_20152_: bool;
    var phi_20167_: bool;
    var phi_20099_: type_52;
    var phi_20102_: type_52;
    var phi_20103_: bool;
    var phi_20108_: type_52;
    var phi_20209_: bool;
    var phi_20224_: bool;
    var phi_20239_: bool;
    var phi_20254_: bool;
    var phi_20186_: type_52;
    var phi_20189_: type_52;
    var phi_20190_: bool;
    var phi_20195_: type_52;
    var phi_19987_: bool;
    var phi_19990_: bool;
    var phi_19991_: bool;
    var phi_974_: u32;
    var phi_975_: u32;
    var phi_976_: u32;
    var phi_977_: u32;
    var phi_978_: u32;
    var phi_979_: u32;
    var phi_980_: u32;
    var phi_981_: u32;
    var phi_982_: u32;
    var phi_863_: vec4<f32>;
    var phi_864_: vec4<f32>;
    var phi_865_: bool;
    var phi_869_: vec4<f32>;
    var phi_870_: bool;
    var phi_774_: bool;
    var phi_811_: bool;
    var phi_817_: f32;
    var phi_827_: vec4<f32>;
    var phi_828_: vec4<f32>;
    var phi_829_: bool;
    var phi_833_: vec4<f32>;
    var phi_834_: bool;
    var phi_838_: vec4<f32>;
    var phi_839_: bool;
    var phi_871_: vec4<f32>;
    var phi_872_: bool;
    var phi_700_: vec4<f32>;
    var phi_701_: vec4<f32>;
    var phi_702_: bool;
    var phi_706_: vec4<f32>;
    var phi_707_: bool;
    var phi_708_: vec4<f32>;
    var phi_709_: bool;
    var phi_710_: bool;
    var phi_718_: vec4<f32>;
    var phi_719_: bool;
    var phi_720_: vec4<f32>;
    var phi_721_: bool;
    var phi_722_: bool;
    var phi_730_: vec4<f32>;
    var phi_731_: bool;
    var phi_881_: vec4<f32>;
    var phi_882_: bool;
    var phi_903_: type_49;
    var phi_905_: type_49;
    var phi_552_: u32;
    var phi_555_: u32;
    var phi_557_: u32;
    var phi_559_: vec4<f32>;
    var phi_561_: type_49;
    var phi_984_: type_49;
    var phi_985_: bool;
    var phi_986_: bool;
    var phi_30208_: bool;
    var phi_19882_: bool;
    var phi_19880_: type_49;
    var local_2: type_49;
    var local_3: vec4<f32>;
    var phi_1154_: type_53;
    var local_4: vec4<f32>;
    var local_5: vec4<f32>;
    var phi_1113_: vec3<f32>;
    var phi_1114_: vec3<f32>;
    var phi_1155_: type_53;
    var local_6: type_49;
    var phi_1160_: type_49;
    var phi_1161_: type_49;
    var phi_1166_: type_49;
    var phi_1176_: u32;
    var phi_1180_: u32;
    var phi_1181_: bool;
    var phi_1184_: bool;
    var phi_530_: u32;
    var phi_531_: u32;
    var phi_30320_: bool;
    var phi_1185_: type_49;
    var phi_1186_: u32;
    var phi_1187_: bool;
    var phi_1189_: u32;
    var phi_1190_: bool;
    var phi_1191_: type_49;
    var phi_1193_: bool;
    var phi_1201_: u32;
    var phi_1202_: u32;
    var phi_1204_: type_49;
    var phi_1361_: f32;
    var phi_20276_: f32;
    var phi_20279_: f32;
    var phi_1363_: f32;
    var phi_1364_: f32;
    var phi_1366_: f32;
    var phi_1368_: type_49;
    var phi_30317_: bool;
    var phi_1370_: vec3<f32>;
    var phi_1373_: i32;
    var phi_1389_: vec3<f32>;
    var phi_1392_: vec3<f32>;
    var phi_1742_: vec3<f32>;
    var phi_1744_: vec3<f32>;
    var phi_1745_: bool;
    var phi_30313_: bool;
    var phi_1755_: vec3<f32>;
    var phi_1758_: i32;
    var phi_1760_: u32;
    var phi_20397_: bool;
    var phi_2183_: u32;
    var phi_2213_: u32;
    var phi_2184_: u32;
    var phi_2214_: bool;
    var local_7: u32;
    var local_8: u32;
    var local_9: u32;
    var phi_2232_: u32;
    var phi_3543_: type_56;
    var phi_3551_: type_55;
    var phi_3553_: type_55;
    var phi_3554_: bool;
    var phi_3555_: type_55;
    var phi_3556_: bool;
    var phi_3557_: bool;
    var phi_20509_: f32;
    var phi_20521_: type_52;
    var phi_20532_: type_57;
    var phi_20533_: type_57;
    var phi_20537_: type_57;
    var phi_20538_: type_57;
    var phi_20539_: bool;
    var phi_20543_: type_57;
    var phi_20430_: type_57;
    var phi_20581_: bool;
    var phi_20661_: f32;
    var phi_20673_: type_52;
    var phi_20684_: type_57;
    var phi_20685_: type_57;
    var phi_20689_: type_57;
    var phi_20690_: type_57;
    var phi_20691_: bool;
    var phi_20695_: type_57;
    var phi_20629_: f32;
    var phi_20733_: bool;
    var phi_20458_: f32;
    var phi_20460_: f32;
    var phi_20469_: type_57;
    var phi_20473_: type_57;
    var phi_20477_: type_57;
    var phi_3575_: vec3<f32>;
    var phi_3578_: vec3<f32>;
    var phi_3580_: vec3<f32>;
    var phi_3582_: vec3<f32>;
    var phi_3584_: u32;
    var phi_3674_: u32;
    var phi_3689_: type_52;
    var phi_3690_: type_52;
    var phi_3691_: type_52;
    var phi_20932_: bool;
    var phi_21059_: bool;
    var phi_21074_: bool;
    var phi_4026_: vec3<f32>;
    var phi_21196_: f32;
    var phi_21208_: type_52;
    var phi_21219_: type_57;
    var phi_21220_: type_57;
    var phi_21224_: type_57;
    var phi_21225_: type_57;
    var phi_21226_: bool;
    var phi_21230_: type_57;
    var phi_21164_: f32;
    var phi_21306_: f32;
    var phi_21318_: type_52;
    var phi_21329_: type_57;
    var phi_21330_: type_57;
    var phi_21334_: type_57;
    var phi_21335_: type_57;
    var phi_21336_: bool;
    var phi_21340_: type_57;
    var phi_21274_: f32;
    var phi_21101_: vec3<f32>;
    var phi_21102_: u32;
    var phi_21472_: bool;
    var phi_21135_: vec3<f32>;
    var phi_21136_: u32;
    var local_10: vec3<f32>;
    var phi_21143_: type_58;
    var phi_21147_: type_58;
    var phi_4085_: vec3<f32>;
    var phi_4086_: vec3<f32>;
    var phi_3576_: vec3<f32>;
    var phi_3579_: vec3<f32>;
    var phi_3581_: vec3<f32>;
    var phi_3583_: vec3<f32>;
    var phi_3585_: u32;
    var phi_30281_: bool;
    var local_11: vec3<f32>;
    var local_12: vec3<f32>;
    var local_13: vec3<f32>;
    var phi_30403_: bool;
    var phi_4106_: type_55;
    var phi_30402_: bool;
    var phi_4107_: type_55;
    var phi_30401_: bool;
    var phi_4130_: vec3<f32>;
    var phi_4131_: i32;
    var phi_4132_: u32;
    var phi_30400_: bool;
    var phi_1756_: vec3<f32>;
    var phi_1759_: i32;
    var phi_1761_: u32;
    var phi_4133_: bool;
    var phi_30399_: bool;
    var phi_4476_: vec3<f32>;
    var phi_4479_: u32;
    var phi_21600_: f32;
    var phi_21615_: f32;
    var phi_21630_: f32;
    var phi_4550_: i32;
    var phi_4580_: i32;
    var phi_4588_: vec3<f32>;
    var phi_4591_: i32;
    var phi_4614_: vec3<f32>;
    var phi_4617_: i32;
    var phi_4615_: vec3<f32>;
    var phi_4618_: i32;
    var phi_4589_: vec3<f32>;
    var phi_4592_: i32;
    var phi_4819_: vec3<f32>;
    var phi_4820_: vec3<f32>;
    var phi_4821_: vec3<f32>;
    var phi_4477_: vec3<f32>;
    var phi_4480_: u32;
    var phi_30404_: bool;
    var local_14: vec3<f32>;
    var phi_30452_: bool;
    var phi_4832_: f32;
    var phi_4856_: f32;
    var phi_4858_: u32;
    var phi_4861_: f32;
    var phi_4967_: f32;
    var phi_4968_: f32;
    var phi_4984_: f32;
    var phi_4859_: u32;
    var phi_4862_: f32;
    var phi_4987_: u32;
    var phi_4990_: f32;
    var phi_5101_: f32;
    var phi_4988_: u32;
    var phi_4991_: f32;
    var local_15: f32;
    var local_16: vec3<f32>;
    var local_17: vec3<f32>;
    var local_18: vec3<f32>;
    var phi_21746_: f32;
    var phi_21758_: type_52;
    var phi_21769_: type_57;
    var phi_21770_: type_57;
    var phi_21774_: type_57;
    var phi_21775_: type_57;
    var phi_21776_: bool;
    var phi_21780_: type_57;
    var phi_21714_: f32;
    var phi_5502_: bool;
    var phi_21906_: f32;
    var phi_21918_: type_52;
    var phi_21929_: type_57;
    var phi_21930_: type_57;
    var phi_21934_: type_57;
    var phi_21935_: type_57;
    var phi_21936_: bool;
    var phi_21940_: type_57;
    var phi_21827_: type_57;
    var phi_21978_: bool;
    var phi_22058_: f32;
    var phi_22070_: type_52;
    var phi_22081_: type_57;
    var phi_22082_: type_57;
    var phi_22086_: type_57;
    var phi_22087_: type_57;
    var phi_22088_: bool;
    var phi_22092_: type_57;
    var phi_22026_: f32;
    var phi_22130_: bool;
    var phi_21855_: f32;
    var phi_21857_: f32;
    var phi_21866_: type_57;
    var phi_21870_: type_57;
    var phi_21874_: type_57;
    var phi_5516_: vec3<f32>;
    var phi_5519_: u32;
    var phi_5609_: u32;
    var phi_5624_: type_52;
    var phi_5625_: type_52;
    var phi_5626_: type_52;
    var phi_22329_: bool;
    var phi_5517_: vec3<f32>;
    var phi_5520_: u32;
    var phi_30431_: bool;
    var local_19: vec3<f32>;
    var local_20: vec3<f32>;
    var local_21: vec3<f32>;
    var phi_30498_: bool;
    var phi_5652_: vec3<f32>;
    var phi_30497_: bool;
    var phi_5654_: vec3<f32>;
    var phi_5656_: bool;
    var phi_30493_: bool;
    var phi_5667_: vec3<f32>;
    var phi_5668_: bool;
    var phi_30492_: bool;
    var phi_5670_: vec3<f32>;
    var phi_5671_: bool;
    var phi_5672_: bool;
    var phi_30491_: bool;
    var phi_5676_: vec3<f32>;
    var phi_5677_: vec3<f32>;
    var phi_5678_: bool;
    var phi_5679_: vec3<f32>;
    var phi_5680_: bool;
    var phi_6009_: vec3<f32>;
    var phi_6012_: u32;
    var phi_22400_: f32;
    var phi_22415_: f32;
    var phi_22430_: f32;
    var phi_6083_: i32;
    var phi_6113_: i32;
    var phi_6121_: vec3<f32>;
    var phi_6124_: i32;
    var phi_6147_: vec3<f32>;
    var phi_6150_: i32;
    var phi_6221_: vec3<f32>;
    var phi_6224_: u32;
    var phi_6347_: vec3<f32>;
    var phi_6222_: vec3<f32>;
    var phi_6225_: u32;
    var phi_6148_: vec3<f32>;
    var phi_6151_: i32;
    var phi_6122_: vec3<f32>;
    var phi_6125_: i32;
    var phi_6352_: vec3<f32>;
    var phi_6353_: vec3<f32>;
    var phi_6354_: vec3<f32>;
    var phi_6010_: vec3<f32>;
    var phi_6013_: u32;
    var phi_30486_: bool;
    var phi_30528_: bool;
    var phi_6374_: vec3<f32>;
    var phi_6587_: u32;
    var phi_6588_: bool;
    var phi_6591_: u32;
    var phi_6592_: bool;
    var phi_6593_: bool;
    var phi_6603_: f32;
    var phi_6606_: f32;
    var phi_6608_: u32;
    var phi_6610_: vec3<f32>;
    var phi_6612_: vec3<f32>;
    var phi_6614_: f32;
    var phi_6616_: f32;
    var phi_6654_: u32;
    var phi_6669_: vec3<f32>;
    var phi_6689_: vec3<f32>;
    var phi_22445_: vec3<f32>;
    var phi_6690_: u32;
    var phi_6691_: vec3<f32>;
    var phi_6692_: u32;
    var phi_6693_: vec3<f32>;
    var phi_6694_: u32;
    var phi_6696_: vec3<f32>;
    var phi_6831_: u32;
    var phi_6834_: f32;
    var phi_6836_: f32;
    var phi_6838_: f32;
    var phi_6840_: vec3<f32>;
    var phi_22500_: f32;
    var phi_22506_: f32;
    var phi_22534_: f32;
    var phi_22540_: f32;
    var phi_22568_: f32;
    var phi_22574_: f32;
    var phi_22602_: f32;
    var phi_22608_: f32;
    var phi_22636_: f32;
    var phi_22642_: f32;
    var phi_22670_: f32;
    var phi_22676_: f32;
    var phi_22704_: f32;
    var phi_22710_: f32;
    var phi_22738_: f32;
    var phi_22744_: f32;
    var phi_6832_: u32;
    var phi_6835_: f32;
    var phi_6837_: f32;
    var phi_6839_: f32;
    var phi_6841_: vec3<f32>;
    var local_22: f32;
    var local_23: f32;
    var local_24: f32;
    var phi_7257_: f32;
    var phi_7309_: vec3<f32>;
    var phi_7310_: f32;
    var phi_7311_: vec3<f32>;
    var phi_7312_: f32;
    var phi_7313_: vec3<f32>;
    var phi_7314_: f32;
    var phi_7316_: vec3<f32>;
    var phi_7317_: f32;
    var phi_7318_: f32;
    var phi_7319_: vec3<f32>;
    var phi_7320_: vec3<f32>;
    var phi_7321_: f32;
    var phi_7322_: f32;
    var phi_7323_: vec3<f32>;
    var phi_7324_: vec3<f32>;
    var phi_7325_: f32;
    var phi_7326_: f32;
    var phi_7327_: vec3<f32>;
    var phi_7328_: vec3<f32>;
    var phi_7329_: f32;
    var phi_7330_: f32;
    var phi_7331_: vec3<f32>;
    var phi_7332_: vec3<f32>;
    var phi_7334_: f32;
    var phi_7335_: f32;
    var phi_7336_: vec3<f32>;
    var phi_7337_: vec3<f32>;
    var phi_7338_: f32;
    var phi_7339_: f32;
    var phi_7340_: f32;
    var phi_7341_: f32;
    var phi_7342_: vec3<f32>;
    var phi_7343_: vec3<f32>;
    var phi_7344_: f32;
    var phi_7345_: f32;
    var phi_6604_: f32;
    var phi_6607_: f32;
    var phi_6609_: u32;
    var phi_6611_: vec3<f32>;
    var phi_6613_: vec3<f32>;
    var phi_6615_: f32;
    var phi_6617_: f32;
    var local_25: f32;
    var local_26: f32;
    var local_27: f32;
    var phi_22820_: bool;
    var local_28: f32;
    var phi_7377_: vec3<f32>;
    var local_29: f32;
    var local_30: f32;
    var local_31: f32;
    var phi_7456_: f32;
    var phi_7457_: vec3<f32>;
    var phi_7458_: f32;
    var phi_7460_: vec3<f32>;
    var phi_7463_: u32;
    var phi_7917_: f32;
    var phi_8097_: u32;
    var phi_8100_: f32;
    var phi_8102_: f32;
    var phi_8104_: f32;
    var phi_8106_: vec3<f32>;
    var phi_22909_: f32;
    var phi_22915_: f32;
    var phi_22943_: f32;
    var phi_22949_: f32;
    var phi_22977_: f32;
    var phi_22983_: f32;
    var phi_23011_: f32;
    var phi_23017_: f32;
    var phi_23045_: f32;
    var phi_23051_: f32;
    var phi_23079_: f32;
    var phi_23085_: f32;
    var phi_23113_: f32;
    var phi_23119_: f32;
    var phi_23147_: f32;
    var phi_23153_: f32;
    var phi_8098_: u32;
    var phi_8101_: f32;
    var phi_8103_: f32;
    var phi_8105_: f32;
    var phi_8107_: vec3<f32>;
    var local_32: f32;
    var local_33: f32;
    var local_34: f32;
    var phi_8520_: f32;
    var phi_23199_: bool;
    var phi_23214_: bool;
    var phi_8613_: vec3<f32>;
    var phi_8614_: vec3<f32>;
    var phi_8615_: vec3<f32>;
    var phi_8616_: vec3<f32>;
    var phi_7461_: vec3<f32>;
    var phi_7464_: u32;
    var phi_8635_: vec3<f32>;
    var phi_8638_: u32;
    var phi_8640_: f32;
    var phi_8947_: vec3<f32>;
    var phi_8948_: u32;
    var phi_8949_: f32;
    var phi_8950_: vec3<f32>;
    var phi_8967_: vec3<f32>;
    var phi_8969_: u32;
    var phi_8972_: vec3<f32>;
    var phi_8974_: f32;
    var phi_8976_: f32;
    var phi_8978_: f32;
    var phi_8998_: bool;
    var phi_9014_: bool;
    var phi_9016_: bool;
    var phi_9018_: bool;
    var phi_9019_: bool;
    var phi_9020_: bool;
    var phi_9021_: bool;
    var phi_9022_: bool;
    var phi_9066_: vec3<f32>;
    var phi_23233_: vec3<f32>;
    var phi_9067_: u32;
    var phi_9068_: vec3<f32>;
    var phi_9069_: u32;
    var phi_9070_: vec3<f32>;
    var phi_9205_: u32;
    var phi_9208_: f32;
    var phi_9210_: f32;
    var phi_9212_: f32;
    var phi_9214_: vec3<f32>;
    var phi_23288_: f32;
    var phi_23294_: f32;
    var phi_23322_: f32;
    var phi_23328_: f32;
    var phi_23356_: f32;
    var phi_23362_: f32;
    var phi_23390_: f32;
    var phi_23396_: f32;
    var phi_23424_: f32;
    var phi_23430_: f32;
    var phi_23458_: f32;
    var phi_23464_: f32;
    var phi_23492_: f32;
    var phi_23498_: f32;
    var phi_23526_: f32;
    var phi_23532_: f32;
    var phi_9206_: u32;
    var phi_9209_: f32;
    var phi_9211_: f32;
    var phi_9213_: f32;
    var phi_9215_: vec3<f32>;
    var local_35: f32;
    var local_36: f32;
    var local_37: f32;
    var phi_9628_: f32;
    var phi_9654_: vec3<f32>;
    var phi_9655_: f32;
    var phi_9656_: f32;
    var phi_9657_: vec3<f32>;
    var phi_9658_: f32;
    var phi_9659_: f32;
    var phi_9660_: vec3<f32>;
    var phi_9661_: f32;
    var phi_9662_: f32;
    var phi_9663_: vec3<f32>;
    var phi_9664_: f32;
    var phi_9665_: f32;
    var phi_9667_: vec3<f32>;
    var phi_9668_: f32;
    var phi_9669_: f32;
    var phi_9670_: f32;
    var phi_8970_: u32;
    var phi_8973_: vec3<f32>;
    var phi_8975_: f32;
    var phi_8977_: f32;
    var phi_8979_: f32;
    var local_38: f32;
    var local_39: f32;
    var local_40: f32;
    var phi_9960_: type_59;
    var local_41: f32;
    var phi_9961_: type_59;
    var local_42: f32;
    var local_43: f32;
    var phi_9962_: type_59;
    var phi_9963_: type_59;
    var phi_8636_: vec3<f32>;
    var phi_8639_: u32;
    var phi_8641_: f32;
    var local_44: f32;
    var local_45: f32;
    var local_46: f32;
    var local_47: f32;
    var phi_9992_: vec3<f32>;
    var local_48: vec3<f32>;
    var local_49: vec3<f32>;
    var local_50: vec3<f32>;
    var local_51: vec3<f32>;
    var local_52: vec3<f32>;
    var local_53: vec3<f32>;
    var phi_30732_: bool;
    var phi_10039_: vec3<f32>;
    var phi_30731_: bool;
    var phi_10040_: vec3<f32>;
    var phi_30730_: bool;
    var phi_10041_: vec3<f32>;
    var phi_30729_: bool;
    var phi_10060_: vec3<f32>;
    var phi_30728_: bool;
    var phi_1371_: vec3<f32>;
    var phi_1374_: i32;
    var phi_30727_: bool;
    var phi_23711_: f32;
    var phi_23723_: type_52;
    var phi_23734_: type_57;
    var phi_23735_: type_57;
    var phi_23739_: type_57;
    var phi_23740_: type_57;
    var phi_23741_: bool;
    var phi_23745_: type_57;
    var phi_23632_: type_57;
    var phi_23783_: bool;
    var phi_23863_: f32;
    var phi_23875_: type_52;
    var phi_23886_: type_57;
    var phi_23887_: type_57;
    var phi_23891_: type_57;
    var phi_23892_: type_57;
    var phi_23893_: bool;
    var phi_23897_: type_57;
    var phi_23831_: f32;
    var phi_23935_: bool;
    var phi_23660_: f32;
    var phi_23662_: f32;
    var phi_23671_: type_57;
    var phi_23675_: type_57;
    var phi_23679_: type_57;
    var phi_23950_: bool;
    var phi_10459_: vec3<f32>;
    var phi_10462_: vec3<f32>;
    var phi_10464_: vec3<f32>;
    var phi_10466_: vec3<f32>;
    var phi_10468_: u32;
    var phi_10558_: u32;
    var phi_10573_: type_52;
    var phi_10574_: type_52;
    var phi_10575_: type_52;
    var phi_24149_: bool;
    var phi_24276_: bool;
    var phi_24291_: bool;
    var phi_10910_: vec3<f32>;
    var phi_10983_: vec3<f32>;
    var phi_10984_: vec3<f32>;
    var phi_10985_: bool;
    var phi_10990_: vec3<f32>;
    var phi_10991_: bool;
    var phi_24415_: bool;
    var phi_24430_: bool;
    var phi_11282_: vec3<f32>;
    var phi_10460_: vec3<f32>;
    var phi_10463_: vec3<f32>;
    var phi_10465_: vec3<f32>;
    var phi_10467_: vec3<f32>;
    var phi_10469_: u32;
    var phi_30734_: bool;
    var local_54: vec3<f32>;
    var local_55: vec3<f32>;
    var local_56: vec3<f32>;
    var local_57: vec3<f32>;
    var local_58: vec3<f32>;
    var local_59: vec3<f32>;
    var local_60: vec3<f32>;
    var local_61: vec3<f32>;
    var phi_33435_: bool;
    var phi_11339_: type_53;
    var phi_33434_: bool;
    var phi_11341_: type_53;
    var phi_11342_: bool;
    var phi_11347_: type_53;
    var local_62: vec3<f32>;
    var local_63: vec3<f32>;
    var local_64: vec3<f32>;
    var phi_33432_: bool;
    var phi_11379_: vec3<f32>;
    var phi_11380_: vec3<f32>;
    var phi_33431_: bool;
    var phi_11381_: vec3<f32>;
    var phi_11382_: vec3<f32>;
    var phi_11413_: i32;
    var phi_11416_: u32;
    var phi_11470_: bool;
    var phi_11476_: bool;
    var phi_11483_: u32;
    var phi_11485_: i32;
    var phi_11486_: u32;
    var phi_11414_: i32;
    var phi_11417_: u32;
    var phi_11488_: bool;
    var local_65: u32;
    var phi_11533_: vec3<f32>;
    var phi_11550_: vec3<f32>;
    var phi_24606_: bool;
    var phi_11581_: f32;
    var phi_11693_: vec3<f32>;
    var phi_11699_: f32;
    var phi_24679_: bool;
    var phi_24668_: f32;
    var phi_11789_: vec3<f32>;
    var phi_11790_: vec3<f32>;
    var phi_11791_: vec3<f32>;
    var phi_31432_: u32;
    var phi_11793_: vec3<f32>;
    var phi_11794_: vec3<f32>;
    var phi_11795_: vec3<f32>;
    var phi_11796_: bool;
    var phi_11811_: vec3<f32>;
    var phi_11812_: vec3<f32>;
    var phi_31430_: u32;
    var phi_11814_: vec3<f32>;
    var phi_11815_: vec3<f32>;
    var phi_11816_: vec3<f32>;
    var phi_31429_: u32;
    var phi_11817_: vec3<f32>;
    var phi_11818_: vec3<f32>;
    var phi_11819_: vec3<f32>;
    var phi_11820_: bool;
    var phi_31428_: u32;
    var phi_11821_: vec3<f32>;
    var phi_11822_: vec3<f32>;
    var phi_11823_: vec3<f32>;
    var phi_11824_: bool;
    var phi_11825_: bool;
    var phi_24752_: bool;
    var phi_24914_: bool;
    var phi_24929_: bool;
    var phi_24944_: bool;
    var phi_24971_: bool;
    var phi_24986_: bool;
    var phi_25001_: bool;
    var phi_25028_: bool;
    var phi_25043_: bool;
    var phi_25058_: bool;
    var phi_12004_: f32;
    var phi_12012_: f32;
    var phi_12020_: f32;
    var phi_25255_: bool;
    var phi_25244_: f32;
    var phi_12127_: vec3<f32>;
    var phi_25283_: bool;
    var phi_12128_: vec3<f32>;
    var phi_12129_: vec3<f32>;
    var phi_12130_: vec3<f32>;
    var phi_31811_: vec3<f32>;
    var phi_31735_: vec3<f32>;
    var phi_31657_: u32;
    var phi_31576_: f32;
    var phi_12132_: vec3<f32>;
    var phi_12133_: vec3<f32>;
    var phi_12134_: bool;
    var phi_25298_: bool;
    var phi_25349_: bool;
    var phi_25338_: f32;
    var phi_31783_: vec3<f32>;
    var phi_11654_: vec3<f32>;
    var phi_32132_: vec3<f32>;
    var phi_31927_: f32;
    var phi_31782_: vec3<f32>;
    var phi_31474_: u32;
    var phi_11656_: vec3<f32>;
    var phi_11657_: bool;
    var phi_32078_: vec3<f32>;
    var phi_31887_: f32;
    var phi_31809_: vec3<f32>;
    var phi_31733_: vec3<f32>;
    var phi_31655_: u32;
    var phi_31574_: f32;
    var phi_31498_: u32;
    var phi_31425_: u32;
    var phi_12142_: vec3<f32>;
    var phi_12143_: vec3<f32>;
    var phi_12144_: vec3<f32>;
    var phi_12145_: bool;
    var phi_25415_: bool;
    var phi_12214_: vec3<f32>;
    var phi_31487_: u32;
    var phi_12216_: vec3<f32>;
    var phi_12217_: bool;
    var phi_31486_: u32;
    var phi_12218_: vec3<f32>;
    var phi_12219_: bool;
    var phi_31484_: u32;
    var phi_12225_: vec3<f32>;
    var phi_12226_: bool;
    var phi_12256_: vec3<f32>;
    var phi_12257_: vec3<f32>;
    var phi_12258_: u32;
    var phi_31482_: u32;
    var phi_12260_: vec3<f32>;
    var phi_12261_: vec3<f32>;
    var phi_12262_: u32;
    var phi_12263_: bool;
    var phi_12264_: bool;
    var phi_31481_: u32;
    var phi_12265_: vec3<f32>;
    var phi_12266_: vec3<f32>;
    var phi_12267_: u32;
    var phi_12269_: bool;
    var phi_12270_: bool;
    var phi_12271_: bool;
    var phi_32075_: vec3<f32>;
    var phi_31884_: f32;
    var phi_31806_: vec3<f32>;
    var phi_31730_: vec3<f32>;
    var phi_31652_: u32;
    var phi_31571_: f32;
    var phi_31495_: u32;
    var phi_31480_: u32;
    var phi_12272_: vec3<f32>;
    var phi_12273_: vec3<f32>;
    var phi_12274_: u32;
    var phi_12275_: vec3<f32>;
    var phi_12276_: bool;
    var phi_12277_: bool;
    var phi_12278_: bool;
    var phi_12279_: bool;
    var phi_33411_: bool;
    var phi_31479_: u32;
    var phi_12288_: u32;
    var phi_12291_: vec3<f32>;
    var phi_12293_: vec3<f32>;
    var phi_12295_: u32;
    var phi_12297_: vec3<f32>;
    var phi_12299_: vec3<f32>;
    var phi_14988_: u32;
    var phi_14991_: u32;
    var phi_14989_: u32;
    var phi_14992_: u32;
    var local_66: u32;
    var phi_15029_: u32;
    var phi_15032_: u32;
    var phi_15030_: u32;
    var phi_15033_: u32;
    var local_67: u32;
    var local_68: u32;
    var phi_15109_: f32;
    var local_69: u32;
    var local_70: u32;
    var phi_15158_: f32;
    var phi_15314_: f32;
    var phi_15321_: vec3<f32>;
    var phi_33774_: u32;
    var phi_15323_: f32;
    var phi_15324_: vec3<f32>;
    var phi_15325_: vec3<f32>;
    var phi_14461_: u32;
    var phi_14464_: u32;
    var phi_14462_: u32;
    var phi_14465_: u32;
    var local_71: u32;
    var local_72: u32;
    var phi_14857_: f32;
    var phi_14860_: f32;
    var phi_25633_: bool;
    var phi_25614_: f32;
    var phi_14861_: f32;
    var phi_25648_: bool;
    var phi_25671_: bool;
    var phi_25686_: bool;
    var phi_14612_: f32;
    var phi_25701_: bool;
    var phi_25716_: bool;
    var phi_25731_: bool;
    var phi_25796_: bool;
    var phi_25884_: bool;
    var phi_25865_: f32;
    var phi_14677_: type_64;
    var phi_14863_: type_64;
    var phi_14941_: bool;
    var phi_14950_: vec3<f32>;
    var phi_14952_: vec3<f32>;
    var phi_14953_: bool;
    var phi_14958_: f32;
    var phi_14959_: vec3<f32>;
    var phi_33782_: u32;
    var phi_14960_: f32;
    var phi_14961_: vec3<f32>;
    var phi_14962_: vec3<f32>;
    var phi_14963_: f32;
    var phi_33781_: u32;
    var phi_14964_: f32;
    var phi_14965_: vec3<f32>;
    var phi_14966_: vec3<f32>;
    var phi_14967_: f32;
    var phi_33773_: u32;
    var phi_15326_: f32;
    var phi_15327_: vec3<f32>;
    var phi_15328_: vec3<f32>;
    var phi_15329_: f32;
    var phi_25899_: bool;
    var phi_26009_: f32;
    var phi_26037_: f32;
    var phi_26065_: f32;
    var phi_26199_: bool;
    var phi_26214_: bool;
    var phi_26229_: bool;
    var phi_26256_: bool;
    var phi_26271_: bool;
    var phi_26286_: bool;
    var phi_26313_: bool;
    var phi_26328_: bool;
    var phi_26343_: bool;
    var phi_12633_: f32;
    var phi_12641_: f32;
    var phi_12649_: f32;
    var phi_26540_: bool;
    var phi_26529_: f32;
    var phi_12756_: vec3<f32>;
    var phi_26568_: bool;
    var phi_12757_: vec3<f32>;
    var phi_12769_: type_59;
    var phi_12770_: type_59;
    var phi_12771_: bool;
    var phi_12776_: type_59;
    var phi_26605_: bool;
    var phi_26635_: bool;
    var phi_26624_: f32;
    var phi_26674_: vec3<f32>;
    var phi_12804_: type_59;
    var phi_12805_: type_59;
    var phi_26696_: vec3<f32>;
    var phi_12806_: type_59;
    var phi_12807_: type_59;
    var phi_12808_: bool;
    var phi_12812_: type_59;
    var phi_26728_: bool;
    var phi_26799_: bool;
    var phi_13632_: f32;
    var phi_26814_: bool;
    var phi_26829_: bool;
    var phi_14105_: vec3<f32>;
    var phi_14106_: vec3<f32>;
    var phi_14107_: bool;
    var phi_14112_: vec3<f32>;
    var phi_14113_: bool;
    var phi_26953_: bool;
    var phi_26968_: bool;
    var phi_14404_: vec3<f32>;
    var phi_14406_: vec3<f32>;
    var phi_14433_: vec3<f32>;
    var phi_14434_: vec3<f32>;
    var phi_13250_: vec3<f32>;
    var phi_13251_: vec3<f32>;
    var phi_13252_: bool;
    var phi_13257_: vec3<f32>;
    var phi_13258_: bool;
    var phi_27130_: bool;
    var phi_27145_: bool;
    var phi_13549_: vec3<f32>;
    var phi_13551_: vec3<f32>;
    var phi_33816_: u32;
    var phi_14435_: f32;
    var phi_14436_: vec3<f32>;
    var phi_14437_: vec3<f32>;
    var phi_33815_: u32;
    var phi_14438_: f32;
    var phi_14439_: vec3<f32>;
    var phi_14440_: vec3<f32>;
    var phi_33772_: u32;
    var phi_15330_: f32;
    var phi_15331_: vec3<f32>;
    var phi_15332_: vec3<f32>;
    var phi_15333_: f32;
    var phi_12395_: f32;
    var phi_12396_: f32;
    var phi_33771_: u32;
    var phi_15334_: f32;
    var phi_15335_: vec3<f32>;
    var phi_15336_: vec3<f32>;
    var phi_15337_: f32;
    var phi_27198_: bool;
    var phi_27308_: f32;
    var phi_27336_: f32;
    var phi_27364_: f32;
    var phi_27498_: bool;
    var phi_27513_: bool;
    var phi_27528_: bool;
    var phi_27555_: bool;
    var phi_27570_: bool;
    var phi_27585_: bool;
    var phi_27612_: bool;
    var phi_27627_: bool;
    var phi_27642_: bool;
    var phi_15560_: f32;
    var phi_15568_: f32;
    var phi_15576_: f32;
    var phi_27839_: bool;
    var phi_27828_: f32;
    var phi_15683_: vec3<f32>;
    var phi_27867_: bool;
    var phi_15684_: vec3<f32>;
    var phi_15696_: type_59;
    var phi_15697_: type_59;
    var phi_15698_: bool;
    var phi_15703_: type_59;
    var phi_27904_: bool;
    var phi_27934_: bool;
    var phi_27923_: f32;
    var phi_27973_: vec3<f32>;
    var phi_15731_: type_59;
    var phi_15732_: type_59;
    var phi_27995_: vec3<f32>;
    var phi_15733_: type_59;
    var phi_15734_: type_59;
    var phi_15735_: bool;
    var phi_15739_: type_59;
    var phi_28009_: f32;
    var phi_28012_: f32;
    var phi_15748_: f32;
    var phi_15782_: u32;
    var phi_15785_: u32;
    var phi_15787_: u32;
    var phi_15791_: type_49;
    var phi_28109_: bool;
    var phi_28139_: bool;
    var phi_28086_: type_52;
    var phi_28089_: type_52;
    var phi_28090_: bool;
    var phi_28095_: type_52;
    var phi_28196_: bool;
    var phi_28211_: bool;
    var phi_28226_: bool;
    var phi_28241_: bool;
    var phi_28173_: type_52;
    var phi_28176_: type_52;
    var phi_28177_: bool;
    var phi_28182_: type_52;
    var phi_28283_: bool;
    var phi_28298_: bool;
    var phi_28313_: bool;
    var phi_28328_: bool;
    var phi_28260_: type_52;
    var phi_28263_: type_52;
    var phi_28264_: bool;
    var phi_28269_: type_52;
    var phi_28062_: bool;
    var phi_28065_: bool;
    var phi_28066_: bool;
    var phi_16198_: u32;
    var phi_16199_: u32;
    var phi_16200_: u32;
    var phi_16201_: u32;
    var phi_16202_: u32;
    var phi_16203_: u32;
    var phi_16204_: u32;
    var phi_16205_: u32;
    var phi_16206_: u32;
    var phi_16087_: vec4<f32>;
    var phi_16088_: vec4<f32>;
    var phi_16089_: bool;
    var phi_16093_: vec4<f32>;
    var phi_16094_: bool;
    var phi_15999_: bool;
    var phi_16036_: bool;
    var phi_16042_: f32;
    var phi_16052_: vec4<f32>;
    var phi_16053_: vec4<f32>;
    var phi_16054_: bool;
    var phi_16058_: vec4<f32>;
    var phi_16059_: bool;
    var phi_16063_: vec4<f32>;
    var phi_16064_: bool;
    var phi_16095_: vec4<f32>;
    var phi_16096_: bool;
    var phi_15928_: vec4<f32>;
    var phi_15929_: vec4<f32>;
    var phi_15930_: bool;
    var phi_15934_: vec4<f32>;
    var phi_15935_: bool;
    var phi_15936_: vec4<f32>;
    var phi_15937_: bool;
    var phi_15938_: bool;
    var phi_15946_: vec4<f32>;
    var phi_15947_: bool;
    var phi_15948_: vec4<f32>;
    var phi_15949_: bool;
    var phi_15950_: bool;
    var phi_15958_: vec4<f32>;
    var phi_15959_: bool;
    var phi_16105_: vec4<f32>;
    var phi_16106_: bool;
    var phi_16127_: type_49;
    var phi_16129_: type_49;
    var phi_16135_: u32;
    var phi_15783_: u32;
    var phi_15786_: u32;
    var phi_15788_: u32;
    var phi_15792_: type_49;
    var phi_16208_: type_49;
    var phi_16209_: bool;
    var phi_16210_: bool;
    var phi_33331_: bool;
    var phi_19885_: bool;
    var phi_19884_: type_49;
    var phi_16382_: type_49;
    var phi_16387_: type_49;
    var phi_16438_: vec3<f32>;
    var phi_16439_: vec3<f32>;
    var phi_16442_: vec3<f32>;
    var phi_16443_: vec3<f32>;
    var phi_16444_: vec3<f32>;
    var phi_16445_: vec3<f32>;
    var phi_33718_: bool;
    var phi_16446_: u32;
    var phi_16447_: vec3<f32>;
    var phi_16448_: vec3<f32>;
    var phi_16449_: vec3<f32>;
    var phi_16450_: vec3<f32>;
    var phi_33717_: bool;
    var phi_16451_: u32;
    var phi_16452_: vec3<f32>;
    var phi_16453_: vec3<f32>;
    var phi_16454_: vec3<f32>;
    var phi_16455_: vec3<f32>;
    var phi_33960_: u32;
    var phi_33716_: bool;
    var phi_12289_: u32;
    var phi_12292_: vec3<f32>;
    var phi_12294_: vec3<f32>;
    var phi_12296_: u32;
    var phi_12298_: vec3<f32>;
    var phi_12300_: vec3<f32>;
    var phi_33729_: u32;
    var phi_33715_: bool;
    var phi_28464_: bool;
    var phi_28494_: bool;
    var phi_28483_: f32;
    var phi_28414_: bool;
    var phi_28550_: vec3<f32>;
    var phi_28706_: bool;
    var phi_28721_: bool;
    var phi_28736_: bool;
    var phi_28645_: vec3<f32>;
    var phi_28648_: vec3<f32>;
    var phi_28649_: bool;
    var phi_28419_: vec3<f32>;
    var phi_28423_: vec3<f32>;
    var phi_28424_: bool;
    var phi_28836_: bool;
    var phi_28810_: vec3<f32>;
    var phi_28431_: vec3<f32>;
    var phi_28851_: bool;
    var phi_28961_: f32;
    var phi_28989_: f32;
    var phi_29017_: f32;
    var phi_29151_: bool;
    var phi_29166_: bool;
    var phi_29181_: bool;
    var phi_29208_: bool;
    var phi_29223_: bool;
    var phi_29238_: bool;
    var phi_29265_: bool;
    var phi_29280_: bool;
    var phi_29295_: bool;
    var phi_16672_: f32;
    var phi_16680_: f32;
    var phi_16688_: f32;
    var phi_29492_: bool;
    var phi_29481_: f32;
    var phi_16795_: vec3<f32>;
    var phi_29520_: bool;
    var phi_16796_: vec3<f32>;
    var phi_16808_: type_59;
    var phi_16809_: type_59;
    var phi_16810_: bool;
    var phi_16815_: type_59;
    var phi_29557_: bool;
    var phi_29587_: bool;
    var phi_29576_: f32;
    var phi_29626_: vec3<f32>;
    var phi_16843_: type_59;
    var phi_16844_: type_59;
    var phi_29648_: vec3<f32>;
    var phi_16845_: type_59;
    var phi_16846_: type_59;
    var phi_16847_: bool;
    var phi_16851_: type_59;
    var phi_29673_: bool;
    var phi_16927_: vec3<f32>;
    var phi_35329_: u32;
    var phi_16929_: vec3<f32>;
    var phi_16930_: bool;
    var phi_35328_: u32;
    var phi_16931_: vec3<f32>;
    var phi_16932_: bool;
    var phi_35326_: u32;
    var phi_16938_: vec3<f32>;
    var phi_16939_: bool;
    var phi_16968_: vec3<f32>;
    var phi_16980_: vec3<f32>;
    var phi_16981_: u32;
    var phi_16982_: vec3<f32>;
    var phi_35324_: u32;
    var phi_16984_: vec3<f32>;
    var phi_16985_: u32;
    var phi_16986_: vec3<f32>;
    var phi_16987_: vec3<f32>;
    var phi_16988_: bool;
    var phi_35323_: u32;
    var phi_16989_: vec3<f32>;
    var phi_16990_: u32;
    var phi_16991_: vec3<f32>;
    var phi_16992_: vec3<f32>;
    var phi_16993_: bool;
    var phi_16994_: bool;
    var phi_35321_: u32;
    var phi_17003_: vec3<f32>;
    var phi_17004_: vec3<f32>;
    var phi_17005_: vec3<f32>;
    var phi_17006_: u32;
    var phi_17007_: f32;
    var phi_17008_: vec3<f32>;
    var phi_17009_: bool;
    var phi_35319_: u32;
    var phi_35221_: bool;
    var phi_17014_: u32;
    var phi_17015_: vec3<f32>;
    var phi_17016_: vec3<f32>;
    var phi_17017_: vec3<f32>;
    var phi_17020_: vec3<f32>;
    var phi_17021_: vec3<f32>;
    var phi_17026_: u32;
    var phi_17027_: vec3<f32>;
    var phi_17028_: vec3<f32>;
    var phi_17029_: vec3<f32>;
    var phi_17030_: vec3<f32>;
    var phi_17031_: vec3<f32>;
    var phi_17032_: vec3<f32>;
    var phi_17033_: u32;
    var phi_17034_: f32;
    var phi_17035_: vec3<f32>;
    var phi_17036_: bool;
    var phi_17037_: bool;
    var phi_35317_: u32;
    var phi_35219_: bool;
    var phi_17059_: u32;
    var phi_17060_: vec3<f32>;
    var phi_17061_: vec3<f32>;
    var phi_17062_: vec3<f32>;
    var phi_17063_: vec3<f32>;
    var phi_17064_: bool;
    var phi_17065_: vec3<f32>;
    var phi_17066_: vec3<f32>;
    var phi_17067_: u32;
    var phi_17068_: bool;
    var phi_17069_: f32;
    var phi_17070_: bool;
    var phi_17071_: vec3<f32>;
    var phi_17072_: bool;
    var phi_17073_: bool;
    var phi_17074_: bool;
    var phi_35316_: u32;
    var phi_35218_: bool;
    var phi_17075_: vec3<f32>;
    var phi_17076_: vec3<f32>;
    var phi_17077_: f32;
    var phi_17078_: f32;
    var phi_17079_: f32;
    var phi_17080_: vec3<f32>;
    var phi_17081_: type_14;
    var phi_17082_: u32;
    var phi_17083_: u32;
    var phi_17084_: vec3<f32>;
    var phi_17085_: vec3<f32>;
    var phi_17086_: vec3<f32>;
    var phi_17087_: vec3<f32>;
    var phi_17088_: bool;
    var phi_17089_: vec3<f32>;
    var phi_17090_: vec3<f32>;
    var phi_17091_: u32;
    var phi_17092_: bool;
    var phi_17093_: f32;
    var phi_17094_: bool;
    var phi_17095_: vec3<f32>;
    var phi_17098_: bool;
    var phi_17099_: bool;
    var phi_17100_: bool;
    var phi_17101_: bool;
    var phi_35315_: u32;
    var phi_35217_: bool;
    var phi_17102_: vec3<f32>;
    var phi_17103_: vec3<f32>;
    var phi_17104_: f32;
    var phi_17105_: f32;
    var phi_17106_: f32;
    var phi_17107_: vec3<f32>;
    var phi_17108_: type_14;
    var phi_17109_: u32;
    var phi_490_: u32;
    var phi_493_: vec3<f32>;
    var phi_495_: vec3<f32>;
    var phi_497_: vec3<f32>;
    var phi_499_: vec3<f32>;
    var phi_501_: bool;
    var phi_503_: vec3<f32>;
    var phi_505_: vec3<f32>;
    var phi_507_: u32;
    var phi_509_: bool;
    var phi_511_: f32;
    var phi_513_: bool;
    var phi_515_: vec3<f32>;
    var phi_17112_: bool;
    var phi_17114_: bool;
    var phi_17115_: bool;
    var phi_17116_: bool;
    var phi_35216_: bool;
    var phi_19906_: type_49;
    var phi_19905_: u32;
    var phi_19904_: vec3<f32>;
    var phi_19903_: bool;
    var phi_19902_: bool;
    var phi_19901_: bool;
    var phi_19900_: bool;
    var phi_19899_: vec3<f32>;
    var phi_19897_: u32;
    var phi_19895_: type_14;
    var phi_19893_: vec3<f32>;
    var phi_19892_: f32;
    var phi_19891_: f32;
    var phi_19890_: f32;
    var phi_19888_: vec3<f32>;
    var local_73: bool;
    var local_74: bool;
    var local_75: bool;
    var local_76: bool;
    var local_77: bool;
    var local_78: vec3<f32>;
    var local_79: vec3<f32>;
    var local_80: vec3<f32>;
    var phi_29709_: bool;
    var phi_17175_: f32;
    var local_81: f32;
    var local_82: f32;
    var local_83: f32;
    var phi_29727_: f32;
    var phi_29730_: f32;
    var phi_17177_: f32;
    var local_84: vec3<f32>;
    var local_85: vec3<f32>;
    var local_86: vec3<f32>;
    var phi_29741_: bool;
    var phi_29756_: bool;
    var local_87: vec3<f32>;
    var local_88: vec3<f32>;
    var local_89: vec3<f32>;
    var phi_17657_: vec3<f32>;
    var phi_17658_: vec3<f32>;
    var phi_17659_: bool;
    var phi_17664_: vec3<f32>;
    var phi_17665_: bool;
    var phi_29880_: bool;
    var phi_29895_: bool;
    var phi_17956_: vec3<f32>;
    var phi_17958_: vec3<f32>;
    var phi_17985_: vec3<f32>;
    var local_90: vec3<f32>;
    var local_91: vec3<f32>;
    var local_92: vec3<f32>;
    var phi_18007_: vec3<f32>;
    var phi_18008_: vec3<f32>;
    var local_93: vec3<f32>;
    var local_94: vec3<f32>;
    var local_95: vec3<f32>;
    var phi_18056_: bool;
    var phi_18065_: vec3<f32>;
    var phi_18067_: vec3<f32>;
    var phi_18068_: bool;
    var local_96: bool;
    var local_97: vec3<f32>;
    var local_98: vec3<f32>;
    var local_99: vec3<f32>;
    var phi_18232_: f32;
    var phi_18235_: f32;
    var local_100: vec3<f32>;
    var local_101: vec3<f32>;
    var local_102: vec3<f32>;
    var phi_30021_: bool;
    var phi_30002_: f32;
    var phi_18236_: f32;
    var local_103: f32;
    var local_104: f32;
    var local_105: f32;
    var phi_30039_: f32;
    var phi_30042_: f32;
    var phi_18239_: f32;
    var phi_18240_: f32;
    var phi_18241_: f32;
    var phi_18263_: vec3<f32>;
    var phi_18308_: bool;
    var phi_18309_: bool;
    var phi_18310_: bool;
    var local_106: type_49;
    var local_107: vec3<f32>;
    var local_108: vec3<f32>;
    var local_109: f32;
    var local_110: vec3<f32>;
    var local_111: vec3<f32>;
    var local_112: vec3<f32>;
    var local_113: vec3<f32>;
    var local_114: vec3<f32>;
    var local_115: vec3<f32>;
    var local_116: vec3<f32>;
    var local_117: vec3<f32>;
    var local_118: type_49;
    var local_119: u32;
    var local_120: vec3<f32>;
    var local_121: vec3<f32>;
    var local_122: vec3<f32>;
    var local_123: vec3<f32>;
    var local_124: u32;
    var local_125: vec3<f32>;
    var local_126: vec3<f32>;
    var local_127: vec3<f32>;
    var local_128: vec3<f32>;
    var local_129: u32;
    var local_130: vec3<f32>;
    var local_131: vec3<f32>;

    switch bitcast<i32>(0u) {
        default: {
            let _e272 = global;
            let _e286 = (_e272.x + (_e272.y * 4194240u));
            let _e290 = global_1.member.member[0u];
            let _e294 = global_1.member.member[1u];
            if (_e286 >= (_e290 * _e294)) {
            } else {
                if (_e286 < arrayLength((&global_7.member))) {
                } else {
                    break;
                }
                let _e301 = global_7.member[_e286].member;
                let _e305 = global_7.member[_e286].member_1;
                if (_e305.w < 0.5f) {
                } else {
                    let _e311 = global_1.member.member[2u];
                    let _e315 = global_1.member.member_7[0u];
                    let _e319 = ((((_e311 * 2654435769u) ^ _e315) * 747796405u) + 2891336453u);
                    let _e327 = (((_e319 >> bitcast<u32>((((_e319 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e319) * 277803737u);
                    let _e333 = (((_e286 ^ ((_e327 >> bitcast<u32>(22i)) ^ _e327)) * 747796405u) + 2891336453u);
                    let _e341 = (((_e333 >> bitcast<u32>((((_e333 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e333) * 277803737u);
                    let _e358 = (1f / sqrt((((_e305.x * _e305.x) + (_e305.y * _e305.y)) + (_e305.z * _e305.z))));
                    let _e365 = global_1.member.member[3u];
                    let _e368 = global_1.member.member_3[3u];
                    let _e371 = global_1.member.member_1[2u];
                    phi_30823_ = ((_e341 >> bitcast<u32>(22i)) ^ _e341);
                    phi_30223_ = false;
                    phi_489_ = 0u;
                    phi_492_ = vec3<f32>(0f, 0f, 0f);
                    phi_494_ = vec3<f32>(1f, 1f, 1f);
                    phi_496_ = vec3<f32>(0f, 0f, 0f);
                    phi_498_ = vec3<f32>((_e305.x * _e358), (_e305.y * _e358), (_e305.z * _e358));
                    phi_500_ = false;
                    phi_502_ = vec3<f32>(0f, 1f, 0f);
                    phi_504_ = vec3<f32>(0f, 0f, 0f);
                    phi_506_ = 0u;
                    phi_508_ = false;
                    phi_510_ = 0f;
                    phi_512_ = true;
                    phi_514_ = vec3<f32>(_e301.x, _e301.y, _e301.z);
                    loop {
                        let _e373 = phi_30823_;
                        let _e375 = phi_30223_;
                        let _e377 = phi_489_;
                        let _e379 = phi_492_;
                        let _e381 = phi_494_;
                        let _e383 = phi_496_;
                        let _e385 = phi_498_;
                        let _e387 = phi_500_;
                        let _e389 = phi_502_;
                        let _e391 = phi_504_;
                        let _e393 = phi_506_;
                        let _e395 = phi_508_;
                        let _e397 = phi_510_;
                        let _e399 = phi_512_;
                        let _e401 = phi_514_;
                        local_73 = _e387;
                        local_74 = _e387;
                        local_75 = _e387;
                        local_76 = _e387;
                        local_77 = _e399;
                        local_78 = _e385;
                        local_79 = _e385;
                        local_80 = _e385;
                        local_81 = _e397;
                        local_82 = _e397;
                        local_83 = _e397;
                        local_84 = _e385;
                        local_85 = _e385;
                        local_86 = _e385;
                        local_87 = _e401;
                        local_88 = _e401;
                        local_89 = _e401;
                        local_131 = _e401;
                        local_90 = _e381;
                        local_91 = _e381;
                        local_92 = _e381;
                        local_93 = _e385;
                        local_94 = _e385;
                        local_95 = _e385;
                        local_96 = _e399;
                        local_97 = _e401;
                        local_98 = _e401;
                        local_99 = _e401;
                        local_100 = _e401;
                        local_101 = _e401;
                        local_102 = _e401;
                        local_103 = _e397;
                        local_104 = _e397;
                        local_105 = _e397;
                        if _e387 {
                            if _e387 {
                                phi_531_ = u32();
                            } else {
                                if _e387 {
                                    phi_530_ = _e377;
                                } else {
                                    phi_530_ = (_e377 + 1u);
                                }
                                let _e1035 = phi_530_;
                                phi_531_ = _e1035;
                            }
                            let _e1037 = phi_531_;
                            phi_30320_ = _e375;
                            phi_1185_ = type_49(300000000000000000000000000000000000000f, 4294967295u, vec3<f32>(0f, 1f, 0f), vec3<f32>(0f, 1f, 0f));
                            phi_1186_ = _e1037;
                            phi_1187_ = select(true, bool(), _e387);
                            phi_1189_ = u32();
                            phi_1190_ = bool();
                            phi_1191_ = type_49();
                            phi_1193_ = select(true, false, _e387);
                        } else {
                            let _e404 = global_1.member.member_1[0u];
                            let _e407 = global_1.member.member_1[1u];
                            let _e409 = (vec3<f32>(1f, 1f, 1f) / _e385);
                            local_1 = array<u32, 100>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                            phi_551_ = _e404;
                            phi_554_ = select(0u, 1u, (_e404 > 0u));
                            phi_556_ = (_e404 + _e407);
                            phi_558_ = vec4<f32>(0f, 0f, 0f, 0f);
                            phi_560_ = type_49(300000000000000000000000000000000000000f, 4294967295u, vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                            loop {
                                let _e413 = phi_551_;
                                let _e415 = phi_554_;
                                let _e417 = phi_556_;
                                let _e419 = phi_558_;
                                let _e421 = phi_560_;
                                local_2 = _e421;
                                local_3 = _e419;
                                local_4 = _e419;
                                local_5 = _e419;
                                local_6 = _e421;
                                local_106 = _e421;
                                if (_e413 < _e417) {
                                    let _e593 = global_2.member[_e413];
                                    if (_e593.member_6.x == 0u) {
                                        let _e707 = (_e593.member_1.x - _e593.member.x);
                                        let _e708 = (_e593.member_1.y - _e593.member.y);
                                        let _e709 = (_e593.member_1.z - _e593.member.z);
                                        let _e714 = (_e593.member_2.x - _e593.member.x);
                                        let _e715 = (_e593.member_2.y - _e593.member.y);
                                        let _e716 = (_e593.member_2.z - _e593.member.z);
                                        let _e721 = ((_e385.y * _e716) - (_e715 * _e385.z));
                                        let _e725 = ((_e385.z * _e714) - (_e716 * _e385.x));
                                        let _e728 = ((_e385.x * _e715) - (_e714 * _e385.y));
                                        let _e733 = (((_e707 * _e721) + (_e708 * _e725)) + (_e709 * _e728));
                                        if (abs(_e733) < 0.000000000001f) {
                                            phi_730_ = vec4<f32>();
                                            phi_731_ = true;
                                        } else {
                                            let _e736 = (1f / _e733);
                                            let _e738 = (_e401.x - _e593.member.x);
                                            let _e740 = (_e401.y - _e593.member.y);
                                            let _e742 = (_e401.z - _e593.member.z);
                                            let _e748 = ((((_e738 * _e721) + (_e740 * _e725)) + (_e742 * _e728)) * _e736);
                                            if (_e748 < 0f) {
                                                phi_720_ = vec4<f32>();
                                                phi_721_ = true;
                                                phi_722_ = false;
                                            } else {
                                                let _e750 = (_e748 > 1f);
                                                if _e750 {
                                                    phi_718_ = vec4<f32>();
                                                    phi_719_ = false;
                                                } else {
                                                    let _e753 = ((_e740 * _e709) - (_e708 * _e742));
                                                    let _e756 = ((_e742 * _e707) - (_e709 * _e738));
                                                    let _e759 = ((_e738 * _e708) - (_e707 * _e740));
                                                    let _e765 = ((((_e385.x * _e753) + (_e385.y * _e756)) + (_e385.z * _e759)) * _e736);
                                                    if (_e765 < 0f) {
                                                        phi_708_ = vec4<f32>();
                                                        phi_709_ = true;
                                                        phi_710_ = false;
                                                    } else {
                                                        let _e768 = ((_e748 + _e765) > 1f);
                                                        if _e768 {
                                                            phi_706_ = vec4<f32>();
                                                            phi_707_ = false;
                                                        } else {
                                                            let _e774 = ((((_e714 * _e753) + (_e715 * _e756)) + (_e716 * _e759)) * _e736);
                                                            if (_e774 <= 0f) {
                                                                phi_701_ = vec4<f32>();
                                                                phi_702_ = true;
                                                            } else {
                                                                let _e776 = (_e774 >= _e421.member);
                                                                if _e776 {
                                                                    phi_700_ = vec4<f32>();
                                                                } else {
                                                                    phi_700_ = vec4<f32>(_e774, _e748, _e765, 1f);
                                                                }
                                                                let _e779 = phi_700_;
                                                                phi_701_ = _e779;
                                                                phi_702_ = _e776;
                                                            }
                                                            let _e781 = phi_701_;
                                                            let _e783 = phi_702_;
                                                            phi_706_ = _e781;
                                                            phi_707_ = _e783;
                                                        }
                                                        let _e785 = phi_706_;
                                                        let _e787 = phi_707_;
                                                        phi_708_ = _e785;
                                                        phi_709_ = _e768;
                                                        phi_710_ = _e787;
                                                    }
                                                    let _e789 = phi_708_;
                                                    let _e791 = phi_709_;
                                                    let _e793 = phi_710_;
                                                    phi_718_ = _e789;
                                                    phi_719_ = select(_e793, true, _e791);
                                                }
                                                let _e796 = phi_718_;
                                                let _e798 = phi_719_;
                                                phi_720_ = _e796;
                                                phi_721_ = _e750;
                                                phi_722_ = _e798;
                                            }
                                            let _e800 = phi_720_;
                                            let _e802 = phi_721_;
                                            let _e804 = phi_722_;
                                            phi_730_ = _e800;
                                            phi_731_ = select(_e804, true, _e802);
                                        }
                                        let _e807 = phi_730_;
                                        let _e809 = phi_731_;
                                        phi_881_ = _e807;
                                        phi_882_ = _e809;
                                    } else {
                                        if (_e593.member_6.x == 1u) {
                                            let _e624 = (_e401.x - _e593.member.x);
                                            let _e626 = (_e401.y - _e593.member.y);
                                            let _e628 = (_e401.z - _e593.member.z);
                                            let _e636 = (((_e624 * _e385.x) + (_e626 * _e385.y)) + (_e628 * _e385.z));
                                            if (_e636 > 0f) {
                                                phi_774_ = select(true, false, ((((_e624 * _e624) + (_e626 * _e626)) + (_e628 * _e628)) > (_e593.member.w * _e593.member.w)));
                                            } else {
                                                phi_774_ = true;
                                            }
                                            let _e649 = phi_774_;
                                            if _e649 {
                                                let _e653 = (_e624 - (_e385.x * _e636));
                                                let _e654 = (_e626 - (_e385.y * _e636));
                                                let _e655 = (_e628 - (_e385.z * _e636));
                                                let _e664 = ((_e593.member.w * _e593.member.w) - (((_e653 * _e653) + (_e654 * _e654)) + (_e655 * _e655)));
                                                if (_e664 < 0f) {
                                                    phi_833_ = vec4<f32>();
                                                    phi_834_ = true;
                                                } else {
                                                    let _e666 = sqrt(_e664);
                                                    let _e667 = -(_e636);
                                                    let _e668 = (_e667 - _e666);
                                                    if (_e668 <= 0f) {
                                                        phi_811_ = true;
                                                    } else {
                                                        phi_811_ = (_e668 >= _e421.member);
                                                    }
                                                    let _e672 = phi_811_;
                                                    if _e672 {
                                                        phi_817_ = (_e667 + _e666);
                                                    } else {
                                                        phi_817_ = _e668;
                                                    }
                                                    let _e675 = phi_817_;
                                                    if (_e675 <= 0f) {
                                                        phi_828_ = vec4<f32>();
                                                        phi_829_ = true;
                                                    } else {
                                                        let _e677 = (_e675 >= _e421.member);
                                                        if _e677 {
                                                            phi_827_ = vec4<f32>();
                                                        } else {
                                                            phi_827_ = vec4<f32>(_e675, 0f, 0f, 1f);
                                                        }
                                                        let _e680 = phi_827_;
                                                        phi_828_ = _e680;
                                                        phi_829_ = _e677;
                                                    }
                                                    let _e682 = phi_828_;
                                                    let _e684 = phi_829_;
                                                    phi_833_ = _e682;
                                                    phi_834_ = _e684;
                                                }
                                                let _e686 = phi_833_;
                                                let _e688 = phi_834_;
                                                phi_838_ = _e686;
                                                phi_839_ = _e688;
                                            } else {
                                                phi_838_ = vec4<f32>();
                                                phi_839_ = true;
                                            }
                                            let _e690 = phi_838_;
                                            let _e692 = phi_839_;
                                            phi_871_ = _e690;
                                            phi_872_ = _e692;
                                        } else {
                                            if (abs(_e385.y) < 0.000000000001f) {
                                                phi_869_ = vec4<f32>();
                                                phi_870_ = true;
                                            } else {
                                                let _e605 = ((_e593.member.x - _e401.y) / _e385.y);
                                                if (_e605 <= 0f) {
                                                    phi_864_ = vec4<f32>();
                                                    phi_865_ = true;
                                                } else {
                                                    let _e607 = (_e605 >= _e421.member);
                                                    if _e607 {
                                                        phi_863_ = vec4<f32>();
                                                    } else {
                                                        phi_863_ = vec4<f32>(_e605, 0f, 0f, 1f);
                                                    }
                                                    let _e610 = phi_863_;
                                                    phi_864_ = _e610;
                                                    phi_865_ = _e607;
                                                }
                                                let _e612 = phi_864_;
                                                let _e614 = phi_865_;
                                                phi_869_ = _e612;
                                                phi_870_ = _e614;
                                            }
                                            let _e616 = phi_869_;
                                            let _e618 = phi_870_;
                                            phi_871_ = _e616;
                                            phi_872_ = _e618;
                                        }
                                        let _e694 = phi_871_;
                                        let _e696 = phi_872_;
                                        phi_881_ = _e694;
                                        phi_882_ = select(true, false, select(true, false, _e696));
                                    }
                                    let _e811 = phi_881_;
                                    let _e813 = phi_882_;
                                    let _e816 = select(vec4<f32>(0f, 0f, 0f, 0f), _e811, vec4(select(true, false, _e813)));
                                    let _e818 = (_e816.w > 0f);
                                    if _e818 {
                                        let _e824 = type_49(_e816.x, _e421.member_1, _e421.member_2, _e421.member_3);
                                        let _e829 = type_49(_e824.member, _e413, _e824.member_2, _e824.member_3);
                                        phi_903_ = _e829;
                                        phi_905_ = _e829;
                                    } else {
                                        phi_903_ = type_49();
                                        phi_905_ = _e421;
                                    }
                                    let _e831 = phi_903_;
                                    let _e833 = phi_905_;
                                    phi_552_ = (_e413 + 1u);
                                    phi_555_ = _e415;
                                    phi_557_ = _e417;
                                    phi_559_ = select(_e419, _e816, vec4(_e818));
                                    phi_561_ = _e833;
                                    phi_984_ = _e831;
                                    phi_985_ = true;
                                    phi_986_ = false;
                                } else {
                                    let _e423 = (_e415 == 0u);
                                    if _e423 {
                                        phi_980_ = u32();
                                        phi_981_ = u32();
                                        phi_982_ = u32();
                                    } else {
                                        let _e424 = (_e415 - 1u);
                                        let _e425 = (_e424 < 100u);
                                        if _e425 {
                                        } else {
                                            phi_30208_ = true;
                                            phi_19882_ = bool();
                                            phi_19880_ = type_49();
                                            break;
                                        }
                                        let _e427 = local_1[_e424];
                                        let _e429 = global_3.member[_e427];
                                        let _e443 = ((_e429.member.x - _e401.x) * _e409.x);
                                        let _e445 = ((_e429.member.y - _e401.y) * _e409.y);
                                        let _e447 = ((_e429.member.z - _e401.z) * _e409.z);
                                        let _e454 = ((_e429.member_2.x - _e401.x) * _e409.x);
                                        let _e455 = ((_e429.member_2.y - _e401.y) * _e409.y);
                                        let _e456 = ((_e429.member_2.z - _e401.z) * _e409.z);
                                        if (_e443 == _e443) {
                                            let _e458 = (_e454 == _e454);
                                            if _e458 {
                                                let _e459 = (_e443 != _e443);
                                                if _e459 {
                                                    phi_20035_ = true;
                                                } else {
                                                    phi_20035_ = (_e454 <= _e443);
                                                }
                                                let _e462 = phi_20035_;
                                                let _e463 = select(_e443, _e454, _e462);
                                                if _e459 {
                                                    phi_20065_ = true;
                                                } else {
                                                    phi_20065_ = (_e454 >= _e443);
                                                }
                                                let _e468 = phi_20065_;
                                                let _e469 = select(_e443, _e454, _e468);
                                                phi_20012_ = type_52(select(-300000000000000000000000000000000000000f, _e463, (_e463 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e469, (_e469 <= 300000000000000000000000000000000000000f)));
                                            } else {
                                                phi_20012_ = type_52();
                                            }
                                            let _e474 = phi_20012_;
                                            phi_20015_ = _e474;
                                            phi_20016_ = select(true, false, _e458);
                                        } else {
                                            phi_20015_ = type_52();
                                            phi_20016_ = true;
                                        }
                                        let _e477 = phi_20015_;
                                        let _e479 = phi_20016_;
                                        if _e479 {
                                            phi_20021_ = type_52(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                        } else {
                                            phi_20021_ = _e477;
                                        }
                                        let _e481 = phi_20021_;
                                        if (_e445 == _e445) {
                                            let _e485 = (_e455 == _e455);
                                            if _e485 {
                                                let _e486 = (_e445 != _e445);
                                                if _e486 {
                                                    phi_20122_ = true;
                                                } else {
                                                    phi_20122_ = (_e455 <= _e445);
                                                }
                                                let _e489 = phi_20122_;
                                                let _e490 = select(_e445, _e455, _e489);
                                                if (_e481.member != _e481.member) {
                                                    phi_20137_ = true;
                                                } else {
                                                    phi_20137_ = (_e490 >= _e481.member);
                                                }
                                                let _e494 = phi_20137_;
                                                if _e486 {
                                                    phi_20152_ = true;
                                                } else {
                                                    phi_20152_ = (_e455 >= _e445);
                                                }
                                                let _e498 = phi_20152_;
                                                let _e499 = select(_e445, _e455, _e498);
                                                if (_e481.member_1 != _e481.member_1) {
                                                    phi_20167_ = true;
                                                } else {
                                                    phi_20167_ = (_e499 <= _e481.member_1);
                                                }
                                                let _e503 = phi_20167_;
                                                phi_20099_ = type_52(select(_e481.member, _e490, _e494), select(_e481.member_1, _e499, _e503));
                                            } else {
                                                phi_20099_ = type_52();
                                            }
                                            let _e507 = phi_20099_;
                                            phi_20102_ = _e507;
                                            phi_20103_ = select(true, false, _e485);
                                        } else {
                                            phi_20102_ = type_52();
                                            phi_20103_ = true;
                                        }
                                        let _e510 = phi_20102_;
                                        let _e512 = phi_20103_;
                                        if _e512 {
                                            phi_20108_ = _e481;
                                        } else {
                                            phi_20108_ = _e510;
                                        }
                                        let _e514 = phi_20108_;
                                        if (_e447 == _e447) {
                                            let _e518 = (_e456 == _e456);
                                            if _e518 {
                                                let _e519 = (_e447 != _e447);
                                                if _e519 {
                                                    phi_20209_ = true;
                                                } else {
                                                    phi_20209_ = (_e456 <= _e447);
                                                }
                                                let _e522 = phi_20209_;
                                                let _e523 = select(_e447, _e456, _e522);
                                                if (_e514.member != _e514.member) {
                                                    phi_20224_ = true;
                                                } else {
                                                    phi_20224_ = (_e523 >= _e514.member);
                                                }
                                                let _e527 = phi_20224_;
                                                if _e519 {
                                                    phi_20239_ = true;
                                                } else {
                                                    phi_20239_ = (_e456 >= _e447);
                                                }
                                                let _e531 = phi_20239_;
                                                let _e532 = select(_e447, _e456, _e531);
                                                if (_e514.member_1 != _e514.member_1) {
                                                    phi_20254_ = true;
                                                } else {
                                                    phi_20254_ = (_e532 <= _e514.member_1);
                                                }
                                                let _e536 = phi_20254_;
                                                phi_20186_ = type_52(select(_e514.member, _e523, _e527), select(_e514.member_1, _e532, _e536));
                                            } else {
                                                phi_20186_ = type_52();
                                            }
                                            let _e540 = phi_20186_;
                                            phi_20189_ = _e540;
                                            phi_20190_ = select(true, false, _e518);
                                        } else {
                                            phi_20189_ = type_52();
                                            phi_20190_ = true;
                                        }
                                        let _e543 = phi_20189_;
                                        let _e545 = phi_20190_;
                                        if _e545 {
                                            phi_20195_ = _e514;
                                        } else {
                                            phi_20195_ = _e543;
                                        }
                                        let _e547 = phi_20195_;
                                        if (_e547.member_1 < 0f) {
                                            phi_19990_ = bool();
                                            phi_19991_ = true;
                                        } else {
                                            let _e551 = (_e547.member > _e547.member_1);
                                            if _e551 {
                                                phi_19987_ = bool();
                                            } else {
                                                phi_19987_ = (_e547.member > _e421.member);
                                            }
                                            let _e554 = phi_19987_;
                                            phi_19990_ = _e554;
                                            phi_19991_ = _e551;
                                        }
                                        let _e556 = phi_19990_;
                                        let _e558 = phi_19991_;
                                        if !(select(_e556, true, _e558)) {
                                            if (_e429.member_3 > 0u) {
                                                phi_974_ = _e429.member_1;
                                                phi_975_ = _e424;
                                                phi_976_ = (_e429.member_1 + _e429.member_3);
                                            } else {
                                                let _e563 = (_e415 + 1u);
                                                let _e564 = (_e563 <= 100u);
                                                if _e564 {
                                                    if _e425 {
                                                    } else {
                                                        phi_30208_ = true;
                                                        phi_19882_ = bool();
                                                        phi_19880_ = type_49();
                                                        break;
                                                    }
                                                    local_1[_e424] = _e429.member_1;
                                                    if (_e415 < 100u) {
                                                    } else {
                                                        phi_30208_ = true;
                                                        phi_19882_ = bool();
                                                        phi_19880_ = type_49();
                                                        break;
                                                    }
                                                    local_1[_e415] = (_e427 + 1u);
                                                }
                                                phi_974_ = _e413;
                                                phi_975_ = select(_e424, _e563, _e564);
                                                phi_976_ = _e417;
                                            }
                                            let _e573 = phi_974_;
                                            let _e575 = phi_975_;
                                            let _e577 = phi_976_;
                                            phi_977_ = _e573;
                                            phi_978_ = _e575;
                                            phi_979_ = _e577;
                                        } else {
                                            phi_977_ = _e413;
                                            phi_978_ = _e424;
                                            phi_979_ = _e417;
                                        }
                                        let _e579 = phi_977_;
                                        let _e581 = phi_978_;
                                        let _e583 = phi_979_;
                                        phi_980_ = _e579;
                                        phi_981_ = _e581;
                                        phi_982_ = _e583;
                                    }
                                    let _e585 = phi_980_;
                                    let _e587 = phi_981_;
                                    let _e589 = phi_982_;
                                    phi_552_ = _e585;
                                    phi_555_ = _e587;
                                    phi_557_ = _e589;
                                    phi_559_ = _e419;
                                    phi_561_ = _e421;
                                    phi_984_ = type_49();
                                    phi_985_ = select(true, false, _e423);
                                    phi_986_ = _e423;
                                }
                                let _e838 = phi_552_;
                                let _e840 = phi_555_;
                                let _e842 = phi_557_;
                                let _e844 = phi_559_;
                                let _e846 = phi_561_;
                                let _e848 = phi_984_;
                                let _e850 = phi_985_;
                                let _e852 = phi_986_;
                                phi_551_ = _e838;
                                phi_554_ = _e840;
                                phi_556_ = _e842;
                                phi_558_ = _e844;
                                phi_560_ = _e846;
                                phi_30208_ = _e375;
                                phi_19882_ = _e852;
                                phi_19880_ = _e848;
                                if !(_e850) {
                                    break;
                                }
                            }
                            let _e855 = phi_30208_;
                            let _e857 = phi_19882_;
                            let _e859 = phi_19880_;
                            phi_35216_ = _e855;
                            phi_19906_ = type_49();
                            phi_19905_ = u32();
                            phi_19904_ = vec3<f32>();
                            phi_19903_ = bool();
                            phi_19902_ = bool();
                            phi_19901_ = bool();
                            phi_19900_ = bool();
                            phi_19899_ = vec3<f32>();
                            phi_19897_ = u32();
                            phi_19895_ = type_14();
                            phi_19893_ = vec3<f32>();
                            phi_19892_ = f32();
                            phi_19891_ = f32();
                            phi_19890_ = f32();
                            phi_19888_ = vec3<f32>();
                            if _e855 {
                                break;
                            }
                            if _e857 {
                                let _e861 = local_2;
                                if (_e861.member_1 != 4294967295u) {
                                    let _e865 = global_2.member[_e861.member_1];
                                    if (_e865.member_6.x == 0u) {
                                        let _e914 = (_e865.member_1.x - _e865.member.x);
                                        let _e915 = (_e865.member_1.y - _e865.member.y);
                                        let _e916 = (_e865.member_1.z - _e865.member.z);
                                        let _e921 = (_e865.member_2.x - _e865.member.x);
                                        let _e922 = (_e865.member_2.y - _e865.member.y);
                                        let _e923 = (_e865.member_2.z - _e865.member.z);
                                        let _e926 = ((_e915 * _e923) - (_e922 * _e916));
                                        let _e929 = ((_e916 * _e921) - (_e923 * _e914));
                                        let _e932 = ((_e914 * _e922) - (_e921 * _e915));
                                        let _e939 = (1f / sqrt((((_e926 * _e926) + (_e929 * _e929)) + (_e932 * _e932))));
                                        let _e943 = vec3<f32>((_e926 * _e939), (_e929 * _e939), (_e932 * _e939));
                                        if (_e865.member_6.w == 1u) {
                                            let _e952 = local_4;
                                            let _e956 = local_5;
                                            let _e958 = ((1f - _e952.y) - _e956.z);
                                            let _e979 = (((_e865.member_3.x * _e958) + (_e865.member_4.x * _e952.y)) + (_e865.member_5.x * _e956.z));
                                            let _e980 = (((_e865.member_3.y * _e958) + (_e865.member_4.y * _e952.y)) + (_e865.member_5.y * _e956.z));
                                            let _e981 = (((_e865.member_3.z * _e958) + (_e865.member_4.z * _e952.y)) + (_e865.member_5.z * _e956.z));
                                            let _e986 = (((_e979 * _e979) + (_e980 * _e980)) + (_e981 * _e981));
                                            if (_e986 > 0f) {
                                                let _e989 = (1f / sqrt(_e986));
                                                phi_1113_ = vec3<f32>((_e979 * _e989), (_e980 * _e989), (_e981 * _e989));
                                            } else {
                                                phi_1113_ = _e943;
                                            }
                                            let _e995 = phi_1113_;
                                            phi_1114_ = _e995;
                                        } else {
                                            phi_1114_ = _e943;
                                        }
                                        let _e997 = phi_1114_;
                                        phi_1155_ = type_53(_e943, _e997);
                                    } else {
                                        if (_e865.member_6.x == 1u) {
                                            let _e876 = local_3;
                                            let _e902 = vec3<f32>((((_e401.x + (_e385.x * _e876.x)) - _e865.member.x) / _e865.member.w), (((_e401.y + (_e385.y * _e876.x)) - _e865.member.y) / _e865.member.w), (((_e401.z + (_e385.z * _e876.x)) - _e865.member.z) / _e865.member.w));
                                            phi_1154_ = type_53(_e902, _e902);
                                        } else {
                                            let _e873 = select(vec3<f32>(0f, 1f, 0f), vec3<f32>(-0f, -1f, -0f), vec3((_e385.y > 0f)));
                                            phi_1154_ = type_53(_e873, _e873);
                                        }
                                        let _e905 = phi_1154_;
                                        phi_1155_ = _e905;
                                    }
                                    let _e1000 = phi_1155_;
                                    let _e1004 = local_6;
                                    let _e1009 = type_49(_e1004.member, _e1004.member_1, _e1000.member, _e1004.member_3);
                                    phi_1160_ = type_49(_e1009.member, _e1009.member_1, _e1009.member_2, _e1000.member_1);
                                } else {
                                    let _e13880 = local_106;
                                    phi_1160_ = _e13880;
                                }
                                let _e1016 = phi_1160_;
                                phi_1161_ = _e1016;
                            } else {
                                phi_1161_ = type_49();
                            }
                            let _e1018 = phi_1161_;
                            if select(true, false, _e857) {
                                phi_1166_ = _e859;
                            } else {
                                phi_1166_ = _e1018;
                            }
                            let _e1021 = phi_1166_;
                            if _e387 {
                                phi_1180_ = u32();
                                phi_1181_ = bool();
                                phi_1184_ = false;
                            } else {
                                let _e1023 = (_e1021.member_1 == 4294967295u);
                                if _e387 {
                                    phi_1176_ = _e377;
                                } else {
                                    phi_1176_ = (_e377 + 1u);
                                }
                                let _e1026 = phi_1176_;
                                phi_1180_ = _e1026;
                                phi_1181_ = _e1023;
                                phi_1184_ = _e1023;
                            }
                            let _e1028 = phi_1180_;
                            let _e1030 = phi_1181_;
                            let _e1032 = phi_1184_;
                            phi_30320_ = _e855;
                            phi_1185_ = _e1021;
                            phi_1186_ = _e1028;
                            phi_1187_ = _e1030;
                            phi_1189_ = _e1028;
                            phi_1190_ = _e1030;
                            phi_1191_ = _e1021;
                            phi_1193_ = _e1032;
                        }
                        let _e1041 = phi_30320_;
                        let _e1043 = phi_1185_;
                        let _e1045 = phi_1186_;
                        let _e1047 = phi_1187_;
                        let _e1049 = phi_1189_;
                        let _e1051 = phi_1190_;
                        let _e1053 = phi_1191_;
                        let _e1055 = phi_1193_;
                        if _e387 {
                            if _e387 {
                                phi_1201_ = _e377;
                            } else {
                                phi_1201_ = (_e377 + 1u);
                            }
                            let _e1058 = phi_1201_;
                            phi_1202_ = _e1058;
                            phi_1204_ = _e1043;
                        } else {
                            phi_1202_ = _e1049;
                            phi_1204_ = _e1053;
                        }
                        let _e1060 = phi_1202_;
                        let _e1062 = phi_1204_;
                        let _e1064 = select(_e1055, false, _e387);
                        if _e1064 {
                            if _e399 {
                                phi_1364_ = 1f;
                            } else {
                                if ((_e368 & 2u) == 0u) {
                                    phi_1363_ = 1f;
                                } else {
                                    let _e1069 = global_1.member.member_3[1u];
                                    let _e1072 = global_1.member.member_3[2u];
                                    let _e1077 = ((atan2(_e385.z, _e385.x) * 0.15915494f) + 0.5f);
                                    let _e1082 = select(_e385.y, -1f, (_e385.y < -1f));
                                    let _e1085 = acos(select(_e1082, 1f, (_e1082 > 1f)));
                                    let _e1087 = f32(_e1069);
                                    let _e1088 = ((_e1077 - floor(_e1077)) * _e1087);
                                    let _e1093 = select(select(u32(_e1088), 0u, (_e1088 < 0f)), 4294967295u, (_e1088 > 4294967000f));
                                    let _e1094 = (_e1069 - 1u);
                                    let _e1097 = f32(_e1072);
                                    let _e1098 = ((_e1085 * 0.31830987f) * _e1097);
                                    let _e1103 = select(select(u32(_e1098), 0u, (_e1098 < 0f)), 4294967295u, (_e1098 > 4294967000f));
                                    let _e1104 = (_e1072 - 1u);
                                    let _e1106 = select(_e1103, _e1104, (_e1104 < _e1103));
                                    let _e1107 = sin(_e1085);
                                    if (_e1107 <= 0f) {
                                        phi_1361_ = 0f;
                                    } else {
                                        let _e1111 = global_1.member.member_2[3u];
                                        let _e1114 = global_1.member.member_3[0u];
                                        let _e1118 = (_e1111 + _e1106);
                                        let _e1121 = global_6.member[(_e1118 + 1u)];
                                        let _e1123 = global_6.member[_e1118];
                                        let _e1126 = ((_e1114 + (_e1106 * (_e1069 + 1u))) + select(_e1093, _e1094, (_e1094 < _e1093)));
                                        let _e1129 = global_6.member[(_e1126 + 1u)];
                                        let _e1131 = global_6.member[_e1126];
                                        phi_1361_ = ((((_e1121 - _e1123) * _e1097) * ((_e1129 - _e1131) * _e1087)) / (19.73921f * _e1107));
                                    }
                                    let _e1138 = phi_1361_;
                                    if (_e397 >= 300000000000000000000000000000000000000f) {
                                        phi_20279_ = 1f;
                                    } else {
                                        let _e1140 = (_e397 * _e397);
                                        let _e1142 = (_e1140 + (_e1138 * _e1138));
                                        if (_e1142 == 0f) {
                                            phi_20276_ = 0f;
                                        } else {
                                            phi_20276_ = (_e1140 / _e1142);
                                        }
                                        let _e1146 = phi_20276_;
                                        phi_20279_ = _e1146;
                                    }
                                    let _e1148 = phi_20279_;
                                    phi_1363_ = _e1148;
                                }
                                let _e1150 = phi_1363_;
                                phi_1364_ = _e1150;
                            }
                            let _e1152 = phi_1364_;
                            phi_1366_ = _e1152;
                            phi_1368_ = _e1043;
                        } else {
                            phi_1366_ = 1f;
                            phi_1368_ = _e1062;
                        }
                        let _e1154 = phi_1366_;
                        let _e1156 = phi_1368_;
                        let _e1157 = select(_e1060, _e1045, _e1064);
                        let _e1158 = select(select(_e1051, false, _e387), _e1047, _e1064);
                        phi_30317_ = _e1041;
                        phi_1370_ = _e379;
                        phi_1373_ = 0i;
                        loop {
                            let _e1160 = phi_30317_;
                            let _e1162 = phi_1370_;
                            let _e1164 = phi_1373_;
                            local_62 = _e1162;
                            local_63 = _e1162;
                            local_64 = _e1162;
                            local_116 = _e1162;
                            local_117 = _e1162;
                            local_130 = _e1162;
                            let _e1165 = (_e1164 < 2i);
                            if _e1165 {
                                let _e1166 = (_e1164 == 0i);
                                if _e1166 {
                                    phi_1392_ = _e383;
                                } else {
                                    if _e1158 {
                                        phi_1389_ = (_e381 * _e1154);
                                    } else {
                                        phi_1389_ = vec3<f32>(0f, 0f, 0f);
                                    }
                                    let _e1169 = phi_1389_;
                                    phi_1392_ = _e1169;
                                }
                                let _e1171 = phi_1392_;
                                let _e1172 = vec3(_e1166);
                                let _e1173 = select(_e385, _e389, _e1172);
                                let _e1174 = select(_e401, _e391, _e1172);
                                if ((((_e1171.x * _e1171.x) + (_e1171.y * _e1171.y)) + (_e1171.z * _e1171.z)) > 0f) {
                                    let _e1186 = global_1.member.member_8[0u];
                                    let _e1189 = global_1.member.member_8[3u];
                                    if (_e1186 == 4294967295u) {
                                        let _e1191 = (_e1189 == 4294967295u);
                                        if _e1191 {
                                            let _e1194 = global_1.member.member_3[1u];
                                            let _e1197 = global_1.member.member_3[2u];
                                            let _e1202 = ((atan2(_e1173.z, _e1173.x) * 0.15915494f) + 0.5f);
                                            let _e1207 = (((_e1202 - floor(_e1202)) * f32(_e1194)) - 0.5f);
                                            let _e1210 = select(_e1173.y, -1f, (_e1173.y < -1f));
                                            let _e1217 = (((acos(select(_e1210, 1f, (_e1210 > 1f))) * 0.31830987f) * f32(_e1197)) - 0.5f);
                                            let _e1218 = (_e1197 - 1u);
                                            let _e1219 = f32(_e1218);
                                            if (0f <= _e1219) {
                                            } else {
                                                phi_30727_ = true;
                                                break;
                                            }
                                            let _e1222 = select(_e1217, 0f, (_e1217 < 0f));
                                            let _e1224 = select(_e1222, _e1219, (_e1222 > _e1219));
                                            let _e1225 = floor(_e1207);
                                            let _e1226 = floor(_e1224);
                                            let _e1227 = (_e1207 - _e1225);
                                            let _e1228 = (_e1224 - _e1226);
                                            let _e1239 = (_e1194 == 0u);
                                            if _e1239 {
                                                phi_30727_ = true;
                                                break;
                                            }
                                            let _e1240 = (bitcast<u32>((select(0i, select(select(i32(_e1225), i32(-2147483648), (_e1225 < -2147483600f)), 2147483647i, (_e1225 > 2147483500f)), (_e1225 == _e1225)) + bitcast<i32>(_e1194))) % _e1194);
                                            let _e1245 = select(select(u32(_e1226), 0u, (_e1226 < 0f)), 4294967295u, (_e1226 > 4294967000f));
                                            let _e1246 = (_e1245 + 1u);
                                            let _e1251 = global_1.member.member_2[2u];
                                            let _e1252 = (_e1245 * _e1194);
                                            if _e1239 {
                                                phi_30727_ = true;
                                                break;
                                            }
                                            let _e1253 = (_e1240 % _e1194);
                                            let _e1256 = (_e1251 + (3u * (_e1252 + _e1253)));
                                            let _e1258 = global_6.member[_e1256];
                                            let _e1261 = global_6.member[(_e1256 + 1u)];
                                            let _e1264 = global_6.member[(_e1256 + 2u)];
                                            if _e1239 {
                                                phi_30727_ = true;
                                                break;
                                            }
                                            let _e1267 = ((_e1240 + 1u) % _e1194);
                                            let _e1270 = (_e1251 + (3u * (_e1252 + _e1267)));
                                            let _e1272 = global_6.member[_e1270];
                                            let _e1275 = global_6.member[(_e1270 + 1u)];
                                            let _e1278 = global_6.member[(_e1270 + 2u)];
                                            let _e1286 = (vec3<f32>(_e1258, _e1261, _e1264) + vec3<f32>(((_e1272 - _e1258) * _e1227), ((_e1275 - _e1261) * _e1227), ((_e1278 - _e1264) * _e1227)));
                                            let _e1287 = (select(_e1246, _e1218, (_e1218 < _e1246)) * _e1194);
                                            if _e1239 {
                                                phi_30727_ = true;
                                                break;
                                            }
                                            let _e1290 = (_e1251 + (3u * (_e1287 + _e1253)));
                                            let _e1292 = global_6.member[_e1290];
                                            let _e1295 = global_6.member[(_e1290 + 1u)];
                                            let _e1298 = global_6.member[(_e1290 + 2u)];
                                            if _e1239 {
                                                phi_30727_ = true;
                                                break;
                                            }
                                            let _e1302 = (_e1251 + (3u * (_e1287 + _e1267)));
                                            let _e1304 = global_6.member[_e1302];
                                            let _e1307 = global_6.member[(_e1302 + 1u)];
                                            let _e1310 = global_6.member[(_e1302 + 2u)];
                                            let _e1318 = (vec3<f32>(_e1292, _e1295, _e1298) + vec3<f32>(((_e1304 - _e1292) * _e1227), ((_e1307 - _e1295) * _e1227), ((_e1310 - _e1298) * _e1227)));
                                            phi_1742_ = (_e1286 + vec3<f32>(((_e1318.x - _e1286.x) * _e1228), ((_e1318.y - _e1286.y) * _e1228), ((_e1318.z - _e1286.z) * _e1228)));
                                        } else {
                                            phi_1742_ = vec3<f32>();
                                        }
                                        let _e1334 = phi_1742_;
                                        phi_1744_ = _e1334;
                                        phi_1745_ = select(true, false, _e1191);
                                    } else {
                                        phi_1744_ = vec3<f32>();
                                        phi_1745_ = true;
                                    }
                                    let _e1337 = phi_1744_;
                                    let _e1339 = phi_1745_;
                                    if _e1339 {
                                        let _e1340 = (_e1189 != 4294967295u);
                                        if (_e1186 != 4294967295u) {
                                            phi_30313_ = _e1160;
                                            phi_1755_ = vec3<f32>(0f, 0f, 0f);
                                            phi_1758_ = 0i;
                                            phi_1760_ = _e1186;
                                            loop {
                                                let _e1343 = phi_30313_;
                                                let _e1345 = phi_1755_;
                                                let _e1347 = phi_1758_;
                                                let _e1349 = phi_1760_;
                                                local_16 = _e1345;
                                                local_17 = _e1345;
                                                local_18 = _e1345;
                                                local_110 = _e1345;
                                                local_111 = _e1345;
                                                if (_e1349 != 4294967295u) {
                                                    let _e1351 = (_e1347 < 2i);
                                                    if _e1351 {
                                                        let _e1353 = global_6.member[_e1349];
                                                        let _e1356 = global_6.member[(_e1349 + 1u)];
                                                        let _e1359 = global_6.member[(_e1349 + 2u)];
                                                        let _e1362 = global_6.member[(_e1349 + 3u)];
                                                        let _e1365 = global_6.member[(_e1349 + 4u)];
                                                        let _e1368 = global_6.member[(_e1349 + 5u)];
                                                        let _e1371 = global_6.member[(_e1349 + 6u)];
                                                        let _e1374 = global_6.member[(_e1349 + 7u)];
                                                        let _e1377 = global_6.member[(_e1349 + 8u)];
                                                        let _e1380 = global_6.member[(_e1349 + 9u)];
                                                        let _e1383 = global_6.member[(_e1349 + 10u)];
                                                        let _e1386 = global_6.member[(_e1349 + 11u)];
                                                        let _e1389 = global_6.member[(_e1349 + 12u)];
                                                        let _e1392 = global_6.member[(_e1349 + 13u)];
                                                        let _e1395 = global_6.member[(_e1349 + 14u)];
                                                        let _e1398 = global_6.member[(_e1349 + 15u)];
                                                        let _e1402 = global_6.member[(_e1349 + 16u)];
                                                        let _e1405 = global_6.member[(_e1349 + 17u)];
                                                        let _e1408 = global_6.member[(_e1349 + 18u)];
                                                        let _e1411 = global_6.member[(_e1349 + 19u)];
                                                        let _e1414 = global_6.member[(_e1349 + 20u)];
                                                        let _e1417 = global_6.member[(_e1349 + 30u)];
                                                        let _e1418 = bitcast<u32>(_e1417);
                                                        let _e1421 = global_6.member[(_e1349 + 31u)];
                                                        let _e1422 = bitcast<u32>(_e1421);
                                                        let _e1425 = global_6.member[(_e1349 + 32u)];
                                                        let _e1428 = (_e1173.y < 0f);
                                                        if _e1428 {
                                                            phi_3555_ = type_55();
                                                            phi_3556_ = true;
                                                            phi_3557_ = false;
                                                        } else {
                                                            let _e1429 = (_e1422 == 4294967295u);
                                                            if _e1429 {
                                                                phi_3553_ = type_55();
                                                                phi_3554_ = false;
                                                            } else {
                                                                let _e1446 = (_e1174.x - _e1353);
                                                                let _e1447 = (_e1174.y - _e1356);
                                                                let _e1448 = (_e1174.z - _e1359);
                                                                let _e1456 = ((((((_e1174.x * _e1174.x) + (_e1174.y * _e1174.y)) + (_e1174.z * _e1174.z)) - (2f * (((_e1174.x * _e1353) + (_e1174.y * _e1356)) + (_e1174.z * _e1359)))) + _e1368) / (sqrt((((_e1446 * _e1446) + (_e1447 * _e1447)) + (_e1448 * _e1448))) + _e1362));
                                                                let _e1457 = (_e1349 + 21u);
                                                                let _e1460 = global_6.member[(_e1349 + 28u)];
                                                                if (_e1456 > _e1460) {
                                                                    phi_3543_ = type_56(0u, type_56().member_1);
                                                                } else {
                                                                    let _e1463 = global_6.member[_e1457];
                                                                    if (_e1456 != _e1456) {
                                                                        phi_20397_ = true;
                                                                    } else {
                                                                        phi_20397_ = (_e1463 >= _e1456);
                                                                    }
                                                                    let _e1467 = phi_20397_;
                                                                    let _e1468 = select(_e1456, _e1463, _e1467);
                                                                    phi_2183_ = 0u;
                                                                    loop {
                                                                        let _e1470 = phi_2183_;
                                                                        local_7 = _e1470;
                                                                        local_8 = _e1470;
                                                                        local_9 = _e1470;
                                                                        if (_e1470 < 8u) {
                                                                            let _e1474 = global_6.member[(_e1457 + _e1470)];
                                                                            let _e1475 = (_e1474 <= _e1468);
                                                                            if _e1475 {
                                                                                phi_2213_ = (_e1470 + 1u);
                                                                            } else {
                                                                                phi_2213_ = u32();
                                                                            }
                                                                            let _e1478 = phi_2213_;
                                                                            phi_2184_ = _e1478;
                                                                            phi_2214_ = _e1475;
                                                                        } else {
                                                                            phi_2184_ = u32();
                                                                            phi_2214_ = false;
                                                                        }
                                                                        let _e1480 = phi_2184_;
                                                                        let _e1482 = phi_2214_;
                                                                        phi_2183_ = _e1480;
                                                                        if !(_e1482) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e1485 = local_7;
                                                                    if (_e1485 < 1u) {
                                                                        phi_2232_ = 1u;
                                                                    } else {
                                                                        let _e1488 = local_8;
                                                                        let _e1491 = local_9;
                                                                        phi_2232_ = select(_e1491, 7u, (_e1488 > 7u));
                                                                    }
                                                                    let _e1494 = phi_2232_;
                                                                    let _e1495 = (_e1494 - 1u);
                                                                    let _e1498 = global_6.member[(_e1457 + _e1495)];
                                                                    let _e1501 = global_6.member[(_e1457 + _e1494)];
                                                                    let _e1504 = ((_e1468 - _e1498) / (_e1501 - _e1498));
                                                                    let _e1505 = select(_e1173.y, 0f, _e1428);
                                                                    let _e1511 = (sqrt((asin(select(_e1505, 1f, (_e1505 > 1f))) * 0.63661975f)) * 127f);
                                                                    let _e1515 = (atan2(_e1173.z, _e1173.x) * 0.15915494f);
                                                                    let _e1518 = ((_e1515 - floor(_e1515)) * 256f);
                                                                    let _e1519 = floor(_e1518);
                                                                    let _e1525 = (select(select(u32(_e1519), 0u, (_e1519 < 0f)), 4294967295u, (_e1519 > 4294967000f)) % 256u);
                                                                    let _e1526 = floor(_e1511);
                                                                    let _e1531 = select(select(u32(_e1526), 0u, (_e1526 < 0f)), 4294967295u, (_e1526 > 4294967000f));
                                                                    let _e1533 = select(_e1531, 126u, (126u < _e1531));
                                                                    let _e1535 = ((_e1525 + 1u) % 256u);
                                                                    let _e1537 = (_e1518 - _e1519);
                                                                    let _e1539 = (_e1511 - f32(_e1533));
                                                                    let _e1541 = (_e1422 + (_e1495 * 294912u));
                                                                    let _e1542 = (_e1533 * 256u);
                                                                    let _e1544 = (9u * (_e1542 + _e1525));
                                                                    let _e1545 = (_e1541 + _e1544);
                                                                    let _e1547 = (9u * (_e1542 + _e1535));
                                                                    let _e1548 = (_e1541 + _e1547);
                                                                    let _e1549 = ((_e1533 + 1u) * 256u);
                                                                    let _e1551 = (9u * (_e1549 + _e1525));
                                                                    let _e1552 = (_e1541 + _e1551);
                                                                    let _e1554 = (9u * (_e1549 + _e1535));
                                                                    let _e1555 = (_e1541 + _e1554);
                                                                    let _e1556 = (1f - _e1537);
                                                                    let _e1557 = (1f - _e1539);
                                                                    let _e1558 = (_e1556 * _e1557);
                                                                    let _e1559 = (_e1537 * _e1557);
                                                                    let _e1560 = (_e1556 * _e1539);
                                                                    let _e1561 = (_e1537 * _e1539);
                                                                    let _e1563 = global_6.member[_e1545];
                                                                    let _e1566 = global_6.member[(_e1545 + 1u)];
                                                                    let _e1569 = global_6.member[(_e1545 + 2u)];
                                                                    let _e1574 = global_6.member[_e1548];
                                                                    let _e1577 = global_6.member[(_e1548 + 1u)];
                                                                    let _e1580 = global_6.member[(_e1548 + 2u)];
                                                                    let _e1588 = global_6.member[_e1552];
                                                                    let _e1591 = global_6.member[(_e1552 + 1u)];
                                                                    let _e1594 = global_6.member[(_e1552 + 2u)];
                                                                    let _e1602 = global_6.member[_e1555];
                                                                    let _e1605 = global_6.member[(_e1555 + 1u)];
                                                                    let _e1608 = global_6.member[(_e1555 + 2u)];
                                                                    let _e1617 = global_6.member[(_e1545 + 3u)];
                                                                    let _e1620 = global_6.member[(_e1545 + 4u)];
                                                                    let _e1623 = global_6.member[(_e1545 + 5u)];
                                                                    let _e1629 = global_6.member[(_e1548 + 3u)];
                                                                    let _e1632 = global_6.member[(_e1548 + 4u)];
                                                                    let _e1635 = global_6.member[(_e1548 + 5u)];
                                                                    let _e1644 = global_6.member[(_e1552 + 3u)];
                                                                    let _e1647 = global_6.member[(_e1552 + 4u)];
                                                                    let _e1650 = global_6.member[(_e1552 + 5u)];
                                                                    let _e1659 = global_6.member[(_e1555 + 3u)];
                                                                    let _e1662 = global_6.member[(_e1555 + 4u)];
                                                                    let _e1665 = global_6.member[(_e1555 + 5u)];
                                                                    let _e1674 = global_6.member[(_e1545 + 6u)];
                                                                    let _e1677 = global_6.member[(_e1545 + 7u)];
                                                                    let _e1680 = global_6.member[(_e1545 + 8u)];
                                                                    let _e1686 = global_6.member[(_e1548 + 6u)];
                                                                    let _e1689 = global_6.member[(_e1548 + 7u)];
                                                                    let _e1692 = global_6.member[(_e1548 + 8u)];
                                                                    let _e1701 = global_6.member[(_e1552 + 6u)];
                                                                    let _e1704 = global_6.member[(_e1552 + 7u)];
                                                                    let _e1707 = global_6.member[(_e1552 + 8u)];
                                                                    let _e1716 = global_6.member[(_e1555 + 6u)];
                                                                    let _e1719 = global_6.member[(_e1555 + 7u)];
                                                                    let _e1722 = global_6.member[(_e1555 + 8u)];
                                                                    let _e1730 = (_e1422 + (_e1494 * 294912u));
                                                                    let _e1731 = (_e1730 + _e1544);
                                                                    let _e1732 = (_e1730 + _e1547);
                                                                    let _e1733 = (_e1730 + _e1551);
                                                                    let _e1734 = (_e1730 + _e1554);
                                                                    let _e1736 = global_6.member[_e1731];
                                                                    let _e1739 = global_6.member[(_e1731 + 1u)];
                                                                    let _e1742 = global_6.member[(_e1731 + 2u)];
                                                                    let _e1747 = global_6.member[_e1732];
                                                                    let _e1750 = global_6.member[(_e1732 + 1u)];
                                                                    let _e1753 = global_6.member[(_e1732 + 2u)];
                                                                    let _e1761 = global_6.member[_e1733];
                                                                    let _e1764 = global_6.member[(_e1733 + 1u)];
                                                                    let _e1767 = global_6.member[(_e1733 + 2u)];
                                                                    let _e1775 = global_6.member[_e1734];
                                                                    let _e1778 = global_6.member[(_e1734 + 1u)];
                                                                    let _e1781 = global_6.member[(_e1734 + 2u)];
                                                                    let _e1790 = global_6.member[(_e1731 + 3u)];
                                                                    let _e1793 = global_6.member[(_e1731 + 4u)];
                                                                    let _e1796 = global_6.member[(_e1731 + 5u)];
                                                                    let _e1802 = global_6.member[(_e1732 + 3u)];
                                                                    let _e1805 = global_6.member[(_e1732 + 4u)];
                                                                    let _e1808 = global_6.member[(_e1732 + 5u)];
                                                                    let _e1817 = global_6.member[(_e1733 + 3u)];
                                                                    let _e1820 = global_6.member[(_e1733 + 4u)];
                                                                    let _e1823 = global_6.member[(_e1733 + 5u)];
                                                                    let _e1832 = global_6.member[(_e1734 + 3u)];
                                                                    let _e1835 = global_6.member[(_e1734 + 4u)];
                                                                    let _e1838 = global_6.member[(_e1734 + 5u)];
                                                                    let _e1847 = global_6.member[(_e1731 + 6u)];
                                                                    let _e1850 = global_6.member[(_e1731 + 7u)];
                                                                    let _e1853 = global_6.member[(_e1731 + 8u)];
                                                                    let _e1859 = global_6.member[(_e1732 + 6u)];
                                                                    let _e1862 = global_6.member[(_e1732 + 7u)];
                                                                    let _e1865 = global_6.member[(_e1732 + 8u)];
                                                                    let _e1874 = global_6.member[(_e1733 + 6u)];
                                                                    let _e1877 = global_6.member[(_e1733 + 7u)];
                                                                    let _e1880 = global_6.member[(_e1733 + 8u)];
                                                                    let _e1889 = global_6.member[(_e1734 + 6u)];
                                                                    let _e1892 = global_6.member[(_e1734 + 7u)];
                                                                    let _e1895 = global_6.member[(_e1734 + 8u)];
                                                                    let _e1902 = (1f - _e1504);
                                                                    phi_3543_ = type_56(1u, type_55(vec3<f32>(((((((_e1563 * _e1558) + (_e1574 * _e1559)) + (_e1588 * _e1560)) + (_e1602 * _e1561)) * _e1902) + (((((_e1736 * _e1558) + (_e1747 * _e1559)) + (_e1761 * _e1560)) + (_e1775 * _e1561)) * _e1504)), ((((((_e1566 * _e1558) + (_e1577 * _e1559)) + (_e1591 * _e1560)) + (_e1605 * _e1561)) * _e1902) + (((((_e1739 * _e1558) + (_e1750 * _e1559)) + (_e1764 * _e1560)) + (_e1778 * _e1561)) * _e1504)), ((((((_e1569 * _e1558) + (_e1580 * _e1559)) + (_e1594 * _e1560)) + (_e1608 * _e1561)) * _e1902) + (((((_e1742 * _e1558) + (_e1753 * _e1559)) + (_e1767 * _e1560)) + (_e1781 * _e1561)) * _e1504))), vec3<f32>(((((((_e1617 * _e1558) + (_e1629 * _e1559)) + (_e1644 * _e1560)) + (_e1659 * _e1561)) * _e1902) + (((((_e1790 * _e1558) + (_e1802 * _e1559)) + (_e1817 * _e1560)) + (_e1832 * _e1561)) * _e1504)), ((((((_e1620 * _e1558) + (_e1632 * _e1559)) + (_e1647 * _e1560)) + (_e1662 * _e1561)) * _e1902) + (((((_e1793 * _e1558) + (_e1805 * _e1559)) + (_e1820 * _e1560)) + (_e1835 * _e1561)) * _e1504)), ((((((_e1623 * _e1558) + (_e1635 * _e1559)) + (_e1650 * _e1560)) + (_e1665 * _e1561)) * _e1902) + (((((_e1796 * _e1558) + (_e1808 * _e1559)) + (_e1823 * _e1560)) + (_e1838 * _e1561)) * _e1504))), vec3<f32>(((((((_e1674 * _e1558) + (_e1686 * _e1559)) + (_e1701 * _e1560)) + (_e1716 * _e1561)) * _e1902) + (((((_e1847 * _e1558) + (_e1859 * _e1559)) + (_e1874 * _e1560)) + (_e1889 * _e1561)) * _e1504)), ((((((_e1677 * _e1558) + (_e1689 * _e1559)) + (_e1704 * _e1560)) + (_e1719 * _e1561)) * _e1902) + (((((_e1850 * _e1558) + (_e1862 * _e1559)) + (_e1877 * _e1560)) + (_e1892 * _e1561)) * _e1504)), ((((((_e1680 * _e1558) + (_e1692 * _e1559)) + (_e1707 * _e1560)) + (_e1722 * _e1561)) * _e1902) + (((((_e1853 * _e1558) + (_e1865 * _e1559)) + (_e1880 * _e1560)) + (_e1895 * _e1561)) * _e1504)))));
                                                                }
                                                                let _e1939 = phi_3543_;
                                                                let _e1942 = (bitcast<i32>(_e1939.member) != 0i);
                                                                if _e1942 {
                                                                    phi_3551_ = _e1939.member_1;
                                                                } else {
                                                                    phi_3551_ = type_55();
                                                                }
                                                                let _e1945 = phi_3551_;
                                                                phi_3553_ = _e1945;
                                                                phi_3554_ = select(true, false, _e1942);
                                                            }
                                                            let _e1948 = phi_3553_;
                                                            let _e1950 = phi_3554_;
                                                            phi_3555_ = _e1948;
                                                            phi_3556_ = _e1429;
                                                            phi_3557_ = _e1950;
                                                        }
                                                        let _e1952 = phi_3555_;
                                                        let _e1954 = phi_3556_;
                                                        let _e1956 = phi_3557_;
                                                        if select(_e1956, true, _e1954) {
                                                            let _e1970 = ((((_e1174.x - _e1353) * _e1173.x) + ((_e1174.y - _e1356) * _e1173.y)) + ((_e1174.z - _e1359) * _e1173.z));
                                                            let _e1982 = ((((_e1174.x * _e1174.x) + (_e1174.y * _e1174.y)) + (_e1174.z * _e1174.z)) - (2f * (((_e1174.x * _e1353) + (_e1174.y * _e1356)) + (_e1174.z * _e1359))));
                                                            let _e1983 = (_e1982 + _e1371);
                                                            let _e1984 = (_e1970 * _e1970);
                                                            let _e1985 = (_e1984 - _e1983);
                                                            if (_e1985 < 0f) {
                                                                phi_20537_ = type_57(0u, type_57().member_1);
                                                                phi_20538_ = type_57();
                                                                phi_20539_ = true;
                                                            } else {
                                                                if (_e1970 < 0f) {
                                                                    phi_20509_ = -(sqrt(_e1985));
                                                                } else {
                                                                    phi_20509_ = sqrt(_e1985);
                                                                }
                                                                let _e1992 = phi_20509_;
                                                                let _e1994 = -((_e1970 + _e1992));
                                                                let _e1995 = (_e1994 == 0f);
                                                                if _e1995 {
                                                                    phi_20532_ = type_57(1u, type_52(0f, 0f));
                                                                    phi_20533_ = type_57();
                                                                } else {
                                                                    let _e1996 = (_e1983 / _e1994);
                                                                    if (_e1994 < _e1996) {
                                                                        phi_20521_ = type_52(_e1994, _e1996);
                                                                    } else {
                                                                        phi_20521_ = type_52(_e1996, _e1994);
                                                                    }
                                                                    let _e2001 = phi_20521_;
                                                                    phi_20532_ = type_57();
                                                                    phi_20533_ = type_57(1u, _e2001);
                                                                }
                                                                let _e2004 = phi_20532_;
                                                                let _e2006 = phi_20533_;
                                                                phi_20537_ = _e2004;
                                                                phi_20538_ = _e2006;
                                                                phi_20539_ = _e1995;
                                                            }
                                                            let _e2011 = phi_20537_;
                                                            let _e2013 = phi_20538_;
                                                            let _e2015 = phi_20539_;
                                                            if _e2015 {
                                                                phi_20543_ = _e2011;
                                                            } else {
                                                                phi_20543_ = _e2013;
                                                            }
                                                            let _e2017 = phi_20543_;
                                                            if (bitcast<i32>(_e2017.member) != 0i) {
                                                                phi_20430_ = type_57(0u, _e2017.member_1);
                                                            } else {
                                                                phi_20430_ = type_57(1u, type_57().member_1);
                                                            }
                                                            let _e2027 = phi_20430_;
                                                            if (bitcast<i32>(_e2027.member) != 0i) {
                                                                phi_20477_ = type_57(0u, type_57().member_1);
                                                            } else {
                                                                if (_e2027.member_1.member_1 <= 0f) {
                                                                    phi_20473_ = type_57(0u, type_57().member_1);
                                                                } else {
                                                                    if (_e2027.member_1.member != _e2027.member_1.member) {
                                                                        phi_20581_ = true;
                                                                    } else {
                                                                        phi_20581_ = (0f >= _e2027.member_1.member);
                                                                    }
                                                                    let _e2039 = phi_20581_;
                                                                    let _e2040 = select(_e2027.member_1.member, 0f, _e2039);
                                                                    let _e2041 = (_e1982 + _e1368);
                                                                    if (_e2041 > 0f) {
                                                                        let _e2043 = (_e1984 - _e2041);
                                                                        if (_e2043 < 0f) {
                                                                            phi_20689_ = type_57(0u, type_57().member_1);
                                                                            phi_20690_ = type_57();
                                                                            phi_20691_ = true;
                                                                        } else {
                                                                            if (_e1970 < 0f) {
                                                                                phi_20661_ = -(sqrt(_e2043));
                                                                            } else {
                                                                                phi_20661_ = sqrt(_e2043);
                                                                            }
                                                                            let _e2050 = phi_20661_;
                                                                            let _e2052 = -((_e1970 + _e2050));
                                                                            let _e2053 = (_e2052 == 0f);
                                                                            if _e2053 {
                                                                                phi_20684_ = type_57(1u, type_52(0f, 0f));
                                                                                phi_20685_ = type_57();
                                                                            } else {
                                                                                let _e2054 = (_e2041 / _e2052);
                                                                                if (_e2052 < _e2054) {
                                                                                    phi_20673_ = type_52(_e2052, _e2054);
                                                                                } else {
                                                                                    phi_20673_ = type_52(_e2054, _e2052);
                                                                                }
                                                                                let _e2059 = phi_20673_;
                                                                                phi_20684_ = type_57();
                                                                                phi_20685_ = type_57(1u, _e2059);
                                                                            }
                                                                            let _e2062 = phi_20684_;
                                                                            let _e2064 = phi_20685_;
                                                                            phi_20689_ = _e2062;
                                                                            phi_20690_ = _e2064;
                                                                            phi_20691_ = _e2053;
                                                                        }
                                                                        let _e2069 = phi_20689_;
                                                                        let _e2071 = phi_20690_;
                                                                        let _e2073 = phi_20691_;
                                                                        if _e2073 {
                                                                            phi_20695_ = _e2069;
                                                                        } else {
                                                                            phi_20695_ = _e2071;
                                                                        }
                                                                        let _e2075 = phi_20695_;
                                                                        if (bitcast<i32>(_e2075.member) != 0i) {
                                                                            phi_20629_ = select(_e2075.member_1.member_1, _e2075.member_1.member, (_e2075.member_1.member >= 0f));
                                                                        } else {
                                                                            phi_20629_ = -1f;
                                                                        }
                                                                        let _e2086 = phi_20629_;
                                                                        if (_e2086 >= 0f) {
                                                                            if (_e2027.member_1.member_1 != _e2027.member_1.member_1) {
                                                                                phi_20733_ = true;
                                                                            } else {
                                                                                phi_20733_ = (_e2086 <= _e2027.member_1.member_1);
                                                                            }
                                                                            let _e2091 = phi_20733_;
                                                                            phi_20458_ = select(_e2027.member_1.member_1, _e2086, _e2091);
                                                                        } else {
                                                                            phi_20458_ = _e2027.member_1.member_1;
                                                                        }
                                                                        let _e2094 = phi_20458_;
                                                                        phi_20460_ = _e2094;
                                                                    } else {
                                                                        phi_20460_ = _e2027.member_1.member_1;
                                                                    }
                                                                    let _e2096 = phi_20460_;
                                                                    if (_e2096 <= _e2040) {
                                                                        phi_20469_ = type_57(0u, type_57().member_1);
                                                                    } else {
                                                                        phi_20469_ = type_57(1u, type_52(_e2040, _e2096));
                                                                    }
                                                                    let _e2104 = phi_20469_;
                                                                    phi_20473_ = _e2104;
                                                                }
                                                                let _e2109 = phi_20473_;
                                                                phi_20477_ = _e2109;
                                                            }
                                                            let _e2114 = phi_20477_;
                                                            if (bitcast<i32>(_e2114.member) != 0i) {
                                                                phi_3575_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_3578_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_3580_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_3582_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_3584_ = 0u;
                                                                loop {
                                                                    let _e2123 = phi_3575_;
                                                                    let _e2125 = phi_3578_;
                                                                    let _e2127 = phi_3580_;
                                                                    let _e2129 = phi_3582_;
                                                                    let _e2131 = phi_3584_;
                                                                    local_11 = _e2129;
                                                                    local_12 = _e2127;
                                                                    local_13 = _e2125;
                                                                    let _e2132 = (_e2131 < 32u);
                                                                    if _e2132 {
                                                                        let _e2133 = -(_e1970);
                                                                        if (_e2114.member_1.member <= _e2114.member_1.member_1) {
                                                                        } else {
                                                                            phi_30281_ = true;
                                                                            break;
                                                                        }
                                                                        let _e2136 = select(_e2133, _e2114.member_1.member, (_e2133 < _e2114.member_1.member));
                                                                        let _e2138 = select(_e2136, _e2114.member_1.member_1, (_e2136 > _e2114.member_1.member_1));
                                                                        let _e2139 = (_e2138 - _e2114.member_1.member);
                                                                        let _e2140 = (_e2114.member_1.member_1 - _e2138);
                                                                        let _e2141 = (_e2114.member_1.member_1 - _e2114.member_1.member);
                                                                        let _e2142 = (0.000000001f * _e2141);
                                                                        if (_e2139 <= _e2142) {
                                                                            let _e2217 = (f32(_e2131) * 0.03125f);
                                                                            let _e2220 = (f32((_e2131 + 1u)) * 0.03125f);
                                                                            let _e2222 = ((_e2217 + _e2220) * 0.5f);
                                                                            phi_3691_ = type_52((_e2114.member_1.member + ((_e2141 * _e2222) * _e2222)), (_e2141 * ((_e2220 * _e2220) - (_e2217 * _e2217))));
                                                                        } else {
                                                                            if (_e2140 <= _e2142) {
                                                                                let _e2200 = (f32((31u - _e2131)) * 0.03125f);
                                                                                let _e2203 = (f32((32u - _e2131)) * 0.03125f);
                                                                                let _e2205 = ((_e2200 + _e2203) * 0.5f);
                                                                                phi_3690_ = type_52((_e2114.member_1.member_1 - ((_e2141 * _e2205) * _e2205)), (_e2141 * ((_e2203 * _e2203) - (_e2200 * _e2200))));
                                                                            } else {
                                                                                let _e2148 = round(((32f * _e2139) / (_e2139 + _e2140)));
                                                                                let _e2153 = select(select(u32(_e2148), 0u, (_e2148 < 0f)), 4294967295u, (_e2148 > 4294967000f));
                                                                                if (_e2153 < 1u) {
                                                                                    phi_3674_ = 1u;
                                                                                } else {
                                                                                    phi_3674_ = select(_e2153, 31u, (_e2153 > 31u));
                                                                                }
                                                                                let _e2158 = phi_3674_;
                                                                                if (_e2131 < _e2158) {
                                                                                    let _e2179 = ((_e2158 - 1u) - _e2131);
                                                                                    let _e2181 = f32(_e2158);
                                                                                    let _e2182 = (f32(_e2179) / _e2181);
                                                                                    let _e2185 = (f32((_e2179 + 1u)) / _e2181);
                                                                                    let _e2187 = ((_e2182 + _e2185) * 0.5f);
                                                                                    phi_3689_ = type_52((_e2138 - ((_e2139 * _e2187) * _e2187)), (_e2139 * ((_e2185 * _e2185) - (_e2182 * _e2182))));
                                                                                } else {
                                                                                    let _e2161 = (_e2131 - _e2158);
                                                                                    let _e2163 = f32((32u - _e2158));
                                                                                    let _e2164 = (f32(_e2161) / _e2163);
                                                                                    let _e2167 = (f32((_e2161 + 1u)) / _e2163);
                                                                                    let _e2169 = ((_e2164 + _e2167) * 0.5f);
                                                                                    phi_3689_ = type_52((_e2138 + ((_e2140 * _e2169) * _e2169)), (_e2140 * ((_e2167 * _e2167) - (_e2164 * _e2164))));
                                                                                }
                                                                                let _e2197 = phi_3689_;
                                                                                phi_3690_ = _e2197;
                                                                            }
                                                                            let _e2215 = phi_3690_;
                                                                            phi_3691_ = _e2215;
                                                                        }
                                                                        let _e2232 = phi_3691_;
                                                                        let _e2235 = (_e1173.x * _e2232.member);
                                                                        let _e2236 = (_e1173.y * _e2232.member);
                                                                        let _e2237 = (_e1173.z * _e2232.member);
                                                                        let _e2242 = (_e1174 + vec3<f32>(_e2235, _e2236, _e2237));
                                                                        let _e2257 = ((((_e2242.x * _e2242.x) + (_e2242.y * _e2242.y)) + (_e2242.z * _e2242.z)) - (2f * (((_e2242.x * _e1353) + (_e2242.y * _e1356)) + (_e2242.z * _e1359))));
                                                                        let _e2258 = (_e2257 + _e1368);
                                                                        let _e2259 = (_e2242.x - _e1353);
                                                                        let _e2260 = (_e2242.y - _e1356);
                                                                        let _e2261 = (_e2242.z - _e1359);
                                                                        let _e2269 = (_e2258 / (sqrt((((_e2259 * _e2259) + (_e2260 * _e2260)) + (_e2261 * _e2261))) + _e1362));
                                                                        let _e2273 = (1f - (abs((_e2269 - 25000f)) * 0.00006666667f));
                                                                        if (_e2273 != _e2273) {
                                                                            phi_20932_ = true;
                                                                        } else {
                                                                            phi_20932_ = (0f >= _e2273);
                                                                        }
                                                                        let _e2277 = phi_20932_;
                                                                        let _e2279 = -(_e2269);
                                                                        let _e2282 = (exp((_e2279 / _e1383)) * _e2232.member_1);
                                                                        let _e2285 = (exp((_e2279 / _e1386)) * _e2232.member_1);
                                                                        let _e2287 = (_e2282 * 0.5f);
                                                                        let _e2288 = (_e2285 * 0.5f);
                                                                        let _e2289 = ((select(_e2273, 0f, _e2277) * _e2232.member_1) * 0.5f);
                                                                        let _e2291 = (_e2123.x + _e2287);
                                                                        let _e2293 = (_e2123.y + _e2288);
                                                                        let _e2295 = (_e2123.z + _e2289);
                                                                        let _e2300 = (_e1402 * 1.1f);
                                                                        let _e2301 = (_e2300 * _e2293);
                                                                        let _e2320 = (_e1402 * _e2285);
                                                                        if (_e1418 == 4294967295u) {
                                                                            phi_4026_ = vec3<f32>(0f, 0f, 0f);
                                                                        } else {
                                                                            let _e2328 = ((_e1174.x + _e2235) - _e1353);
                                                                            let _e2329 = ((_e1174.y + _e2236) - _e1356);
                                                                            let _e2330 = ((_e1174.z + _e2237) - _e1359);
                                                                            let _e2337 = (1f / sqrt((((_e2328 * _e2328) + (_e2329 * _e2329)) + (_e2330 * _e2330))));
                                                                            let _e2345 = ((((_e2328 * _e2337) * _e1374) + ((_e2329 * _e2337) * _e1377)) + ((_e2330 * _e2337) * _e1380));
                                                                            if (_e2269 != _e2269) {
                                                                                phi_21059_ = true;
                                                                            } else {
                                                                                phi_21059_ = (0f >= _e2269);
                                                                            }
                                                                            let _e2349 = phi_21059_;
                                                                            let _e2353 = sqrt((select(_e2269, 0f, _e2349) / (_e1365 - _e1362)));
                                                                            if (_e2353 != _e2353) {
                                                                                phi_21074_ = true;
                                                                            } else {
                                                                                phi_21074_ = (1f <= _e2353);
                                                                            }
                                                                            let _e2357 = phi_21074_;
                                                                            let _e2359 = (select(_e2353, 1f, _e2357) * 31f);
                                                                            let _e2361 = select(_e2345, -1f, (_e2345 < -1f));
                                                                            let _e2365 = ((select(_e2361, 1f, (_e2361 > 1f)) + 1f) * 15.5f);
                                                                            let _e2370 = select(select(u32(_e2359), 0u, (_e2359 < 0f)), 4294967295u, (_e2359 > 4294967000f));
                                                                            let _e2372 = select(_e2370, 30u, (30u < _e2370));
                                                                            let _e2377 = select(select(u32(_e2365), 0u, (_e2365 < 0f)), 4294967295u, (_e2365 > 4294967000f));
                                                                            let _e2379 = select(_e2377, 30u, (30u < _e2377));
                                                                            let _e2381 = (_e2359 - f32(_e2372));
                                                                            let _e2383 = (_e2365 - f32(_e2379));
                                                                            let _e2387 = (_e1418 + (3u * ((_e2372 * 32u) + _e2379)));
                                                                            let _e2392 = (_e1418 + (3u * (((_e2372 + 1u) * 32u) + _e2379)));
                                                                            let _e2394 = global_6.member[_e2387];
                                                                            let _e2397 = global_6.member[(_e2387 + 1u)];
                                                                            let _e2400 = global_6.member[(_e2387 + 2u)];
                                                                            let _e2401 = (1f - _e2383);
                                                                            let _e2407 = global_6.member[(_e2387 + 3u)];
                                                                            let _e2410 = global_6.member[(_e2387 + 4u)];
                                                                            let _e2413 = global_6.member[(_e2387 + 5u)];
                                                                            let _e2420 = (1f - _e2381);
                                                                            let _e2425 = global_6.member[_e2392];
                                                                            let _e2428 = global_6.member[(_e2392 + 1u)];
                                                                            let _e2431 = global_6.member[(_e2392 + 2u)];
                                                                            let _e2437 = global_6.member[(_e2392 + 3u)];
                                                                            let _e2440 = global_6.member[(_e2392 + 4u)];
                                                                            let _e2443 = global_6.member[(_e2392 + 5u)];
                                                                            phi_4026_ = vec3<f32>(((((_e2394 * _e2401) + (_e2407 * _e2383)) * _e2420) + (((_e2425 * _e2401) + (_e2437 * _e2383)) * _e2381)), ((((_e2397 * _e2401) + (_e2410 * _e2383)) * _e2420) + (((_e2428 * _e2401) + (_e2440 * _e2383)) * _e2381)), ((((_e2400 * _e2401) + (_e2413 * _e2383)) * _e2420) + (((_e2431 * _e2401) + (_e2443 * _e2383)) * _e2381)));
                                                                        }
                                                                        let _e2458 = phi_4026_;
                                                                        let _e2476 = (((_e2259 * _e1374) + (_e2260 * _e1377)) + (_e2261 * _e1380));
                                                                        let _e2477 = (_e2476 * _e2476);
                                                                        let _e2478 = (_e2477 - _e2258);
                                                                        if (_e2478 < 0f) {
                                                                            phi_21224_ = type_57(0u, type_57().member_1);
                                                                            phi_21225_ = type_57();
                                                                            phi_21226_ = true;
                                                                        } else {
                                                                            if (_e2476 < 0f) {
                                                                                phi_21196_ = -(sqrt(_e2478));
                                                                            } else {
                                                                                phi_21196_ = sqrt(_e2478);
                                                                            }
                                                                            let _e2485 = phi_21196_;
                                                                            let _e2487 = -((_e2476 + _e2485));
                                                                            let _e2488 = (_e2487 == 0f);
                                                                            if _e2488 {
                                                                                phi_21219_ = type_57(1u, type_52(0f, 0f));
                                                                                phi_21220_ = type_57();
                                                                            } else {
                                                                                let _e2489 = (_e2258 / _e2487);
                                                                                if (_e2487 < _e2489) {
                                                                                    phi_21208_ = type_52(_e2487, _e2489);
                                                                                } else {
                                                                                    phi_21208_ = type_52(_e2489, _e2487);
                                                                                }
                                                                                let _e2494 = phi_21208_;
                                                                                phi_21219_ = type_57();
                                                                                phi_21220_ = type_57(1u, _e2494);
                                                                            }
                                                                            let _e2497 = phi_21219_;
                                                                            let _e2499 = phi_21220_;
                                                                            phi_21224_ = _e2497;
                                                                            phi_21225_ = _e2499;
                                                                            phi_21226_ = _e2488;
                                                                        }
                                                                        let _e2504 = phi_21224_;
                                                                        let _e2506 = phi_21225_;
                                                                        let _e2508 = phi_21226_;
                                                                        if _e2508 {
                                                                            phi_21230_ = _e2504;
                                                                        } else {
                                                                            phi_21230_ = _e2506;
                                                                        }
                                                                        let _e2510 = phi_21230_;
                                                                        if (bitcast<i32>(_e2510.member) != 0i) {
                                                                            phi_21164_ = select(_e2510.member_1.member_1, _e2510.member_1.member, (_e2510.member_1.member >= 0f));
                                                                        } else {
                                                                            phi_21164_ = -1f;
                                                                        }
                                                                        let _e2521 = phi_21164_;
                                                                        if (_e2521 >= 0f) {
                                                                            phi_21147_ = type_58(0u, type_58().member_1);
                                                                        } else {
                                                                            let _e2523 = (_e2257 + _e1371);
                                                                            let _e2524 = (_e2477 - _e2523);
                                                                            if (_e2524 < 0f) {
                                                                                phi_21334_ = type_57(0u, type_57().member_1);
                                                                                phi_21335_ = type_57();
                                                                                phi_21336_ = true;
                                                                            } else {
                                                                                if (_e2476 < 0f) {
                                                                                    phi_21306_ = -(sqrt(_e2524));
                                                                                } else {
                                                                                    phi_21306_ = sqrt(_e2524);
                                                                                }
                                                                                let _e2531 = phi_21306_;
                                                                                let _e2533 = -((_e2476 + _e2531));
                                                                                let _e2534 = (_e2533 == 0f);
                                                                                if _e2534 {
                                                                                    phi_21329_ = type_57(1u, type_52(0f, 0f));
                                                                                    phi_21330_ = type_57();
                                                                                } else {
                                                                                    let _e2535 = (_e2523 / _e2533);
                                                                                    if (_e2533 < _e2535) {
                                                                                        phi_21318_ = type_52(_e2533, _e2535);
                                                                                    } else {
                                                                                        phi_21318_ = type_52(_e2535, _e2533);
                                                                                    }
                                                                                    let _e2540 = phi_21318_;
                                                                                    phi_21329_ = type_57();
                                                                                    phi_21330_ = type_57(1u, _e2540);
                                                                                }
                                                                                let _e2543 = phi_21329_;
                                                                                let _e2545 = phi_21330_;
                                                                                phi_21334_ = _e2543;
                                                                                phi_21335_ = _e2545;
                                                                                phi_21336_ = _e2534;
                                                                            }
                                                                            let _e2550 = phi_21334_;
                                                                            let _e2552 = phi_21335_;
                                                                            let _e2554 = phi_21336_;
                                                                            if _e2554 {
                                                                                phi_21340_ = _e2550;
                                                                            } else {
                                                                                phi_21340_ = _e2552;
                                                                            }
                                                                            let _e2556 = phi_21340_;
                                                                            if (bitcast<i32>(_e2556.member) != 0i) {
                                                                                phi_21274_ = select(_e2556.member_1.member_1, _e2556.member_1.member, (_e2556.member_1.member >= 0f));
                                                                            } else {
                                                                                phi_21274_ = -1f;
                                                                            }
                                                                            let _e2567 = phi_21274_;
                                                                            if (_e2567 < 0f) {
                                                                                phi_21143_ = type_58(0u, type_58().member_1);
                                                                            } else {
                                                                                phi_21101_ = vec3<f32>(0f, 0f, 0f);
                                                                                phi_21102_ = 0u;
                                                                                loop {
                                                                                    let _e2570 = phi_21101_;
                                                                                    let _e2572 = phi_21102_;
                                                                                    local_10 = _e2570;
                                                                                    let _e2573 = (_e2572 < 16u);
                                                                                    if _e2573 {
                                                                                        let _e2575 = (f32(_e2572) * 0.0625f);
                                                                                        let _e2576 = (_e2572 + 1u);
                                                                                        let _e2578 = (f32(_e2576) * 0.0625f);
                                                                                        let _e2580 = ((_e2575 + _e2578) * 0.5f);
                                                                                        let _e2582 = ((_e2567 * _e2580) * _e2580);
                                                                                        let _e2586 = (_e2567 * ((_e2578 * _e2578) - (_e2575 * _e2575)));
                                                                                        let _e2591 = (_e2242 + vec3<f32>((_e1374 * _e2582), (_e1377 * _e2582), (_e1380 * _e2582)));
                                                                                        let _e2608 = (_e2591.x - _e1353);
                                                                                        let _e2609 = (_e2591.y - _e1356);
                                                                                        let _e2610 = (_e2591.z - _e1359);
                                                                                        let _e2618 = ((((((_e2591.x * _e2591.x) + (_e2591.y * _e2591.y)) + (_e2591.z * _e2591.z)) - (2f * (((_e2591.x * _e1353) + (_e2591.y * _e1356)) + (_e2591.z * _e1359)))) + _e1368) / (sqrt((((_e2608 * _e2608) + (_e2609 * _e2609)) + (_e2610 * _e2610))) + _e1362));
                                                                                        let _e2622 = (1f - (abs((_e2618 - 25000f)) * 0.00006666667f));
                                                                                        if (_e2622 != _e2622) {
                                                                                            phi_21472_ = true;
                                                                                        } else {
                                                                                            phi_21472_ = (0f >= _e2622);
                                                                                        }
                                                                                        let _e2626 = phi_21472_;
                                                                                        let _e2628 = -(_e2618);
                                                                                        phi_21135_ = (_e2570 + vec3<f32>((exp((_e2628 / _e1383)) * _e2586), (exp((_e2628 / _e1386)) * _e2586), (select(_e2622, 0f, _e2626) * _e2586)));
                                                                                        phi_21136_ = _e2576;
                                                                                    } else {
                                                                                        phi_21135_ = vec3<f32>();
                                                                                        phi_21136_ = u32();
                                                                                    }
                                                                                    let _e2639 = phi_21135_;
                                                                                    let _e2641 = phi_21136_;
                                                                                    phi_21101_ = _e2639;
                                                                                    phi_21102_ = _e2641;
                                                                                    if !(_e2573) {
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                let _e2644 = local_10;
                                                                                phi_21143_ = type_58(1u, _e2644);
                                                                            }
                                                                            let _e2650 = phi_21143_;
                                                                            phi_21147_ = _e2650;
                                                                        }
                                                                        let _e2655 = phi_21147_;
                                                                        if (bitcast<i32>(_e2655.member) != 0i) {
                                                                            let _e2660 = (vec3<f32>(_e2291, _e2293, _e2295) + _e2655.member_1);
                                                                            let _e2666 = (_e2300 * _e2660.y);
                                                                            let _e2678 = exp(-((((_e1392 * _e2660.x) + _e2666) + (_e1408 * _e2660.z))));
                                                                            let _e2680 = exp(-((((_e1395 * _e2660.x) + _e2666) + (_e1411 * _e2660.z))));
                                                                            let _e2682 = exp(-((((_e1398 * _e2660.x) + _e2666) + (_e1414 * _e2660.z))));
                                                                            phi_4085_ = vec3<f32>((_e2127.x + (_e2678 * _e2285)), (_e2127.y + (_e2680 * _e2285)), (_e2127.z + (_e2682 * _e2285)));
                                                                            phi_4086_ = vec3<f32>((_e2129.x + (_e2678 * _e2282)), (_e2129.y + (_e2680 * _e2282)), (_e2129.z + (_e2682 * _e2282)));
                                                                        } else {
                                                                            phi_4085_ = _e2127;
                                                                            phi_4086_ = _e2129;
                                                                        }
                                                                        let _e2704 = phi_4085_;
                                                                        let _e2706 = phi_4086_;
                                                                        phi_3576_ = vec3<f32>((_e2291 + _e2287), (_e2293 + _e2288), (_e2295 + _e2289));
                                                                        phi_3579_ = vec3<f32>((_e2125.x + ((exp(-((((_e1392 * _e2291) + _e2301) + (_e1408 * _e2295)))) * ((_e1392 * _e2282) + _e2320)) * _e2458.x)), (_e2125.y + ((exp(-((((_e1395 * _e2291) + _e2301) + (_e1411 * _e2295)))) * ((_e1395 * _e2282) + _e2320)) * _e2458.y)), (_e2125.z + ((exp(-((((_e1398 * _e2291) + _e2301) + (_e1414 * _e2295)))) * ((_e1398 * _e2282) + _e2320)) * _e2458.z)));
                                                                        phi_3581_ = _e2704;
                                                                        phi_3583_ = _e2706;
                                                                        phi_3585_ = (_e2131 + 1u);
                                                                    } else {
                                                                        phi_3576_ = vec3<f32>();
                                                                        phi_3579_ = vec3<f32>();
                                                                        phi_3581_ = vec3<f32>();
                                                                        phi_3583_ = vec3<f32>();
                                                                        phi_3585_ = u32();
                                                                    }
                                                                    let _e2713 = phi_3576_;
                                                                    let _e2715 = phi_3579_;
                                                                    let _e2717 = phi_3581_;
                                                                    let _e2719 = phi_3583_;
                                                                    let _e2721 = phi_3585_;
                                                                    phi_3575_ = _e2713;
                                                                    phi_3578_ = _e2715;
                                                                    phi_3580_ = _e2717;
                                                                    phi_3582_ = _e2719;
                                                                    phi_3584_ = _e2721;
                                                                    phi_30281_ = _e1343;
                                                                    if !(_e2132) {
                                                                        break;
                                                                    }
                                                                }
                                                                let _e2724 = phi_30281_;
                                                                phi_30399_ = _e2724;
                                                                if _e2724 {
                                                                    break;
                                                                }
                                                                let _e2726 = local_11;
                                                                let _e2729 = local_12;
                                                                let _e2732 = local_13;
                                                                phi_30403_ = _e2724;
                                                                phi_4106_ = type_55((_e2726 * vec3<f32>(_e1392, _e1395, _e1398)), (_e2729 * _e1402), _e2732);
                                                            } else {
                                                                phi_30403_ = _e1343;
                                                                phi_4106_ = type_55(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                                            }
                                                            let _e2735 = phi_30403_;
                                                            let _e2737 = phi_4106_;
                                                            phi_30402_ = _e2735;
                                                            phi_4107_ = _e2737;
                                                        } else {
                                                            phi_30402_ = _e1343;
                                                            phi_4107_ = _e1952;
                                                        }
                                                        let _e2739 = phi_30402_;
                                                        let _e2741 = phi_4107_;
                                                        let _e2751 = (((_e1173.x * _e1374) + (_e1173.y * _e1377)) + (_e1173.z * _e1380));
                                                        let _e2753 = (1f + (_e2751 * _e2751));
                                                        let _e2754 = (0.059683103f * _e2753);
                                                        let _e2761 = (_e1405 * _e1405);
                                                        let _e2772 = ((0.119366206f * ((1f - _e2761) * _e2753)) / ((2f + _e2761) * pow(((1f + _e2761) - ((2f * _e1405) * _e2751)), 1.5f)));
                                                        phi_30401_ = _e2739;
                                                        phi_4130_ = vec3<f32>((_e1345.x + (((((_e2741.member.x * _e2754) + (_e2741.member_1.x * _e2772)) + _e2741.member_2.x) * _e1389) * 0.38317f)), (_e1345.y + (((((_e2741.member.y * _e2754) + (_e2741.member_1.y * _e2772)) + _e2741.member_2.y) * _e1389) * 0.38317f)), (_e1345.z + (((((_e2741.member.z * _e2754) + (_e2741.member_1.z * _e2772)) + _e2741.member_2.z) * _e1389) * 0.38317f)));
                                                        phi_4131_ = (_e1347 + 1i);
                                                        phi_4132_ = bitcast<u32>(_e1425);
                                                    } else {
                                                        phi_30401_ = _e1343;
                                                        phi_4130_ = vec3<f32>();
                                                        phi_4131_ = i32();
                                                        phi_4132_ = u32();
                                                    }
                                                    let _e2803 = phi_30401_;
                                                    let _e2805 = phi_4130_;
                                                    let _e2807 = phi_4131_;
                                                    let _e2809 = phi_4132_;
                                                    phi_30400_ = _e2803;
                                                    phi_1756_ = _e2805;
                                                    phi_1759_ = _e2807;
                                                    phi_1761_ = _e2809;
                                                    phi_4133_ = _e1351;
                                                } else {
                                                    phi_30400_ = _e1343;
                                                    phi_1756_ = vec3<f32>();
                                                    phi_1759_ = i32();
                                                    phi_1761_ = u32();
                                                    phi_4133_ = false;
                                                }
                                                let _e2811 = phi_30400_;
                                                let _e2813 = phi_1756_;
                                                let _e2815 = phi_1759_;
                                                let _e2817 = phi_1761_;
                                                let _e2819 = phi_4133_;
                                                phi_30313_ = _e2811;
                                                phi_1755_ = _e2813;
                                                phi_1758_ = _e2815;
                                                phi_1760_ = _e2817;
                                                phi_30399_ = _e2811;
                                                if !(_e2819) {
                                                    break;
                                                }
                                            }
                                            let _e2822 = phi_30399_;
                                            phi_30727_ = _e2822;
                                            if _e2822 {
                                                break;
                                            }
                                            if _e1340 {
                                                let _e2825 = global_6.member[(_e1189 + 10u)];
                                                let _e2828 = global_6.member[(_e1189 + 21u)];
                                                let _e2829 = bitcast<u32>(_e2828);
                                                let _e2832 = global_6.member[(_e1189 + 22u)];
                                                let _e2833 = bitcast<u32>(_e2832);
                                                let _e2836 = global_6.member[(_e1189 + 23u)];
                                                let _e2840 = global_6.member[(_e1189 + 24u)];
                                                let _e2844 = global_6.member[(_e1189 + 25u)];
                                                let _e2848 = global_6.member[(_e1189 + 26u)];
                                                let _e2851 = global_6.member[_e1189];
                                                let _e2854 = global_6.member[(_e1189 + 1u)];
                                                let _e2857 = global_6.member[(_e1189 + 2u)];
                                                let _e2860 = global_6.member[(_e1189 + 3u)];
                                                let _e2863 = global_6.member[(_e1189 + 4u)];
                                                let _e2866 = global_6.member[(_e1189 + 5u)];
                                                let _e2869 = global_6.member[(_e1189 + 6u)];
                                                let _e2872 = global_6.member[(_e1189 + 7u)];
                                                let _e2875 = global_6.member[(_e1189 + 8u)];
                                                let _e2883 = (((_e1173.x * _e2860) + (_e1173.y * _e2863)) + (_e1173.z * _e2866));
                                                let _e2885 = select(_e2883, -1f, (_e2883 < -1f));
                                                let _e2888 = asin(select(_e2885, 1f, (_e2885 > 1f)));
                                                let _e2899 = atan2((((_e1173.x * _e2869) + (_e1173.y * _e2872)) + (_e1173.z * _e2875)), (((_e1173.x * _e2851) + (_e1173.y * _e2854)) + (_e1173.z * _e2857)));
                                                if (_e2829 != 4294967295u) {
                                                    let _e2902 = global_6.member[_e2829];
                                                    let _e2905 = global_6.member[(_e2829 + 1u)];
                                                    let _e2906 = bitcast<u32>(_e2905);
                                                    let _e2909 = global_6.member[(_e2829 + 4u)];
                                                    phi_4476_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_4479_ = 0u;
                                                    loop {
                                                        let _e2915 = phi_4476_;
                                                        let _e2917 = phi_4479_;
                                                        local_14 = _e2915;
                                                        let _e2918 = (_e2917 < 6u);
                                                        if _e2918 {
                                                            let _e2919 = (_e2917 / 2u);
                                                            if (_e2919 == 0u) {
                                                                phi_21600_ = _e1173.x;
                                                            } else {
                                                                phi_21600_ = select(_e1173.z, _e1173.y, (_e2919 == 1u));
                                                            }
                                                            let _e2927 = phi_21600_;
                                                            let _e2928 = (_e2927 * select(-1f, 1f, ((_e2917 % 2u) == 0u)));
                                                            if (_e2928 > 0f) {
                                                                let _e2932 = ((_e2919 + 1u) % 3u);
                                                                if (_e2932 == 0u) {
                                                                    phi_21615_ = _e1173.x;
                                                                } else {
                                                                    phi_21615_ = select(_e1173.z, _e1173.y, (_e2932 == 1u));
                                                                }
                                                                let _e2937 = phi_21615_;
                                                                let _e2938 = (_e2937 / _e2928);
                                                                let _e2940 = ((_e2919 + 2u) % 3u);
                                                                if (_e2940 == 0u) {
                                                                    phi_21630_ = _e1173.x;
                                                                } else {
                                                                    phi_21630_ = select(_e1173.z, _e1173.y, (_e2940 == 1u));
                                                                }
                                                                let _e2945 = phi_21630_;
                                                                let _e2946 = (_e2945 / _e2928);
                                                                let _e2948 = (1f + (_e2902 * 12f));
                                                                if (abs(_e2938) <= _e2948) {
                                                                    if (abs(_e2946) <= _e2948) {
                                                                        let _e2954 = f32(_e2906);
                                                                        let _e2956 = floor((((_e2938 + 1f) * 0.5f) * _e2954));
                                                                        let _e2963 = select(0i, select(select(i32(_e2956), i32(-2147483648), (_e2956 < -2147483600f)), 2147483647i, (_e2956 > 2147483500f)), (_e2956 == _e2956));
                                                                        let _e2965 = (bitcast<i32>(_e2906) - 1i);
                                                                        let _e2966 = (0i <= _e2965);
                                                                        if _e2966 {
                                                                        } else {
                                                                            phi_30404_ = true;
                                                                            break;
                                                                        }
                                                                        if (_e2963 < 0i) {
                                                                            phi_4550_ = 0i;
                                                                        } else {
                                                                            phi_4550_ = select(_e2963, _e2965, (_e2963 > _e2965));
                                                                        }
                                                                        let _e2971 = phi_4550_;
                                                                        let _e2975 = floor((((_e2946 + 1f) * 0.5f) * _e2954));
                                                                        let _e2982 = select(0i, select(select(i32(_e2975), i32(-2147483648), (_e2975 < -2147483600f)), 2147483647i, (_e2975 > 2147483500f)), (_e2975 == _e2975));
                                                                        if _e2966 {
                                                                        } else {
                                                                            phi_30404_ = true;
                                                                            break;
                                                                        }
                                                                        if (_e2982 < 0i) {
                                                                            phi_4580_ = 0i;
                                                                        } else {
                                                                            phi_4580_ = select(_e2982, _e2965, (_e2982 > _e2965));
                                                                        }
                                                                        let _e2987 = phi_4580_;
                                                                        let _e2988 = (_e2971 - 1i);
                                                                        phi_4588_ = _e2915;
                                                                        phi_4591_ = select(0i, _e2988, (0i < _e2988));
                                                                        loop {
                                                                            let _e2992 = phi_4588_;
                                                                            let _e2994 = phi_4591_;
                                                                            let _e2995 = (_e2971 + 1i);
                                                                            local_108 = _e2992;
                                                                            let _e2998 = (_e2994 <= select(_e2995, _e2965, (_e2965 < _e2995)));
                                                                            if _e2998 {
                                                                                let _e2999 = (_e2987 - 1i);
                                                                                phi_4614_ = _e2992;
                                                                                phi_4617_ = select(0i, _e2999, (0i < _e2999));
                                                                                loop {
                                                                                    let _e3003 = phi_4614_;
                                                                                    let _e3005 = phi_4617_;
                                                                                    let _e3006 = (_e2987 + 1i);
                                                                                    local_107 = _e3003;
                                                                                    let _e3009 = (_e3005 <= select(_e3006, _e2965, (_e2965 < _e3006)));
                                                                                    if _e3009 {
                                                                                        let _e3018 = global_6.member[(bitcast<u32>(_e2909) + ((((_e2917 * _e2906) + bitcast<u32>(_e2994)) * _e2906) + bitcast<u32>(_e3005)))];
                                                                                        phi_4615_ = vec3<f32>((_e3003.x + _e3018), vec3<f32>().y, vec3<f32>().z);
                                                                                        phi_4618_ = (_e3005 + 1i);
                                                                                    } else {
                                                                                        phi_4615_ = vec3<f32>();
                                                                                        phi_4618_ = i32();
                                                                                    }
                                                                                    let _e3027 = phi_4615_;
                                                                                    let _e3029 = phi_4618_;
                                                                                    phi_4614_ = _e3027;
                                                                                    phi_4617_ = _e3029;
                                                                                    if !(_e3009) {
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                let _e14039 = local_107;
                                                                                phi_4589_ = _e14039;
                                                                                phi_4592_ = (_e2994 + 1i);
                                                                            } else {
                                                                                phi_4589_ = vec3<f32>();
                                                                                phi_4592_ = i32();
                                                                            }
                                                                            let _e3033 = phi_4589_;
                                                                            let _e3035 = phi_4592_;
                                                                            phi_4588_ = _e3033;
                                                                            phi_4591_ = _e3035;
                                                                            if !(_e2998) {
                                                                                break;
                                                                            }
                                                                        }
                                                                        let _e14043 = local_108;
                                                                        phi_4819_ = _e14043;
                                                                    } else {
                                                                        phi_4819_ = _e2915;
                                                                    }
                                                                    let _e3038 = phi_4819_;
                                                                    phi_4820_ = _e3038;
                                                                } else {
                                                                    phi_4820_ = _e2915;
                                                                }
                                                                let _e3040 = phi_4820_;
                                                                phi_4821_ = _e3040;
                                                            } else {
                                                                phi_4821_ = _e2915;
                                                            }
                                                            let _e3042 = phi_4821_;
                                                            phi_4477_ = _e3042;
                                                            phi_4480_ = (_e2917 + 1u);
                                                        } else {
                                                            phi_4477_ = vec3<f32>();
                                                            phi_4480_ = u32();
                                                        }
                                                        let _e3045 = phi_4477_;
                                                        let _e3047 = phi_4480_;
                                                        phi_4476_ = _e3045;
                                                        phi_4479_ = _e3047;
                                                        phi_30404_ = _e2822;
                                                        if !(_e2918) {
                                                            break;
                                                        }
                                                    }
                                                    let _e3050 = phi_30404_;
                                                    phi_30727_ = _e3050;
                                                    if _e3050 {
                                                        break;
                                                    }
                                                    let _e3052 = local_14;
                                                    phi_30452_ = _e3050;
                                                    phi_4832_ = (_e3052.x * (1f / ((6.2831855f * _e2902) * _e2902)));
                                                } else {
                                                    phi_30452_ = _e2822;
                                                    phi_4832_ = 0f;
                                                }
                                                let _e3056 = phi_30452_;
                                                let _e3058 = phi_4832_;
                                                if (_e2833 != 4294967295u) {
                                                    let _e3075 = (_e2888 * 1.3333334f);
                                                    let _e3082 = global_6.member[_e2833];
                                                    phi_4856_ = (_e3058 + ((_e3082 * ((exp((-(abs(_e2888)) / (0.06f + (abs(_e2899) * 0.015915494f)))) * (0.3f + (0.7f * exp(((_e2899 * -0.71428573f) * (_e2899 * 0.71428573f)))))) + (1.3f * exp((((_e2899 * _e2899) + (_e3075 * _e3075)) * -12.499999f))))) * 4.73f));
                                                } else {
                                                    phi_4856_ = _e3058;
                                                }
                                                let _e3088 = phi_4856_;
                                                phi_4858_ = 0u;
                                                phi_4861_ = _e3088;
                                                loop {
                                                    let _e3090 = phi_4858_;
                                                    let _e3092 = phi_4861_;
                                                    local_109 = _e3092;
                                                    let _e3093 = (_e3090 < bitcast<u32>(_e2840));
                                                    if _e3093 {
                                                        let _e3095 = (bitcast<u32>(_e2836) + (_e3090 * 18u));
                                                        let _e3098 = global_6.member[(_e3095 + 1u)];
                                                        let _e3101 = global_6.member[(_e3095 + 2u)];
                                                        let _e3104 = global_6.member[(_e3095 + 3u)];
                                                        let _e3112 = global_6.member[(_e3095 + 5u)];
                                                        if ((((_e1173.x * _e3098) + (_e1173.y * _e3101)) + (_e1173.z * _e3104)) >= _e3112) {
                                                            let _e3115 = global_6.member[_e3095];
                                                            let _e3120 = select(select(u32(_e3115), 0u, (_e3115 < 0f)), 4294967295u, (_e3115 > 4294967000f));
                                                            if (_e3120 == 0u) {
                                                                phi_4968_ = 1.8f;
                                                            } else {
                                                                if (_e3120 == 1u) {
                                                                    phi_4967_ = 1f;
                                                                } else {
                                                                    phi_4967_ = select(0f, 4.7f, (_e3120 == 2u));
                                                                }
                                                                let _e3126 = phi_4967_;
                                                                phi_4968_ = _e3126;
                                                            }
                                                            let _e3128 = phi_4968_;
                                                            let _e3131 = global_6.member[(_e3095 + 6u)];
                                                            phi_4984_ = (_e3092 + (_e3131 * _e3128));
                                                        } else {
                                                            phi_4984_ = _e3092;
                                                        }
                                                        let _e3135 = phi_4984_;
                                                        phi_4859_ = (_e3090 + 1u);
                                                        phi_4862_ = _e3135;
                                                    } else {
                                                        phi_4859_ = u32();
                                                        phi_4862_ = f32();
                                                    }
                                                    let _e3138 = phi_4859_;
                                                    let _e3140 = phi_4862_;
                                                    phi_4858_ = _e3138;
                                                    phi_4861_ = _e3140;
                                                    if !(_e3093) {
                                                        break;
                                                    }
                                                }
                                                phi_4987_ = 0u;
                                                let _e14063 = local_109;
                                                phi_4990_ = _e14063;
                                                loop {
                                                    let _e3143 = phi_4987_;
                                                    let _e3145 = phi_4990_;
                                                    local_15 = _e3145;
                                                    let _e3146 = (_e3143 < bitcast<u32>(_e2848));
                                                    if _e3146 {
                                                        let _e3148 = (bitcast<u32>(_e2844) + (_e3143 * 29u));
                                                        let _e3151 = global_6.member[(_e3148 + 1u)];
                                                        let _e3154 = global_6.member[(_e3148 + 2u)];
                                                        let _e3157 = global_6.member[(_e3148 + 3u)];
                                                        let _e3165 = global_6.member[(_e3148 + 5u)];
                                                        if ((((_e1173.x * _e3151) + (_e1173.y * _e3154)) + (_e1173.z * _e3157)) >= _e3165) {
                                                            let _e3168 = global_6.member[_e3148];
                                                            let _e3172 = global_6.member[(_e3148 + 6u)];
                                                            phi_5101_ = (_e3145 + (_e3172 * select(1f, 3.3808f, (_e3168 < 0.5f))));
                                                        } else {
                                                            phi_5101_ = _e3145;
                                                        }
                                                        let _e3177 = phi_5101_;
                                                        phi_4988_ = (_e3143 + 1u);
                                                        phi_4991_ = _e3177;
                                                    } else {
                                                        phi_4988_ = u32();
                                                        phi_4991_ = f32();
                                                    }
                                                    let _e3180 = phi_4988_;
                                                    let _e3182 = phi_4991_;
                                                    phi_4987_ = _e3180;
                                                    phi_4990_ = _e3182;
                                                    if !(_e3146) {
                                                        break;
                                                    }
                                                }
                                                let _e3186 = global_6.member[(_e1189 + 9u)];
                                                let _e3188 = local_15;
                                                let _e3191 = local_16;
                                                let _e3195 = local_17;
                                                let _e3200 = local_18;
                                                let _e3205 = ((_e3188 * _e3186) <= (_e2825 * (((0.2126f * _e3191.x) + (0.7152f * _e3195.y)) + (0.0722f * _e3200.z))));
                                                if _e3205 {
                                                    phi_30493_ = _e3056;
                                                    phi_5667_ = vec3<f32>();
                                                    phi_5668_ = bool();
                                                } else {
                                                    let _e3207 = global_6.member[_e1186];
                                                    let _e3210 = global_6.member[(_e1186 + 1u)];
                                                    let _e3213 = global_6.member[(_e1186 + 2u)];
                                                    let _e3216 = global_6.member[(_e1186 + 3u)];
                                                    let _e3219 = global_6.member[(_e1186 + 5u)];
                                                    let _e3222 = global_6.member[(_e1186 + 6u)];
                                                    let _e3225 = global_6.member[(_e1186 + 10u)];
                                                    let _e3228 = global_6.member[(_e1186 + 11u)];
                                                    let _e3231 = global_6.member[(_e1186 + 13u)];
                                                    let _e3234 = global_6.member[(_e1186 + 14u)];
                                                    let _e3237 = global_6.member[(_e1186 + 15u)];
                                                    let _e3240 = global_6.member[(_e1186 + 16u)];
                                                    let _e3243 = global_6.member[(_e1186 + 18u)];
                                                    let _e3246 = global_6.member[(_e1186 + 19u)];
                                                    let _e3249 = global_6.member[(_e1186 + 20u)];
                                                    let _e3264 = ((((_e1174.x * _e1174.x) + (_e1174.y * _e1174.y)) + (_e1174.z * _e1174.z)) - (2f * (((_e1174.x * _e3207) + (_e1174.y * _e3210)) + (_e1174.z * _e3213))));
                                                    let _e3265 = (_e3264 + _e3219);
                                                    let _e3266 = (_e3265 > 0f);
                                                    if _e3266 {
                                                        let _e3274 = ((((_e1174.x - _e3207) * _e1173.x) + ((_e1174.y - _e3210) * _e1173.y)) + ((_e1174.z - _e3213) * _e1173.z));
                                                        let _e3276 = ((_e3274 * _e3274) - _e3265);
                                                        if (_e3276 < 0f) {
                                                            phi_21774_ = type_57(0u, type_57().member_1);
                                                            phi_21775_ = type_57();
                                                            phi_21776_ = true;
                                                        } else {
                                                            if (_e3274 < 0f) {
                                                                phi_21746_ = -(sqrt(_e3276));
                                                            } else {
                                                                phi_21746_ = sqrt(_e3276);
                                                            }
                                                            let _e3283 = phi_21746_;
                                                            let _e3285 = -((_e3274 + _e3283));
                                                            let _e3286 = (_e3285 == 0f);
                                                            if _e3286 {
                                                                phi_21769_ = type_57(1u, type_52(0f, 0f));
                                                                phi_21770_ = type_57();
                                                            } else {
                                                                let _e3287 = (_e3265 / _e3285);
                                                                if (_e3285 < _e3287) {
                                                                    phi_21758_ = type_52(_e3285, _e3287);
                                                                } else {
                                                                    phi_21758_ = type_52(_e3287, _e3285);
                                                                }
                                                                let _e3292 = phi_21758_;
                                                                phi_21769_ = type_57();
                                                                phi_21770_ = type_57(1u, _e3292);
                                                            }
                                                            let _e3295 = phi_21769_;
                                                            let _e3297 = phi_21770_;
                                                            phi_21774_ = _e3295;
                                                            phi_21775_ = _e3297;
                                                            phi_21776_ = _e3286;
                                                        }
                                                        let _e3302 = phi_21774_;
                                                        let _e3304 = phi_21775_;
                                                        let _e3306 = phi_21776_;
                                                        if _e3306 {
                                                            phi_21780_ = _e3302;
                                                        } else {
                                                            phi_21780_ = _e3304;
                                                        }
                                                        let _e3308 = phi_21780_;
                                                        if (bitcast<i32>(_e3308.member) != 0i) {
                                                            phi_21714_ = select(_e3308.member_1.member_1, _e3308.member_1.member, (_e3308.member_1.member >= 0f));
                                                        } else {
                                                            phi_21714_ = -1f;
                                                        }
                                                        let _e3319 = phi_21714_;
                                                        phi_5502_ = select(true, false, (_e3319 >= 0f));
                                                    } else {
                                                        phi_5502_ = true;
                                                    }
                                                    let _e3323 = phi_5502_;
                                                    if _e3323 {
                                                        let _e3331 = ((((_e1174.x - _e3207) * _e1173.x) + ((_e1174.y - _e3210) * _e1173.y)) + ((_e1174.z - _e3213) * _e1173.z));
                                                        let _e3332 = (_e3264 + _e3222);
                                                        let _e3333 = (_e3331 * _e3331);
                                                        let _e3334 = (_e3333 - _e3332);
                                                        if (_e3334 < 0f) {
                                                            phi_21934_ = type_57(0u, type_57().member_1);
                                                            phi_21935_ = type_57();
                                                            phi_21936_ = true;
                                                        } else {
                                                            if (_e3331 < 0f) {
                                                                phi_21906_ = -(sqrt(_e3334));
                                                            } else {
                                                                phi_21906_ = sqrt(_e3334);
                                                            }
                                                            let _e3341 = phi_21906_;
                                                            let _e3343 = -((_e3331 + _e3341));
                                                            let _e3344 = (_e3343 == 0f);
                                                            if _e3344 {
                                                                phi_21929_ = type_57(1u, type_52(0f, 0f));
                                                                phi_21930_ = type_57();
                                                            } else {
                                                                let _e3345 = (_e3332 / _e3343);
                                                                if (_e3343 < _e3345) {
                                                                    phi_21918_ = type_52(_e3343, _e3345);
                                                                } else {
                                                                    phi_21918_ = type_52(_e3345, _e3343);
                                                                }
                                                                let _e3350 = phi_21918_;
                                                                phi_21929_ = type_57();
                                                                phi_21930_ = type_57(1u, _e3350);
                                                            }
                                                            let _e3353 = phi_21929_;
                                                            let _e3355 = phi_21930_;
                                                            phi_21934_ = _e3353;
                                                            phi_21935_ = _e3355;
                                                            phi_21936_ = _e3344;
                                                        }
                                                        let _e3360 = phi_21934_;
                                                        let _e3362 = phi_21935_;
                                                        let _e3364 = phi_21936_;
                                                        if _e3364 {
                                                            phi_21940_ = _e3360;
                                                        } else {
                                                            phi_21940_ = _e3362;
                                                        }
                                                        let _e3366 = phi_21940_;
                                                        if (bitcast<i32>(_e3366.member) != 0i) {
                                                            phi_21827_ = type_57(0u, _e3366.member_1);
                                                        } else {
                                                            phi_21827_ = type_57(1u, type_57().member_1);
                                                        }
                                                        let _e3376 = phi_21827_;
                                                        if (bitcast<i32>(_e3376.member) != 0i) {
                                                            phi_21874_ = type_57(0u, type_57().member_1);
                                                        } else {
                                                            if (_e3376.member_1.member_1 <= 0f) {
                                                                phi_21870_ = type_57(0u, type_57().member_1);
                                                            } else {
                                                                if (_e3376.member_1.member != _e3376.member_1.member) {
                                                                    phi_21978_ = true;
                                                                } else {
                                                                    phi_21978_ = (0f >= _e3376.member_1.member);
                                                                }
                                                                let _e3388 = phi_21978_;
                                                                let _e3389 = select(_e3376.member_1.member, 0f, _e3388);
                                                                if _e3266 {
                                                                    let _e3390 = (_e3333 - _e3265);
                                                                    if (_e3390 < 0f) {
                                                                        phi_22086_ = type_57(0u, type_57().member_1);
                                                                        phi_22087_ = type_57();
                                                                        phi_22088_ = true;
                                                                    } else {
                                                                        if (_e3331 < 0f) {
                                                                            phi_22058_ = -(sqrt(_e3390));
                                                                        } else {
                                                                            phi_22058_ = sqrt(_e3390);
                                                                        }
                                                                        let _e3397 = phi_22058_;
                                                                        let _e3399 = -((_e3331 + _e3397));
                                                                        let _e3400 = (_e3399 == 0f);
                                                                        if _e3400 {
                                                                            phi_22081_ = type_57(1u, type_52(0f, 0f));
                                                                            phi_22082_ = type_57();
                                                                        } else {
                                                                            let _e3401 = (_e3265 / _e3399);
                                                                            if (_e3399 < _e3401) {
                                                                                phi_22070_ = type_52(_e3399, _e3401);
                                                                            } else {
                                                                                phi_22070_ = type_52(_e3401, _e3399);
                                                                            }
                                                                            let _e3406 = phi_22070_;
                                                                            phi_22081_ = type_57();
                                                                            phi_22082_ = type_57(1u, _e3406);
                                                                        }
                                                                        let _e3409 = phi_22081_;
                                                                        let _e3411 = phi_22082_;
                                                                        phi_22086_ = _e3409;
                                                                        phi_22087_ = _e3411;
                                                                        phi_22088_ = _e3400;
                                                                    }
                                                                    let _e3416 = phi_22086_;
                                                                    let _e3418 = phi_22087_;
                                                                    let _e3420 = phi_22088_;
                                                                    if _e3420 {
                                                                        phi_22092_ = _e3416;
                                                                    } else {
                                                                        phi_22092_ = _e3418;
                                                                    }
                                                                    let _e3422 = phi_22092_;
                                                                    if (bitcast<i32>(_e3422.member) != 0i) {
                                                                        phi_22026_ = select(_e3422.member_1.member_1, _e3422.member_1.member, (_e3422.member_1.member >= 0f));
                                                                    } else {
                                                                        phi_22026_ = -1f;
                                                                    }
                                                                    let _e3433 = phi_22026_;
                                                                    if (_e3433 >= 0f) {
                                                                        if (_e3376.member_1.member_1 != _e3376.member_1.member_1) {
                                                                            phi_22130_ = true;
                                                                        } else {
                                                                            phi_22130_ = (_e3433 <= _e3376.member_1.member_1);
                                                                        }
                                                                        let _e3438 = phi_22130_;
                                                                        phi_21855_ = select(_e3376.member_1.member_1, _e3433, _e3438);
                                                                    } else {
                                                                        phi_21855_ = _e3376.member_1.member_1;
                                                                    }
                                                                    let _e3441 = phi_21855_;
                                                                    phi_21857_ = _e3441;
                                                                } else {
                                                                    phi_21857_ = _e3376.member_1.member_1;
                                                                }
                                                                let _e3443 = phi_21857_;
                                                                if (_e3443 <= _e3389) {
                                                                    phi_21866_ = type_57(0u, type_57().member_1);
                                                                } else {
                                                                    phi_21866_ = type_57(1u, type_52(_e3389, _e3443));
                                                                }
                                                                let _e3451 = phi_21866_;
                                                                phi_21870_ = _e3451;
                                                            }
                                                            let _e3456 = phi_21870_;
                                                            phi_21874_ = _e3456;
                                                        }
                                                        let _e3461 = phi_21874_;
                                                        let _e3464 = (bitcast<i32>(_e3461.member) != 0i);
                                                        if _e3464 {
                                                            phi_5516_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_5519_ = 0u;
                                                            loop {
                                                                let _e3470 = phi_5516_;
                                                                let _e3472 = phi_5519_;
                                                                local_19 = _e3470;
                                                                local_20 = _e3470;
                                                                local_21 = _e3470;
                                                                let _e3473 = (_e3472 < 32u);
                                                                if _e3473 {
                                                                    let _e3474 = -(_e3331);
                                                                    if (_e3461.member_1.member <= _e3461.member_1.member_1) {
                                                                    } else {
                                                                        phi_30431_ = true;
                                                                        break;
                                                                    }
                                                                    let _e3477 = select(_e3474, _e3461.member_1.member, (_e3474 < _e3461.member_1.member));
                                                                    let _e3479 = select(_e3477, _e3461.member_1.member_1, (_e3477 > _e3461.member_1.member_1));
                                                                    let _e3480 = (_e3479 - _e3461.member_1.member);
                                                                    let _e3481 = (_e3461.member_1.member_1 - _e3479);
                                                                    let _e3482 = (_e3461.member_1.member_1 - _e3461.member_1.member);
                                                                    let _e3483 = (0.000000001f * _e3482);
                                                                    if (_e3480 <= _e3483) {
                                                                        let _e3558 = (f32(_e3472) * 0.03125f);
                                                                        let _e3561 = (f32((_e3472 + 1u)) * 0.03125f);
                                                                        let _e3563 = ((_e3558 + _e3561) * 0.5f);
                                                                        phi_5626_ = type_52((_e3461.member_1.member + ((_e3482 * _e3563) * _e3563)), (_e3482 * ((_e3561 * _e3561) - (_e3558 * _e3558))));
                                                                    } else {
                                                                        if (_e3481 <= _e3483) {
                                                                            let _e3541 = (f32((31u - _e3472)) * 0.03125f);
                                                                            let _e3544 = (f32((32u - _e3472)) * 0.03125f);
                                                                            let _e3546 = ((_e3541 + _e3544) * 0.5f);
                                                                            phi_5625_ = type_52((_e3461.member_1.member_1 - ((_e3482 * _e3546) * _e3546)), (_e3482 * ((_e3544 * _e3544) - (_e3541 * _e3541))));
                                                                        } else {
                                                                            let _e3489 = round(((32f * _e3480) / (_e3480 + _e3481)));
                                                                            let _e3494 = select(select(u32(_e3489), 0u, (_e3489 < 0f)), 4294967295u, (_e3489 > 4294967000f));
                                                                            if (_e3494 < 1u) {
                                                                                phi_5609_ = 1u;
                                                                            } else {
                                                                                phi_5609_ = select(_e3494, 31u, (_e3494 > 31u));
                                                                            }
                                                                            let _e3499 = phi_5609_;
                                                                            if (_e3472 < _e3499) {
                                                                                let _e3520 = ((_e3499 - 1u) - _e3472);
                                                                                let _e3522 = f32(_e3499);
                                                                                let _e3523 = (f32(_e3520) / _e3522);
                                                                                let _e3526 = (f32((_e3520 + 1u)) / _e3522);
                                                                                let _e3528 = ((_e3523 + _e3526) * 0.5f);
                                                                                phi_5624_ = type_52((_e3479 - ((_e3480 * _e3528) * _e3528)), (_e3480 * ((_e3526 * _e3526) - (_e3523 * _e3523))));
                                                                            } else {
                                                                                let _e3502 = (_e3472 - _e3499);
                                                                                let _e3504 = f32((32u - _e3499));
                                                                                let _e3505 = (f32(_e3502) / _e3504);
                                                                                let _e3508 = (f32((_e3502 + 1u)) / _e3504);
                                                                                let _e3510 = ((_e3505 + _e3508) * 0.5f);
                                                                                phi_5624_ = type_52((_e3479 + ((_e3481 * _e3510) * _e3510)), (_e3481 * ((_e3508 * _e3508) - (_e3505 * _e3505))));
                                                                            }
                                                                            let _e3538 = phi_5624_;
                                                                            phi_5625_ = _e3538;
                                                                        }
                                                                        let _e3556 = phi_5625_;
                                                                        phi_5626_ = _e3556;
                                                                    }
                                                                    let _e3573 = phi_5626_;
                                                                    let _e3580 = (_e1174 + vec3<f32>((_e1173.x * _e3573.member), (_e1173.y * _e3573.member), (_e1173.z * _e3573.member)));
                                                                    let _e3597 = (_e3580.x - _e3207);
                                                                    let _e3598 = (_e3580.y - _e3210);
                                                                    let _e3599 = (_e3580.z - _e3213);
                                                                    let _e3607 = ((((((_e3580.x * _e3580.x) + (_e3580.y * _e3580.y)) + (_e3580.z * _e3580.z)) - (2f * (((_e3580.x * _e3207) + (_e3580.y * _e3210)) + (_e3580.z * _e3213)))) + _e3219) / (sqrt((((_e3597 * _e3597) + (_e3598 * _e3598)) + (_e3599 * _e3599))) + _e3216));
                                                                    let _e3611 = (1f - (abs((_e3607 - 25000f)) * 0.00006666667f));
                                                                    if (_e3611 != _e3611) {
                                                                        phi_22329_ = true;
                                                                    } else {
                                                                        phi_22329_ = (0f >= _e3611);
                                                                    }
                                                                    let _e3615 = phi_22329_;
                                                                    let _e3617 = -(_e3607);
                                                                    phi_5517_ = (_e3470 + vec3<f32>((exp((_e3617 / _e3225)) * _e3573.member_1), (exp((_e3617 / _e3228)) * _e3573.member_1), (select(_e3611, 0f, _e3615) * _e3573.member_1)));
                                                                    phi_5520_ = (_e3472 + 1u);
                                                                } else {
                                                                    phi_5517_ = vec3<f32>();
                                                                    phi_5520_ = u32();
                                                                }
                                                                let _e3629 = phi_5517_;
                                                                let _e3631 = phi_5520_;
                                                                phi_5516_ = _e3629;
                                                                phi_5519_ = _e3631;
                                                                phi_30431_ = _e3056;
                                                                if !(_e3473) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e3634 = phi_30431_;
                                                            phi_30727_ = _e3634;
                                                            if _e3634 {
                                                                break;
                                                            }
                                                            let _e3636 = local_19;
                                                            let _e3643 = local_20;
                                                            let _e3645 = ((_e3240 * 1.1f) * _e3643.y);
                                                            let _e3650 = local_21;
                                                            phi_30498_ = _e3634;
                                                            phi_5652_ = vec3<f32>(exp(-((((_e3231 * _e3636.x) + _e3645) + (_e3243 * _e3650.z)))), exp(-((((_e3234 * _e3636.x) + _e3645) + (_e3246 * _e3650.z)))), exp(-((((_e3237 * _e3636.x) + _e3645) + (_e3249 * _e3650.z)))));
                                                        } else {
                                                            phi_30498_ = _e3056;
                                                            phi_5652_ = vec3<f32>();
                                                        }
                                                        let _e3666 = phi_30498_;
                                                        let _e3668 = phi_5652_;
                                                        phi_30497_ = _e3666;
                                                        phi_5654_ = _e3668;
                                                        phi_5656_ = select(true, false, _e3464);
                                                    } else {
                                                        phi_30497_ = _e3056;
                                                        phi_5654_ = vec3<f32>();
                                                        phi_5656_ = true;
                                                    }
                                                    let _e3671 = phi_30497_;
                                                    let _e3673 = phi_5654_;
                                                    let _e3675 = phi_5656_;
                                                    let _e3679 = select(_e3673, select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e3323)), vec3(_e3675));
                                                    let _e3683 = select(_e3679.y, _e3679.x, (_e3679.x > _e3679.y));
                                                    phi_30493_ = _e3671;
                                                    phi_5667_ = _e3679;
                                                    phi_5668_ = (select(_e3679.z, _e3683, (_e3683 > _e3679.z)) > 0f);
                                                }
                                                let _e3689 = phi_30493_;
                                                let _e3691 = phi_5667_;
                                                let _e3693 = phi_5668_;
                                                phi_30492_ = _e3689;
                                                phi_5670_ = _e3691;
                                                phi_5671_ = _e3693;
                                                phi_5672_ = select(true, false, _e3205);
                                            } else {
                                                phi_30492_ = _e2822;
                                                phi_5670_ = vec3<f32>(1f, 1f, 1f);
                                                phi_5671_ = _e1340;
                                                phi_5672_ = true;
                                            }
                                            let _e3696 = phi_30492_;
                                            let _e3698 = phi_5670_;
                                            let _e3700 = phi_5671_;
                                            let _e3702 = phi_5672_;
                                            phi_30491_ = _e3696;
                                            let _e14134 = local_110;
                                            phi_5676_ = _e14134;
                                            phi_5677_ = _e3698;
                                            phi_5678_ = _e3700;
                                            let _e14139 = local_111;
                                            phi_5679_ = _e14139;
                                            phi_5680_ = _e3702;
                                        } else {
                                            phi_30491_ = _e1160;
                                            phi_5676_ = vec3<f32>(0f, 0f, 0f);
                                            phi_5677_ = vec3<f32>(1f, 1f, 1f);
                                            phi_5678_ = _e1340;
                                            phi_5679_ = vec3<f32>();
                                            phi_5680_ = true;
                                        }
                                        let _e3704 = phi_30491_;
                                        let _e3706 = phi_5676_;
                                        let _e3708 = phi_5677_;
                                        let _e3710 = phi_5678_;
                                        let _e3712 = phi_5679_;
                                        let _e3714 = phi_5680_;
                                        if _e3714 {
                                            if _e3710 {
                                                let _e3715 = (_e1189 + 20u);
                                                let _e3718 = global_6.member[(_e1189 + 21u)];
                                                let _e3719 = bitcast<u32>(_e3718);
                                                let _e3722 = global_6.member[(_e1189 + 22u)];
                                                let _e3723 = bitcast<u32>(_e3722);
                                                let _e3726 = global_6.member[(_e1189 + 23u)];
                                                let _e3730 = global_6.member[(_e1189 + 24u)];
                                                let _e3734 = global_6.member[(_e1189 + 25u)];
                                                let _e3738 = global_6.member[(_e1189 + 26u)];
                                                let _e3741 = global_6.member[_e1189];
                                                let _e3744 = global_6.member[(_e1189 + 1u)];
                                                let _e3747 = global_6.member[(_e1189 + 2u)];
                                                let _e3750 = global_6.member[(_e1189 + 3u)];
                                                let _e3753 = global_6.member[(_e1189 + 4u)];
                                                let _e3756 = global_6.member[(_e1189 + 5u)];
                                                let _e3759 = global_6.member[(_e1189 + 6u)];
                                                let _e3762 = global_6.member[(_e1189 + 7u)];
                                                let _e3765 = global_6.member[(_e1189 + 8u)];
                                                let _e3773 = (((_e1173.x * _e3750) + (_e1173.y * _e3753)) + (_e1173.z * _e3756));
                                                let _e3775 = select(_e3773, -1f, (_e3773 < -1f));
                                                let _e3778 = asin(select(_e3775, 1f, (_e3775 > 1f)));
                                                let _e3789 = atan2((((_e1173.x * _e3759) + (_e1173.y * _e3762)) + (_e1173.z * _e3765)), (((_e1173.x * _e3741) + (_e1173.y * _e3744)) + (_e1173.z * _e3747)));
                                                if (_e3719 != 4294967295u) {
                                                    let _e3792 = global_6.member[_e3719];
                                                    let _e3795 = global_6.member[(_e3719 + 1u)];
                                                    let _e3796 = bitcast<u32>(_e3795);
                                                    let _e3799 = global_6.member[(_e3719 + 2u)];
                                                    let _e3803 = global_6.member[(_e3719 + 3u)];
                                                    let _e3805 = (4f * _e3792);
                                                    phi_6009_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_6012_ = 0u;
                                                    loop {
                                                        let _e3810 = phi_6009_;
                                                        let _e3812 = phi_6012_;
                                                        local_115 = _e3810;
                                                        let _e3813 = (_e3812 < 6u);
                                                        if _e3813 {
                                                            let _e3814 = (_e3812 / 2u);
                                                            if (_e3814 == 0u) {
                                                                phi_22400_ = _e1173.x;
                                                            } else {
                                                                phi_22400_ = select(_e1173.z, _e1173.y, (_e3814 == 1u));
                                                            }
                                                            let _e3822 = phi_22400_;
                                                            let _e3823 = (_e3822 * select(-1f, 1f, ((_e3812 % 2u) == 0u)));
                                                            if (_e3823 > 0f) {
                                                                let _e3827 = ((_e3814 + 1u) % 3u);
                                                                if (_e3827 == 0u) {
                                                                    phi_22415_ = _e1173.x;
                                                                } else {
                                                                    phi_22415_ = select(_e1173.z, _e1173.y, (_e3827 == 1u));
                                                                }
                                                                let _e3832 = phi_22415_;
                                                                let _e3833 = (_e3832 / _e3823);
                                                                let _e3835 = ((_e3814 + 2u) % 3u);
                                                                if (_e3835 == 0u) {
                                                                    phi_22430_ = _e1173.x;
                                                                } else {
                                                                    phi_22430_ = select(_e1173.z, _e1173.y, (_e3835 == 1u));
                                                                }
                                                                let _e3840 = phi_22430_;
                                                                let _e3841 = (_e3840 / _e3823);
                                                                let _e3843 = (1f + (_e3792 * 12f));
                                                                if (abs(_e3833) <= _e3843) {
                                                                    if (abs(_e3841) <= _e3843) {
                                                                        let _e3849 = f32(_e3796);
                                                                        let _e3851 = floor((((_e3833 + 1f) * 0.5f) * _e3849));
                                                                        let _e3858 = select(0i, select(select(i32(_e3851), i32(-2147483648), (_e3851 < -2147483600f)), 2147483647i, (_e3851 > 2147483500f)), (_e3851 == _e3851));
                                                                        let _e3860 = (bitcast<i32>(_e3796) - 1i);
                                                                        let _e3861 = (0i <= _e3860);
                                                                        if _e3861 {
                                                                        } else {
                                                                            phi_30486_ = true;
                                                                            break;
                                                                        }
                                                                        if (_e3858 < 0i) {
                                                                            phi_6083_ = 0i;
                                                                        } else {
                                                                            phi_6083_ = select(_e3858, _e3860, (_e3858 > _e3860));
                                                                        }
                                                                        let _e3866 = phi_6083_;
                                                                        let _e3870 = floor((((_e3841 + 1f) * 0.5f) * _e3849));
                                                                        let _e3877 = select(0i, select(select(i32(_e3870), i32(-2147483648), (_e3870 < -2147483600f)), 2147483647i, (_e3870 > 2147483500f)), (_e3870 == _e3870));
                                                                        if _e3861 {
                                                                        } else {
                                                                            phi_30486_ = true;
                                                                            break;
                                                                        }
                                                                        if (_e3877 < 0i) {
                                                                            phi_6113_ = 0i;
                                                                        } else {
                                                                            phi_6113_ = select(_e3877, _e3860, (_e3877 > _e3860));
                                                                        }
                                                                        let _e3882 = phi_6113_;
                                                                        let _e3883 = (_e3866 - 1i);
                                                                        phi_6121_ = _e3810;
                                                                        phi_6124_ = select(0i, _e3883, (0i < _e3883));
                                                                        loop {
                                                                            let _e3887 = phi_6121_;
                                                                            let _e3889 = phi_6124_;
                                                                            let _e3890 = (_e3866 + 1i);
                                                                            local_114 = _e3887;
                                                                            let _e3893 = (_e3889 <= select(_e3890, _e3860, (_e3860 < _e3890)));
                                                                            if _e3893 {
                                                                                let _e3894 = (_e3882 - 1i);
                                                                                phi_6147_ = _e3887;
                                                                                phi_6150_ = select(0i, _e3894, (0i < _e3894));
                                                                                loop {
                                                                                    let _e3898 = phi_6147_;
                                                                                    let _e3900 = phi_6150_;
                                                                                    let _e3901 = (_e3882 + 1i);
                                                                                    local_113 = _e3898;
                                                                                    let _e3904 = (_e3900 <= select(_e3901, _e3860, (_e3860 < _e3901)));
                                                                                    if _e3904 {
                                                                                        let _e3911 = (bitcast<u32>(_e3799) + ((((_e3812 * _e3796) + bitcast<u32>(_e3889)) * _e3796) + bitcast<u32>(_e3900)));
                                                                                        let _e3913 = global_6.member[_e3911];
                                                                                        let _e3917 = global_6.member[(_e3911 + 1u)];
                                                                                        phi_6221_ = _e3898;
                                                                                        phi_6224_ = bitcast<u32>(_e3913);
                                                                                        loop {
                                                                                            let _e3920 = phi_6221_;
                                                                                            let _e3922 = phi_6224_;
                                                                                            local_112 = _e3920;
                                                                                            let _e3923 = (_e3922 < bitcast<u32>(_e3917));
                                                                                            if _e3923 {
                                                                                                let _e3925 = (bitcast<u32>(_e3803) + (6u * _e3922));
                                                                                                let _e3927 = global_6.member[_e3925];
                                                                                                let _e3930 = global_6.member[(_e3925 + 1u)];
                                                                                                let _e3933 = global_6.member[(_e3925 + 2u)];
                                                                                                let _e3936 = global_6.member[(_e3925 + 3u)];
                                                                                                let _e3939 = global_6.member[(_e3925 + 4u)];
                                                                                                let _e3942 = global_6.member[(_e3925 + 5u)];
                                                                                                let _e3943 = (_e1173.x - _e3927);
                                                                                                let _e3944 = (_e1173.y - _e3930);
                                                                                                let _e3945 = (_e1173.z - _e3933);
                                                                                                let _e3950 = (((_e3943 * _e3943) + (_e3944 * _e3944)) + (_e3945 * _e3945));
                                                                                                if (_e3950 < (_e3805 * _e3805)) {
                                                                                                    let _e3958 = ((1f / ((6.2831855f * _e3792) * _e3792)) * exp((-(_e3950) / ((2f * _e3792) * _e3792))));
                                                                                                    phi_6347_ = vec3<f32>((_e3920.x + (_e3936 * _e3958)), (_e3920.y + (_e3939 * _e3958)), (_e3920.z + (_e3942 * _e3958)));
                                                                                                } else {
                                                                                                    phi_6347_ = _e3920;
                                                                                                }
                                                                                                let _e3970 = phi_6347_;
                                                                                                phi_6222_ = _e3970;
                                                                                                phi_6225_ = (_e3922 + 1u);
                                                                                            } else {
                                                                                                phi_6222_ = vec3<f32>();
                                                                                                phi_6225_ = u32();
                                                                                            }
                                                                                            let _e3973 = phi_6222_;
                                                                                            let _e3975 = phi_6225_;
                                                                                            phi_6221_ = _e3973;
                                                                                            phi_6224_ = _e3975;
                                                                                            if !(_e3923) {
                                                                                                break;
                                                                                            }
                                                                                        }
                                                                                        let _e14159 = local_112;
                                                                                        phi_6148_ = _e14159;
                                                                                        phi_6151_ = (_e3900 + 1i);
                                                                                    } else {
                                                                                        phi_6148_ = vec3<f32>();
                                                                                        phi_6151_ = i32();
                                                                                    }
                                                                                    let _e3979 = phi_6148_;
                                                                                    let _e3981 = phi_6151_;
                                                                                    phi_6147_ = _e3979;
                                                                                    phi_6150_ = _e3981;
                                                                                    if !(_e3904) {
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                let _e14163 = local_113;
                                                                                phi_6122_ = _e14163;
                                                                                phi_6125_ = (_e3889 + 1i);
                                                                            } else {
                                                                                phi_6122_ = vec3<f32>();
                                                                                phi_6125_ = i32();
                                                                            }
                                                                            let _e3985 = phi_6122_;
                                                                            let _e3987 = phi_6125_;
                                                                            phi_6121_ = _e3985;
                                                                            phi_6124_ = _e3987;
                                                                            if !(_e3893) {
                                                                                break;
                                                                            }
                                                                        }
                                                                        let _e14167 = local_114;
                                                                        phi_6352_ = _e14167;
                                                                    } else {
                                                                        phi_6352_ = _e3810;
                                                                    }
                                                                    let _e3990 = phi_6352_;
                                                                    phi_6353_ = _e3990;
                                                                } else {
                                                                    phi_6353_ = _e3810;
                                                                }
                                                                let _e3992 = phi_6353_;
                                                                phi_6354_ = _e3992;
                                                            } else {
                                                                phi_6354_ = _e3810;
                                                            }
                                                            let _e3994 = phi_6354_;
                                                            phi_6010_ = _e3994;
                                                            phi_6013_ = (_e3812 + 1u);
                                                        } else {
                                                            phi_6010_ = vec3<f32>();
                                                            phi_6013_ = u32();
                                                        }
                                                        let _e3997 = phi_6010_;
                                                        let _e3999 = phi_6013_;
                                                        phi_6009_ = _e3997;
                                                        phi_6012_ = _e3999;
                                                        phi_30486_ = _e3704;
                                                        if !(_e3813) {
                                                            break;
                                                        }
                                                    }
                                                    let _e4002 = phi_30486_;
                                                    phi_30727_ = _e4002;
                                                    if _e4002 {
                                                        break;
                                                    }
                                                    phi_30528_ = _e4002;
                                                    let _e14176 = local_115;
                                                    phi_6374_ = _e14176;
                                                } else {
                                                    phi_30528_ = _e3704;
                                                    phi_6374_ = vec3<f32>(0f, 0f, 0f);
                                                }
                                                let _e4004 = phi_30528_;
                                                let _e4006 = phi_6374_;
                                                if (_e3723 != 4294967295u) {
                                                    let _e4009 = global_6.member[_e3723];
                                                    let _e4012 = global_6.member[(_e3723 + 1u)];
                                                    let _e4015 = global_6.member[(_e3723 + 2u)];
                                                    let _e4018 = global_6.member[(_e3723 + 3u)];
                                                    let _e4021 = global_6.member[(_e3723 + 4u)];
                                                    let _e4024 = global_6.member[(_e3723 + 5u)];
                                                    let _e4027 = global_6.member[(_e3723 + 6u)];
                                                    let _e4030 = global_6.member[(_e3723 + 7u)];
                                                    let _e4033 = global_6.member[(_e3723 + 8u)];
                                                    let _e4036 = global_6.member[(_e3723 + 9u)];
                                                    let _e4039 = global_6.member[(_e3723 + 10u)];
                                                    let _e4043 = global_6.member[(_e3723 + 11u)];
                                                    let _e4046 = global_6.member[(_e3723 + 12u)];
                                                    let _e4049 = global_6.member[(_e3723 + 13u)];
                                                    let _e4051 = (_e4009 > 0f);
                                                    let _e4052 = (_e4012 > 0f);
                                                    if _e4052 {
                                                        let _e4055 = (abs(_e3778) <= 0.8f);
                                                        let _e4056 = select(2u, 0u, _e4051);
                                                        phi_6587_ = _e4056;
                                                        phi_6588_ = _e4055;
                                                        phi_6591_ = _e4056;
                                                        phi_6592_ = _e4055;
                                                        phi_6593_ = select(true, false, _e4055);
                                                    } else {
                                                        phi_6587_ = u32();
                                                        phi_6588_ = bool();
                                                        phi_6591_ = select(2u, 0u, _e4051);
                                                        phi_6592_ = false;
                                                        phi_6593_ = true;
                                                    }
                                                    let _e4059 = phi_6587_;
                                                    let _e4061 = phi_6588_;
                                                    let _e4063 = phi_6591_;
                                                    let _e4065 = phi_6592_;
                                                    let _e4067 = phi_6593_;
                                                    let _e4069 = select(select(f32(), 0f, _e4052), 0f, _e4067);
                                                    phi_6603_ = _e4069;
                                                    phi_6606_ = _e4069;
                                                    phi_6608_ = select(_e4059, _e4063, _e4067);
                                                    phi_6610_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_6612_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_6614_ = 0f;
                                                    phi_6616_ = 1f;
                                                    loop {
                                                        let _e4074 = phi_6603_;
                                                        let _e4076 = phi_6606_;
                                                        let _e4078 = phi_6608_;
                                                        let _e4080 = phi_6610_;
                                                        let _e4082 = phi_6612_;
                                                        let _e4084 = phi_6614_;
                                                        let _e4086 = phi_6616_;
                                                        local_25 = _e4086;
                                                        local_26 = _e4086;
                                                        local_27 = _e4086;
                                                        local_28 = _e4086;
                                                        local_29 = _e4084;
                                                        local_30 = _e4076;
                                                        local_31 = _e4074;
                                                        let _e4087 = (_e4078 < select(11u, 2u, _e4067));
                                                        if _e4087 {
                                                            let _e4088 = (_e4078 < 2u);
                                                            if _e4088 {
                                                                phi_6694_ = 4u;
                                                                phi_6696_ = (_e1173 * select(23f, 4f, (_e4078 == 0u)));
                                                            } else {
                                                                if (_e4078 == 2u) {
                                                                    phi_6692_ = 3u;
                                                                    phi_6693_ = vec3<f32>(((_e1173.x * 1.5f) + 5f), ((_e1173.y * 1.5f) + 5f), ((_e1173.z * 1.5f) + 5f));
                                                                } else {
                                                                    let _e4090 = (_e4078 >= 7u);
                                                                    if _e4090 {
                                                                        phi_6654_ = (_e4078 - 7u);
                                                                    } else {
                                                                        phi_6654_ = (_e4078 - 3u);
                                                                    }
                                                                    let _e4094 = phi_6654_;
                                                                    if _e4090 {
                                                                        phi_6669_ = vec3<f32>(((_e1173.x * 11f) + -3f), ((_e1173.y * 11f) + 1f), ((_e1173.z * 11f) + 7f));
                                                                    } else {
                                                                        phi_6669_ = (_e1173 * 7f);
                                                                    }
                                                                    let _e4104 = phi_6669_;
                                                                    if (_e4094 < 3u) {
                                                                        if (_e4094 == 0u) {
                                                                            phi_22445_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                        } else {
                                                                            phi_22445_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e4094 == 1u)));
                                                                        }
                                                                        let _e4123 = phi_22445_;
                                                                        phi_6690_ = 4u;
                                                                        phi_6691_ = (_e4104 + _e4123);
                                                                    } else {
                                                                        if _e4090 {
                                                                            phi_6689_ = (_e4104 + _e4080);
                                                                        } else {
                                                                            phi_6689_ = (_e4104 + vec3<f32>((_e4082.x * 1.1f), (_e4082.y * 1.1f), (_e4082.z * 1.1f)));
                                                                        }
                                                                        let _e4116 = phi_6689_;
                                                                        phi_6690_ = select(7u, 5u, _e4090);
                                                                        phi_6691_ = _e4116;
                                                                    }
                                                                    let _e4126 = phi_6690_;
                                                                    let _e4128 = phi_6691_;
                                                                    phi_6692_ = _e4126;
                                                                    phi_6693_ = _e4128;
                                                                }
                                                                let _e4137 = phi_6692_;
                                                                let _e4139 = phi_6693_;
                                                                phi_6694_ = _e4137;
                                                                phi_6696_ = _e4139;
                                                            }
                                                            let _e4144 = phi_6694_;
                                                            let _e4146 = phi_6696_;
                                                            let _e4148 = select(vec3<f32>(_e4043, _e4046, _e4049), vec3<f32>(_e4033, _e4036, _e4039), vec3(_e4088));
                                                            let _e4150 = global_6.member[_e3715];
                                                            let _e4151 = bitcast<u32>(_e4150);
                                                            let _e4154 = global_6.member[(_e1189 + 11u)];
                                                            let _e4157 = global_6.member[(_e1189 + 12u)];
                                                            let _e4160 = global_6.member[(_e1189 + 13u)];
                                                            let _e4163 = global_6.member[(_e1189 + 14u)];
                                                            let _e4166 = global_6.member[(_e1189 + 15u)];
                                                            let _e4169 = global_6.member[(_e1189 + 16u)];
                                                            let _e4172 = global_6.member[(_e1189 + 17u)];
                                                            let _e4175 = global_6.member[(_e1189 + 18u)];
                                                            let _e4178 = global_6.member[(_e1189 + 19u)];
                                                            phi_6831_ = 0u;
                                                            phi_6834_ = 0f;
                                                            phi_6836_ = 1f;
                                                            phi_6838_ = 0f;
                                                            phi_6840_ = _e4146;
                                                            loop {
                                                                let _e4180 = phi_6831_;
                                                                let _e4182 = phi_6834_;
                                                                let _e4184 = phi_6836_;
                                                                let _e4186 = phi_6838_;
                                                                let _e4188 = phi_6840_;
                                                                local_22 = _e4182;
                                                                local_23 = _e4186;
                                                                local_24 = _e4182;
                                                                let _e4189 = (_e4180 < _e4144);
                                                                if _e4189 {
                                                                    let _e4192 = (_e4188.x + _e4148.x);
                                                                    let _e4195 = (_e4188.y + _e4148.y);
                                                                    let _e4198 = (_e4188.z + _e4148.z);
                                                                    let _e4199 = floor(_e4192);
                                                                    let _e4200 = floor(_e4195);
                                                                    let _e4201 = floor(_e4198);
                                                                    let _e4210 = bitcast<u32>((select(0i, select(select(i32(_e4199), i32(-2147483648), (_e4199 < -2147483600f)), 2147483647i, (_e4199 > 2147483500f)), (_e4199 == _e4199)) & 255i));
                                                                    let _e4219 = bitcast<u32>((select(0i, select(select(i32(_e4200), i32(-2147483648), (_e4200 < -2147483600f)), 2147483647i, (_e4200 > 2147483500f)), (_e4200 == _e4200)) & 255i));
                                                                    let _e4228 = bitcast<u32>((select(0i, select(select(i32(_e4201), i32(-2147483648), (_e4201 < -2147483600f)), 2147483647i, (_e4201 > 2147483500f)), (_e4201 == _e4201)) & 255i));
                                                                    let _e4229 = (_e4192 - _e4199);
                                                                    let _e4230 = (_e4195 - _e4200);
                                                                    let _e4231 = (_e4198 - _e4201);
                                                                    let _e4238 = (((_e4229 * _e4229) * _e4229) * ((_e4229 * ((_e4229 * 6f) - 15f)) + 10f));
                                                                    let _e4245 = (((_e4230 * _e4230) * _e4230) * ((_e4230 * ((_e4230 * 6f) - 15f)) + 10f));
                                                                    let _e4255 = global_6.member[(_e4151 + _e4210)];
                                                                    let _e4261 = (select(select(u32(_e4255), 0u, (_e4255 < 0f)), 4294967295u, (_e4255 > 4294967000f)) + _e4219);
                                                                    let _e4264 = global_6.member[(_e4151 + _e4261)];
                                                                    let _e4270 = (select(select(u32(_e4264), 0u, (_e4264 < 0f)), 4294967295u, (_e4264 > 4294967000f)) + _e4228);
                                                                    let _e4274 = global_6.member[(_e4151 + (_e4261 + 1u))];
                                                                    let _e4280 = (select(select(u32(_e4274), 0u, (_e4274 < 0f)), 4294967295u, (_e4274 > 4294967000f)) + _e4228);
                                                                    let _e4284 = global_6.member[(_e4151 + (_e4210 + 1u))];
                                                                    let _e4290 = (select(select(u32(_e4284), 0u, (_e4284 < 0f)), 4294967295u, (_e4284 > 4294967000f)) + _e4219);
                                                                    let _e4293 = global_6.member[(_e4151 + _e4290)];
                                                                    let _e4299 = (select(select(u32(_e4293), 0u, (_e4293 < 0f)), 4294967295u, (_e4293 > 4294967000f)) + _e4228);
                                                                    let _e4303 = global_6.member[(_e4151 + (_e4290 + 1u))];
                                                                    let _e4309 = (select(select(u32(_e4303), 0u, (_e4303 < 0f)), 4294967295u, (_e4303 > 4294967000f)) + _e4228);
                                                                    let _e4312 = global_6.member[(_e4151 + _e4270)];
                                                                    let _e4318 = (select(select(u32(_e4312), 0u, (_e4312 < 0f)), 4294967295u, (_e4312 > 4294967000f)) % 12u);
                                                                    let _e4321 = select(-1f, 1f, ((_e4318 & 1u) == 0u));
                                                                    let _e4324 = select(-1f, 1f, ((_e4318 & 2u) == 0u));
                                                                    if (_e4318 < 4u) {
                                                                        phi_22506_ = ((_e4321 * _e4229) + (_e4324 * _e4230));
                                                                    } else {
                                                                        if (_e4318 < 8u) {
                                                                            phi_22500_ = ((_e4321 * _e4229) + (_e4324 * _e4231));
                                                                        } else {
                                                                            phi_22500_ = ((_e4321 * _e4230) + (_e4324 * _e4231));
                                                                        }
                                                                        let _e4334 = phi_22500_;
                                                                        phi_22506_ = _e4334;
                                                                    }
                                                                    let _e4339 = phi_22506_;
                                                                    let _e4342 = global_6.member[(_e4151 + _e4299)];
                                                                    let _e4348 = (_e4229 - 1f);
                                                                    let _e4349 = (select(select(u32(_e4342), 0u, (_e4342 < 0f)), 4294967295u, (_e4342 > 4294967000f)) % 12u);
                                                                    let _e4352 = select(-1f, 1f, ((_e4349 & 1u) == 0u));
                                                                    let _e4355 = select(-1f, 1f, ((_e4349 & 2u) == 0u));
                                                                    if (_e4349 < 4u) {
                                                                        phi_22540_ = ((_e4352 * _e4348) + (_e4355 * _e4230));
                                                                    } else {
                                                                        if (_e4349 < 8u) {
                                                                            phi_22534_ = ((_e4352 * _e4348) + (_e4355 * _e4231));
                                                                        } else {
                                                                            phi_22534_ = ((_e4352 * _e4230) + (_e4355 * _e4231));
                                                                        }
                                                                        let _e4365 = phi_22534_;
                                                                        phi_22540_ = _e4365;
                                                                    }
                                                                    let _e4370 = phi_22540_;
                                                                    let _e4373 = global_6.member[(_e4151 + _e4280)];
                                                                    let _e4379 = (_e4230 - 1f);
                                                                    let _e4380 = (select(select(u32(_e4373), 0u, (_e4373 < 0f)), 4294967295u, (_e4373 > 4294967000f)) % 12u);
                                                                    let _e4383 = select(-1f, 1f, ((_e4380 & 1u) == 0u));
                                                                    let _e4386 = select(-1f, 1f, ((_e4380 & 2u) == 0u));
                                                                    if (_e4380 < 4u) {
                                                                        phi_22574_ = ((_e4383 * _e4229) + (_e4386 * _e4379));
                                                                    } else {
                                                                        if (_e4380 < 8u) {
                                                                            phi_22568_ = ((_e4383 * _e4229) + (_e4386 * _e4231));
                                                                        } else {
                                                                            phi_22568_ = ((_e4383 * _e4379) + (_e4386 * _e4231));
                                                                        }
                                                                        let _e4396 = phi_22568_;
                                                                        phi_22574_ = _e4396;
                                                                    }
                                                                    let _e4401 = phi_22574_;
                                                                    let _e4404 = global_6.member[(_e4151 + _e4309)];
                                                                    let _e4410 = (select(select(u32(_e4404), 0u, (_e4404 < 0f)), 4294967295u, (_e4404 > 4294967000f)) % 12u);
                                                                    let _e4413 = select(-1f, 1f, ((_e4410 & 1u) == 0u));
                                                                    let _e4416 = select(-1f, 1f, ((_e4410 & 2u) == 0u));
                                                                    if (_e4410 < 4u) {
                                                                        phi_22608_ = ((_e4413 * _e4348) + (_e4416 * _e4379));
                                                                    } else {
                                                                        if (_e4410 < 8u) {
                                                                            phi_22602_ = ((_e4413 * _e4348) + (_e4416 * _e4231));
                                                                        } else {
                                                                            phi_22602_ = ((_e4413 * _e4379) + (_e4416 * _e4231));
                                                                        }
                                                                        let _e4426 = phi_22602_;
                                                                        phi_22608_ = _e4426;
                                                                    }
                                                                    let _e4431 = phi_22608_;
                                                                    let _e4435 = global_6.member[(_e4151 + (_e4270 + 1u))];
                                                                    let _e4441 = (_e4231 - 1f);
                                                                    let _e4442 = (select(select(u32(_e4435), 0u, (_e4435 < 0f)), 4294967295u, (_e4435 > 4294967000f)) % 12u);
                                                                    let _e4445 = select(-1f, 1f, ((_e4442 & 1u) == 0u));
                                                                    let _e4448 = select(-1f, 1f, ((_e4442 & 2u) == 0u));
                                                                    if (_e4442 < 4u) {
                                                                        phi_22642_ = ((_e4445 * _e4229) + (_e4448 * _e4230));
                                                                    } else {
                                                                        if (_e4442 < 8u) {
                                                                            phi_22636_ = ((_e4445 * _e4229) + (_e4448 * _e4441));
                                                                        } else {
                                                                            phi_22636_ = ((_e4445 * _e4230) + (_e4448 * _e4441));
                                                                        }
                                                                        let _e4458 = phi_22636_;
                                                                        phi_22642_ = _e4458;
                                                                    }
                                                                    let _e4463 = phi_22642_;
                                                                    let _e4467 = global_6.member[(_e4151 + (_e4299 + 1u))];
                                                                    let _e4473 = (select(select(u32(_e4467), 0u, (_e4467 < 0f)), 4294967295u, (_e4467 > 4294967000f)) % 12u);
                                                                    let _e4476 = select(-1f, 1f, ((_e4473 & 1u) == 0u));
                                                                    let _e4479 = select(-1f, 1f, ((_e4473 & 2u) == 0u));
                                                                    if (_e4473 < 4u) {
                                                                        phi_22676_ = ((_e4476 * _e4348) + (_e4479 * _e4230));
                                                                    } else {
                                                                        if (_e4473 < 8u) {
                                                                            phi_22670_ = ((_e4476 * _e4348) + (_e4479 * _e4441));
                                                                        } else {
                                                                            phi_22670_ = ((_e4476 * _e4230) + (_e4479 * _e4441));
                                                                        }
                                                                        let _e4489 = phi_22670_;
                                                                        phi_22676_ = _e4489;
                                                                    }
                                                                    let _e4494 = phi_22676_;
                                                                    let _e4498 = global_6.member[(_e4151 + (_e4280 + 1u))];
                                                                    let _e4504 = (select(select(u32(_e4498), 0u, (_e4498 < 0f)), 4294967295u, (_e4498 > 4294967000f)) % 12u);
                                                                    let _e4507 = select(-1f, 1f, ((_e4504 & 1u) == 0u));
                                                                    let _e4510 = select(-1f, 1f, ((_e4504 & 2u) == 0u));
                                                                    if (_e4504 < 4u) {
                                                                        phi_22710_ = ((_e4507 * _e4229) + (_e4510 * _e4379));
                                                                    } else {
                                                                        if (_e4504 < 8u) {
                                                                            phi_22704_ = ((_e4507 * _e4229) + (_e4510 * _e4441));
                                                                        } else {
                                                                            phi_22704_ = ((_e4507 * _e4379) + (_e4510 * _e4441));
                                                                        }
                                                                        let _e4520 = phi_22704_;
                                                                        phi_22710_ = _e4520;
                                                                    }
                                                                    let _e4525 = phi_22710_;
                                                                    let _e4529 = global_6.member[(_e4151 + (_e4309 + 1u))];
                                                                    let _e4535 = (select(select(u32(_e4529), 0u, (_e4529 < 0f)), 4294967295u, (_e4529 > 4294967000f)) % 12u);
                                                                    let _e4538 = select(-1f, 1f, ((_e4535 & 1u) == 0u));
                                                                    let _e4541 = select(-1f, 1f, ((_e4535 & 2u) == 0u));
                                                                    if (_e4535 < 4u) {
                                                                        phi_22744_ = ((_e4538 * _e4348) + (_e4541 * _e4379));
                                                                    } else {
                                                                        if (_e4535 < 8u) {
                                                                            phi_22738_ = ((_e4538 * _e4348) + (_e4541 * _e4441));
                                                                        } else {
                                                                            phi_22738_ = ((_e4538 * _e4379) + (_e4541 * _e4441));
                                                                        }
                                                                        let _e4551 = phi_22738_;
                                                                        phi_22744_ = _e4551;
                                                                    }
                                                                    let _e4556 = phi_22744_;
                                                                    let _e4559 = (_e4339 + (_e4238 * (_e4370 - _e4339)));
                                                                    let _e4565 = (_e4463 + (_e4238 * (_e4494 - _e4463)));
                                                                    let _e4571 = (_e4559 + (_e4245 * ((_e4401 + (_e4238 * (_e4431 - _e4401))) - _e4559)));
                                                                    phi_6832_ = (_e4180 + 1u);
                                                                    phi_6835_ = (_e4182 + _e4184);
                                                                    phi_6837_ = (_e4184 * 0.5f);
                                                                    phi_6839_ = (_e4186 + ((_e4571 + ((((_e4231 * _e4231) * _e4231) * ((_e4231 * ((_e4231 * 6f) - 15f)) + 10f)) * ((_e4565 + (_e4245 * ((_e4525 + (_e4238 * (_e4556 - _e4525))) - _e4565))) - _e4571))) * _e4184));
                                                                    phi_6841_ = vec3<f32>(((((_e4154 * _e4188.x) + (_e4157 * _e4188.y)) + (_e4160 * _e4188.z)) * 2.03f), ((((_e4163 * _e4188.x) + (_e4166 * _e4188.y)) + (_e4169 * _e4188.z)) * 2.03f), ((((_e4172 * _e4188.x) + (_e4175 * _e4188.y)) + (_e4178 * _e4188.z)) * 2.03f));
                                                                } else {
                                                                    phi_6832_ = u32();
                                                                    phi_6835_ = f32();
                                                                    phi_6837_ = f32();
                                                                    phi_6839_ = f32();
                                                                    phi_6841_ = vec3<f32>();
                                                                }
                                                                let _e4603 = phi_6832_;
                                                                let _e4605 = phi_6835_;
                                                                let _e4607 = phi_6837_;
                                                                let _e4609 = phi_6839_;
                                                                let _e4611 = phi_6841_;
                                                                phi_6831_ = _e4603;
                                                                phi_6834_ = _e4605;
                                                                phi_6836_ = _e4607;
                                                                phi_6838_ = _e4609;
                                                                phi_6840_ = _e4611;
                                                                if !(_e4189) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e4614 = local_22;
                                                            if (_e4614 > 0f) {
                                                                let _e4617 = local_23;
                                                                let _e4619 = local_24;
                                                                phi_7257_ = (_e4617 / _e4619);
                                                            } else {
                                                                phi_7257_ = 0f;
                                                            }
                                                            let _e4622 = phi_7257_;
                                                            if (_e4078 == 0u) {
                                                                phi_7340_ = _e4074;
                                                                phi_7341_ = _e4076;
                                                                phi_7342_ = _e4080;
                                                                phi_7343_ = _e4082;
                                                                phi_7344_ = _e4084;
                                                                phi_7345_ = (_e4086 + (1.5f * _e4622));
                                                            } else {
                                                                if (_e4078 == 1u) {
                                                                    phi_7334_ = _e4074;
                                                                    phi_7335_ = _e4076;
                                                                    phi_7336_ = _e4080;
                                                                    phi_7337_ = _e4082;
                                                                    phi_7338_ = _e4084;
                                                                    phi_7339_ = (_e4086 + (0.8f * _e4622));
                                                                } else {
                                                                    let _e4625 = (_e4078 == 2u);
                                                                    if _e4625 {
                                                                        phi_7329_ = _e4074;
                                                                        phi_7330_ = _e4076;
                                                                        phi_7331_ = _e4080;
                                                                        phi_7332_ = _e4082;
                                                                    } else {
                                                                        if (_e4078 == 3u) {
                                                                            phi_7325_ = _e4074;
                                                                            phi_7326_ = _e4076;
                                                                            phi_7327_ = _e4080;
                                                                            phi_7328_ = vec3<f32>(_e4622, _e4082.y, _e4082.z);
                                                                        } else {
                                                                            if (_e4078 == 4u) {
                                                                                phi_7321_ = _e4074;
                                                                                phi_7322_ = _e4076;
                                                                                phi_7323_ = _e4080;
                                                                                phi_7324_ = vec3<f32>(_e4082.x, _e4622, _e4082.z);
                                                                            } else {
                                                                                if (_e4078 == 5u) {
                                                                                    phi_7317_ = _e4074;
                                                                                    phi_7318_ = _e4076;
                                                                                    phi_7319_ = _e4080;
                                                                                    phi_7320_ = vec3<f32>(_e4082.x, _e4082.y, _e4622);
                                                                                } else {
                                                                                    let _e4629 = (_e4078 == 6u);
                                                                                    if _e4629 {
                                                                                        phi_7314_ = _e4074;
                                                                                        phi_7316_ = _e4080;
                                                                                    } else {
                                                                                        if (_e4078 == 7u) {
                                                                                            phi_7312_ = _e4074;
                                                                                            phi_7313_ = vec3<f32>(_e4622, _e4080.y, _e4080.z);
                                                                                        } else {
                                                                                            if (_e4078 == 8u) {
                                                                                                phi_7310_ = _e4074;
                                                                                                phi_7311_ = vec3<f32>(_e4080.x, _e4622, _e4080.z);
                                                                                            } else {
                                                                                                let _e4632 = (_e4078 == 9u);
                                                                                                if _e4632 {
                                                                                                    phi_7309_ = vec3<f32>(_e4080.x, _e4080.y, _e4622);
                                                                                                } else {
                                                                                                    phi_7309_ = _e4080;
                                                                                                }
                                                                                                let _e4638 = phi_7309_;
                                                                                                phi_7310_ = select(_e4622, _e4074, _e4632);
                                                                                                phi_7311_ = _e4638;
                                                                                            }
                                                                                            let _e4645 = phi_7310_;
                                                                                            let _e4647 = phi_7311_;
                                                                                            phi_7312_ = _e4645;
                                                                                            phi_7313_ = _e4647;
                                                                                        }
                                                                                        let _e4653 = phi_7312_;
                                                                                        let _e4655 = phi_7313_;
                                                                                        phi_7314_ = _e4653;
                                                                                        phi_7316_ = _e4655;
                                                                                    }
                                                                                    let _e4657 = phi_7314_;
                                                                                    let _e4659 = phi_7316_;
                                                                                    phi_7317_ = _e4657;
                                                                                    phi_7318_ = select(_e4076, _e4622, _e4629);
                                                                                    phi_7319_ = _e4659;
                                                                                    phi_7320_ = _e4082;
                                                                                }
                                                                                let _e4666 = phi_7317_;
                                                                                let _e4668 = phi_7318_;
                                                                                let _e4670 = phi_7319_;
                                                                                let _e4672 = phi_7320_;
                                                                                phi_7321_ = _e4666;
                                                                                phi_7322_ = _e4668;
                                                                                phi_7323_ = _e4670;
                                                                                phi_7324_ = _e4672;
                                                                            }
                                                                            let _e4678 = phi_7321_;
                                                                            let _e4680 = phi_7322_;
                                                                            let _e4682 = phi_7323_;
                                                                            let _e4684 = phi_7324_;
                                                                            phi_7325_ = _e4678;
                                                                            phi_7326_ = _e4680;
                                                                            phi_7327_ = _e4682;
                                                                            phi_7328_ = _e4684;
                                                                        }
                                                                        let _e4690 = phi_7325_;
                                                                        let _e4692 = phi_7326_;
                                                                        let _e4694 = phi_7327_;
                                                                        let _e4696 = phi_7328_;
                                                                        phi_7329_ = _e4690;
                                                                        phi_7330_ = _e4692;
                                                                        phi_7331_ = _e4694;
                                                                        phi_7332_ = _e4696;
                                                                    }
                                                                    let _e4698 = phi_7329_;
                                                                    let _e4700 = phi_7330_;
                                                                    let _e4702 = phi_7331_;
                                                                    let _e4704 = phi_7332_;
                                                                    phi_7334_ = _e4698;
                                                                    phi_7335_ = _e4700;
                                                                    phi_7336_ = _e4702;
                                                                    phi_7337_ = _e4704;
                                                                    phi_7338_ = select(_e4084, _e4622, _e4625);
                                                                    phi_7339_ = _e4086;
                                                                }
                                                                let _e4709 = phi_7334_;
                                                                let _e4711 = phi_7335_;
                                                                let _e4713 = phi_7336_;
                                                                let _e4715 = phi_7337_;
                                                                let _e4717 = phi_7338_;
                                                                let _e4719 = phi_7339_;
                                                                phi_7340_ = _e4709;
                                                                phi_7341_ = _e4711;
                                                                phi_7342_ = _e4713;
                                                                phi_7343_ = _e4715;
                                                                phi_7344_ = _e4717;
                                                                phi_7345_ = _e4719;
                                                            }
                                                            let _e4723 = phi_7340_;
                                                            let _e4725 = phi_7341_;
                                                            let _e4727 = phi_7342_;
                                                            let _e4729 = phi_7343_;
                                                            let _e4731 = phi_7344_;
                                                            let _e4733 = phi_7345_;
                                                            phi_6604_ = _e4723;
                                                            phi_6607_ = _e4725;
                                                            phi_6609_ = (_e4078 + 1u);
                                                            phi_6611_ = _e4727;
                                                            phi_6613_ = _e4729;
                                                            phi_6615_ = _e4731;
                                                            phi_6617_ = _e4733;
                                                        } else {
                                                            phi_6604_ = f32();
                                                            phi_6607_ = f32();
                                                            phi_6609_ = u32();
                                                            phi_6611_ = vec3<f32>();
                                                            phi_6613_ = vec3<f32>();
                                                            phi_6615_ = f32();
                                                            phi_6617_ = f32();
                                                        }
                                                        let _e4736 = phi_6604_;
                                                        let _e4738 = phi_6607_;
                                                        let _e4740 = phi_6609_;
                                                        let _e4742 = phi_6611_;
                                                        let _e4744 = phi_6613_;
                                                        let _e4746 = phi_6615_;
                                                        let _e4748 = phi_6617_;
                                                        phi_6603_ = _e4736;
                                                        phi_6606_ = _e4738;
                                                        phi_6608_ = _e4740;
                                                        phi_6610_ = _e4742;
                                                        phi_6612_ = _e4744;
                                                        phi_6614_ = _e4746;
                                                        phi_6616_ = _e4748;
                                                        if !(_e4087) {
                                                            break;
                                                        }
                                                    }
                                                    if _e4051 {
                                                        let _e4763 = (exp((-(abs(_e3778)) / (0.06f + (abs(_e3789) * 0.015915494f)))) * (0.3f + (0.7f * exp(((_e3789 * -0.71428573f) * (_e3789 * 0.71428573f))))));
                                                        let _e4765 = (_e3778 * 1.3333334f);
                                                        let _e4770 = (1.3f * exp((((_e3789 * _e3789) + (_e4765 * _e4765)) * -12.499999f)));
                                                        let _e4781 = local_25;
                                                        let _e4783 = local_26;
                                                        if (_e4781 != _e4783) {
                                                            phi_22820_ = true;
                                                        } else {
                                                            let _e4786 = local_27;
                                                            phi_22820_ = (0.1f >= _e4786);
                                                        }
                                                        let _e4789 = phi_22820_;
                                                        let _e4791 = local_28;
                                                        let _e4794 = (pow(select(_e4791, 0.1f, _e4789), 1.3f) * _e4009);
                                                        phi_7377_ = vec3<f32>((_e4006.x + (((_e4015 * _e4763) + (_e4024 * _e4770)) * _e4794)), (_e4006.y + (((_e4018 * _e4763) + (_e4027 * _e4770)) * _e4794)), (_e4006.z + (((_e4021 * _e4763) + (_e4030 * _e4770)) * _e4794)));
                                                    } else {
                                                        phi_7377_ = _e4006;
                                                    }
                                                    let _e4806 = phi_7377_;
                                                    if select(_e4061, _e4065, _e4067) {
                                                        let _e4808 = local_29;
                                                        let _e4814 = ((_e3778 - (0.025f * _e4808)) / (0.045f + (abs(_e3789) * 0.0095492955f)));
                                                        let _e4819 = local_30;
                                                        let _e4820 = (_e4819 * 3.3333333f);
                                                        let _e4822 = select(_e4820, 0f, (_e4820 < 0f));
                                                        let _e4824 = select(_e4822, 1f, (_e4822 > 1f));
                                                        let _e4840 = local_31;
                                                        let _e4842 = ((_e4840 - 0.25f) * 5.0000005f);
                                                        let _e4844 = select(_e4842, 0f, (_e4842 < 0f));
                                                        let _e4846 = select(_e4844, 1f, (_e4844 > 1f));
                                                        phi_7456_ = (_e4012 * (((exp((-(_e4814) * _e4814)) * (0.15f + (3f * ((_e4824 * _e4824) * (3f - (2f * _e4824)))))) * (0.5f + (0.9f * exp(((_e3789 * -0.6666667f) * (_e3789 * 0.6666667f)))))) + (1.2f * (exp(((_e3778 * -3.3333333f) * (_e3778 * 3.3333333f))) * ((_e4846 * _e4846) * (3f - (2f * _e4846)))))));
                                                    } else {
                                                        phi_7456_ = 0f;
                                                    }
                                                    let _e4860 = phi_7456_;
                                                    phi_7457_ = _e4806;
                                                    phi_7458_ = _e4860;
                                                } else {
                                                    phi_7457_ = _e4006;
                                                    phi_7458_ = 0f;
                                                }
                                                let _e4862 = phi_7457_;
                                                let _e4864 = phi_7458_;
                                                phi_7460_ = _e4862;
                                                phi_7463_ = 0u;
                                                loop {
                                                    let _e4866 = phi_7460_;
                                                    let _e4868 = phi_7463_;
                                                    local_48 = _e4866;
                                                    local_49 = _e4866;
                                                    local_50 = _e4866;
                                                    let _e4869 = (_e4868 < bitcast<u32>(_e3738));
                                                    if _e4869 {
                                                        let _e4871 = (bitcast<u32>(_e3734) + (_e4868 * 29u));
                                                        let _e4873 = global_6.member[_e4871];
                                                        let _e4874 = (_e4873 > 0.5f);
                                                        let _e4877 = global_6.member[(_e4871 + 1u)];
                                                        let _e4880 = global_6.member[(_e4871 + 2u)];
                                                        let _e4883 = global_6.member[(_e4871 + 3u)];
                                                        let _e4886 = global_6.member[(_e4871 + 4u)];
                                                        let _e4889 = global_6.member[(_e4871 + 5u)];
                                                        let _e4892 = global_6.member[(_e4871 + 6u)];
                                                        let _e4897 = (((_e1173.x * _e4877) + (_e1173.y * _e4880)) + (_e1173.z * _e4883));
                                                        let _e4898 = (_e4897 < _e4889);
                                                        if _e4898 {
                                                            phi_8615_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_8616_ = vec3<f32>();
                                                        } else {
                                                            let _e4901 = global_6.member[(_e4871 + 7u)];
                                                            let _e4904 = global_6.member[(_e4871 + 8u)];
                                                            let _e4907 = global_6.member[(_e4871 + 9u)];
                                                            let _e4910 = global_6.member[(_e4871 + 10u)];
                                                            let _e4913 = global_6.member[(_e4871 + 11u)];
                                                            let _e4916 = global_6.member[(_e4871 + 12u)];
                                                            let _e4919 = global_6.member[(_e4871 + 13u)];
                                                            let _e4922 = global_6.member[(_e4871 + 14u)];
                                                            let _e4925 = global_6.member[(_e4871 + 15u)];
                                                            let _e4928 = global_6.member[(_e4871 + 16u)];
                                                            let _e4931 = global_6.member[(_e4871 + 17u)];
                                                            let _e4934 = global_6.member[(_e4871 + 18u)];
                                                            let _e4937 = global_6.member[(_e4871 + 19u)];
                                                            let _e4940 = global_6.member[(_e4871 + 20u)];
                                                            let _e4943 = global_6.member[(_e4871 + 21u)];
                                                            let _e4946 = global_6.member[(_e4871 + 22u)];
                                                            let _e4949 = global_6.member[(_e4871 + 23u)];
                                                            let _e4952 = global_6.member[(_e4871 + 24u)];
                                                            let _e4955 = global_6.member[(_e4871 + 25u)];
                                                            let _e4958 = global_6.member[(_e4871 + 26u)];
                                                            let _e4961 = global_6.member[(_e4871 + 27u)];
                                                            let _e4964 = global_6.member[(_e4871 + 28u)];
                                                            let _e4971 = (((_e1173.x / _e4897) - _e4877) / _e4886);
                                                            let _e4972 = (((_e1173.y / _e4897) - _e4880) / _e4886);
                                                            let _e4973 = (((_e1173.z / _e4897) - _e4883) / _e4886);
                                                            if _e4874 {
                                                                let _e5578 = (((_e4971 * _e4928) + (_e4972 * _e4931)) + (_e4973 * _e4934));
                                                                let _e5584 = ((((_e4971 * _e4937) + (_e4972 * _e4940)) + (_e4973 * _e4943)) / _e4946);
                                                                phi_8613_ = (vec3<f32>(1.1633643f, 0.97722596f, 0.74455315f) * (_e4892 * exp((-2.5f * pow((sqrt(((_e5578 * _e5578) + (_e5584 * _e5584))) * 3.3333333f), 0.7f)))));
                                                                phi_8614_ = vec3<f32>();
                                                            } else {
                                                                let _e4978 = (((_e4901 * _e4877) + (_e4904 * _e4880)) + (_e4907 * _e4883));
                                                                if (abs(_e4978) < 0.12f) {
                                                                    phi_7917_ = select(0.12f, -0.12f, (_e4978 < 0f));
                                                                } else {
                                                                    phi_7917_ = _e4978;
                                                                }
                                                                let _e4984 = phi_7917_;
                                                                let _e4990 = ((((_e4901 * _e4971) + (_e4904 * _e4972)) + (_e4907 * _e4973)) / _e4984);
                                                                let _e4994 = (_e4971 - (_e4877 * _e4990));
                                                                let _e4995 = (_e4972 - (_e4880 * _e4990));
                                                                let _e4996 = (_e4973 - (_e4883 * _e4990));
                                                                let _e5001 = (((_e4994 * _e4910) + (_e4995 * _e4913)) + (_e4996 * _e4916));
                                                                let _e5006 = (((_e4994 * _e4919) + (_e4995 * _e4922)) + (_e4996 * _e4925));
                                                                let _e5010 = sqrt(((_e5001 * _e5001) + (_e5006 * _e5006)));
                                                                let _e5022 = abs(_e4984);
                                                                let _e5024 = (pow((0.5f + (0.5f * cos(((_e4949 * (atan2(_e5006, _e5001) - (log((_e5010 + 0.03f)) / _e4952))) + _e4955)))), 3f) * sqrt(_e5022));
                                                                let _e5029 = global_6.member[_e3715];
                                                                let _e5030 = bitcast<u32>(_e5029);
                                                                let _e5033 = global_6.member[(_e1189 + 11u)];
                                                                let _e5036 = global_6.member[(_e1189 + 12u)];
                                                                let _e5039 = global_6.member[(_e1189 + 13u)];
                                                                let _e5042 = global_6.member[(_e1189 + 14u)];
                                                                let _e5045 = global_6.member[(_e1189 + 15u)];
                                                                let _e5048 = global_6.member[(_e1189 + 16u)];
                                                                let _e5051 = global_6.member[(_e1189 + 17u)];
                                                                let _e5054 = global_6.member[(_e1189 + 18u)];
                                                                let _e5057 = global_6.member[(_e1189 + 19u)];
                                                                phi_8097_ = 0u;
                                                                phi_8100_ = 0f;
                                                                phi_8102_ = 1f;
                                                                phi_8104_ = 0f;
                                                                phi_8106_ = vec3<f32>((_e5001 * 9f), (_e5006 * 9f), 0f);
                                                                loop {
                                                                    let _e5059 = phi_8097_;
                                                                    let _e5061 = phi_8100_;
                                                                    let _e5063 = phi_8102_;
                                                                    let _e5065 = phi_8104_;
                                                                    let _e5067 = phi_8106_;
                                                                    local_32 = _e5061;
                                                                    local_33 = _e5065;
                                                                    local_34 = _e5061;
                                                                    let _e5068 = (_e5059 < 4u);
                                                                    if _e5068 {
                                                                        let _e5070 = (_e5067.x + _e4958);
                                                                        let _e5072 = (_e5067.y + _e4961);
                                                                        let _e5074 = (_e5067.z + _e4964);
                                                                        let _e5075 = floor(_e5070);
                                                                        let _e5076 = floor(_e5072);
                                                                        let _e5077 = floor(_e5074);
                                                                        let _e5086 = bitcast<u32>((select(0i, select(select(i32(_e5075), i32(-2147483648), (_e5075 < -2147483600f)), 2147483647i, (_e5075 > 2147483500f)), (_e5075 == _e5075)) & 255i));
                                                                        let _e5095 = bitcast<u32>((select(0i, select(select(i32(_e5076), i32(-2147483648), (_e5076 < -2147483600f)), 2147483647i, (_e5076 > 2147483500f)), (_e5076 == _e5076)) & 255i));
                                                                        let _e5104 = bitcast<u32>((select(0i, select(select(i32(_e5077), i32(-2147483648), (_e5077 < -2147483600f)), 2147483647i, (_e5077 > 2147483500f)), (_e5077 == _e5077)) & 255i));
                                                                        let _e5105 = (_e5070 - _e5075);
                                                                        let _e5106 = (_e5072 - _e5076);
                                                                        let _e5107 = (_e5074 - _e5077);
                                                                        let _e5114 = (((_e5105 * _e5105) * _e5105) * ((_e5105 * ((_e5105 * 6f) - 15f)) + 10f));
                                                                        let _e5121 = (((_e5106 * _e5106) * _e5106) * ((_e5106 * ((_e5106 * 6f) - 15f)) + 10f));
                                                                        let _e5131 = global_6.member[(_e5030 + _e5086)];
                                                                        let _e5137 = (select(select(u32(_e5131), 0u, (_e5131 < 0f)), 4294967295u, (_e5131 > 4294967000f)) + _e5095);
                                                                        let _e5140 = global_6.member[(_e5030 + _e5137)];
                                                                        let _e5146 = (select(select(u32(_e5140), 0u, (_e5140 < 0f)), 4294967295u, (_e5140 > 4294967000f)) + _e5104);
                                                                        let _e5150 = global_6.member[(_e5030 + (_e5137 + 1u))];
                                                                        let _e5156 = (select(select(u32(_e5150), 0u, (_e5150 < 0f)), 4294967295u, (_e5150 > 4294967000f)) + _e5104);
                                                                        let _e5160 = global_6.member[(_e5030 + (_e5086 + 1u))];
                                                                        let _e5166 = (select(select(u32(_e5160), 0u, (_e5160 < 0f)), 4294967295u, (_e5160 > 4294967000f)) + _e5095);
                                                                        let _e5169 = global_6.member[(_e5030 + _e5166)];
                                                                        let _e5175 = (select(select(u32(_e5169), 0u, (_e5169 < 0f)), 4294967295u, (_e5169 > 4294967000f)) + _e5104);
                                                                        let _e5179 = global_6.member[(_e5030 + (_e5166 + 1u))];
                                                                        let _e5185 = (select(select(u32(_e5179), 0u, (_e5179 < 0f)), 4294967295u, (_e5179 > 4294967000f)) + _e5104);
                                                                        let _e5188 = global_6.member[(_e5030 + _e5146)];
                                                                        let _e5194 = (select(select(u32(_e5188), 0u, (_e5188 < 0f)), 4294967295u, (_e5188 > 4294967000f)) % 12u);
                                                                        let _e5197 = select(-1f, 1f, ((_e5194 & 1u) == 0u));
                                                                        let _e5200 = select(-1f, 1f, ((_e5194 & 2u) == 0u));
                                                                        if (_e5194 < 4u) {
                                                                            phi_22915_ = ((_e5197 * _e5105) + (_e5200 * _e5106));
                                                                        } else {
                                                                            if (_e5194 < 8u) {
                                                                                phi_22909_ = ((_e5197 * _e5105) + (_e5200 * _e5107));
                                                                            } else {
                                                                                phi_22909_ = ((_e5197 * _e5106) + (_e5200 * _e5107));
                                                                            }
                                                                            let _e5210 = phi_22909_;
                                                                            phi_22915_ = _e5210;
                                                                        }
                                                                        let _e5215 = phi_22915_;
                                                                        let _e5218 = global_6.member[(_e5030 + _e5175)];
                                                                        let _e5224 = (_e5105 - 1f);
                                                                        let _e5225 = (select(select(u32(_e5218), 0u, (_e5218 < 0f)), 4294967295u, (_e5218 > 4294967000f)) % 12u);
                                                                        let _e5228 = select(-1f, 1f, ((_e5225 & 1u) == 0u));
                                                                        let _e5231 = select(-1f, 1f, ((_e5225 & 2u) == 0u));
                                                                        if (_e5225 < 4u) {
                                                                            phi_22949_ = ((_e5228 * _e5224) + (_e5231 * _e5106));
                                                                        } else {
                                                                            if (_e5225 < 8u) {
                                                                                phi_22943_ = ((_e5228 * _e5224) + (_e5231 * _e5107));
                                                                            } else {
                                                                                phi_22943_ = ((_e5228 * _e5106) + (_e5231 * _e5107));
                                                                            }
                                                                            let _e5241 = phi_22943_;
                                                                            phi_22949_ = _e5241;
                                                                        }
                                                                        let _e5246 = phi_22949_;
                                                                        let _e5249 = global_6.member[(_e5030 + _e5156)];
                                                                        let _e5255 = (_e5106 - 1f);
                                                                        let _e5256 = (select(select(u32(_e5249), 0u, (_e5249 < 0f)), 4294967295u, (_e5249 > 4294967000f)) % 12u);
                                                                        let _e5259 = select(-1f, 1f, ((_e5256 & 1u) == 0u));
                                                                        let _e5262 = select(-1f, 1f, ((_e5256 & 2u) == 0u));
                                                                        if (_e5256 < 4u) {
                                                                            phi_22983_ = ((_e5259 * _e5105) + (_e5262 * _e5255));
                                                                        } else {
                                                                            if (_e5256 < 8u) {
                                                                                phi_22977_ = ((_e5259 * _e5105) + (_e5262 * _e5107));
                                                                            } else {
                                                                                phi_22977_ = ((_e5259 * _e5255) + (_e5262 * _e5107));
                                                                            }
                                                                            let _e5272 = phi_22977_;
                                                                            phi_22983_ = _e5272;
                                                                        }
                                                                        let _e5277 = phi_22983_;
                                                                        let _e5280 = global_6.member[(_e5030 + _e5185)];
                                                                        let _e5286 = (select(select(u32(_e5280), 0u, (_e5280 < 0f)), 4294967295u, (_e5280 > 4294967000f)) % 12u);
                                                                        let _e5289 = select(-1f, 1f, ((_e5286 & 1u) == 0u));
                                                                        let _e5292 = select(-1f, 1f, ((_e5286 & 2u) == 0u));
                                                                        if (_e5286 < 4u) {
                                                                            phi_23017_ = ((_e5289 * _e5224) + (_e5292 * _e5255));
                                                                        } else {
                                                                            if (_e5286 < 8u) {
                                                                                phi_23011_ = ((_e5289 * _e5224) + (_e5292 * _e5107));
                                                                            } else {
                                                                                phi_23011_ = ((_e5289 * _e5255) + (_e5292 * _e5107));
                                                                            }
                                                                            let _e5302 = phi_23011_;
                                                                            phi_23017_ = _e5302;
                                                                        }
                                                                        let _e5307 = phi_23017_;
                                                                        let _e5311 = global_6.member[(_e5030 + (_e5146 + 1u))];
                                                                        let _e5317 = (_e5107 - 1f);
                                                                        let _e5318 = (select(select(u32(_e5311), 0u, (_e5311 < 0f)), 4294967295u, (_e5311 > 4294967000f)) % 12u);
                                                                        let _e5321 = select(-1f, 1f, ((_e5318 & 1u) == 0u));
                                                                        let _e5324 = select(-1f, 1f, ((_e5318 & 2u) == 0u));
                                                                        if (_e5318 < 4u) {
                                                                            phi_23051_ = ((_e5321 * _e5105) + (_e5324 * _e5106));
                                                                        } else {
                                                                            if (_e5318 < 8u) {
                                                                                phi_23045_ = ((_e5321 * _e5105) + (_e5324 * _e5317));
                                                                            } else {
                                                                                phi_23045_ = ((_e5321 * _e5106) + (_e5324 * _e5317));
                                                                            }
                                                                            let _e5334 = phi_23045_;
                                                                            phi_23051_ = _e5334;
                                                                        }
                                                                        let _e5339 = phi_23051_;
                                                                        let _e5343 = global_6.member[(_e5030 + (_e5175 + 1u))];
                                                                        let _e5349 = (select(select(u32(_e5343), 0u, (_e5343 < 0f)), 4294967295u, (_e5343 > 4294967000f)) % 12u);
                                                                        let _e5352 = select(-1f, 1f, ((_e5349 & 1u) == 0u));
                                                                        let _e5355 = select(-1f, 1f, ((_e5349 & 2u) == 0u));
                                                                        if (_e5349 < 4u) {
                                                                            phi_23085_ = ((_e5352 * _e5224) + (_e5355 * _e5106));
                                                                        } else {
                                                                            if (_e5349 < 8u) {
                                                                                phi_23079_ = ((_e5352 * _e5224) + (_e5355 * _e5317));
                                                                            } else {
                                                                                phi_23079_ = ((_e5352 * _e5106) + (_e5355 * _e5317));
                                                                            }
                                                                            let _e5365 = phi_23079_;
                                                                            phi_23085_ = _e5365;
                                                                        }
                                                                        let _e5370 = phi_23085_;
                                                                        let _e5374 = global_6.member[(_e5030 + (_e5156 + 1u))];
                                                                        let _e5380 = (select(select(u32(_e5374), 0u, (_e5374 < 0f)), 4294967295u, (_e5374 > 4294967000f)) % 12u);
                                                                        let _e5383 = select(-1f, 1f, ((_e5380 & 1u) == 0u));
                                                                        let _e5386 = select(-1f, 1f, ((_e5380 & 2u) == 0u));
                                                                        if (_e5380 < 4u) {
                                                                            phi_23119_ = ((_e5383 * _e5105) + (_e5386 * _e5255));
                                                                        } else {
                                                                            if (_e5380 < 8u) {
                                                                                phi_23113_ = ((_e5383 * _e5105) + (_e5386 * _e5317));
                                                                            } else {
                                                                                phi_23113_ = ((_e5383 * _e5255) + (_e5386 * _e5317));
                                                                            }
                                                                            let _e5396 = phi_23113_;
                                                                            phi_23119_ = _e5396;
                                                                        }
                                                                        let _e5401 = phi_23119_;
                                                                        let _e5405 = global_6.member[(_e5030 + (_e5185 + 1u))];
                                                                        let _e5411 = (select(select(u32(_e5405), 0u, (_e5405 < 0f)), 4294967295u, (_e5405 > 4294967000f)) % 12u);
                                                                        let _e5414 = select(-1f, 1f, ((_e5411 & 1u) == 0u));
                                                                        let _e5417 = select(-1f, 1f, ((_e5411 & 2u) == 0u));
                                                                        if (_e5411 < 4u) {
                                                                            phi_23153_ = ((_e5414 * _e5224) + (_e5417 * _e5255));
                                                                        } else {
                                                                            if (_e5411 < 8u) {
                                                                                phi_23147_ = ((_e5414 * _e5224) + (_e5417 * _e5317));
                                                                            } else {
                                                                                phi_23147_ = ((_e5414 * _e5255) + (_e5417 * _e5317));
                                                                            }
                                                                            let _e5427 = phi_23147_;
                                                                            phi_23153_ = _e5427;
                                                                        }
                                                                        let _e5432 = phi_23153_;
                                                                        let _e5435 = (_e5215 + (_e5114 * (_e5246 - _e5215)));
                                                                        let _e5441 = (_e5339 + (_e5114 * (_e5370 - _e5339)));
                                                                        let _e5447 = (_e5435 + (_e5121 * ((_e5277 + (_e5114 * (_e5307 - _e5277))) - _e5435)));
                                                                        phi_8098_ = (_e5059 + 1u);
                                                                        phi_8101_ = (_e5061 + _e5063);
                                                                        phi_8103_ = (_e5063 * 0.5f);
                                                                        phi_8105_ = (_e5065 + ((_e5447 + ((((_e5107 * _e5107) * _e5107) * ((_e5107 * ((_e5107 * 6f) - 15f)) + 10f)) * ((_e5441 + (_e5121 * ((_e5401 + (_e5114 * (_e5432 - _e5401))) - _e5441))) - _e5447))) * _e5063));
                                                                        phi_8107_ = vec3<f32>(((((_e5033 * _e5067.x) + (_e5036 * _e5067.y)) + (_e5039 * _e5067.z)) * 2.03f), ((((_e5042 * _e5067.x) + (_e5045 * _e5067.y)) + (_e5048 * _e5067.z)) * 2.03f), ((((_e5051 * _e5067.x) + (_e5054 * _e5067.y)) + (_e5057 * _e5067.z)) * 2.03f));
                                                                    } else {
                                                                        phi_8098_ = u32();
                                                                        phi_8101_ = f32();
                                                                        phi_8103_ = f32();
                                                                        phi_8105_ = f32();
                                                                        phi_8107_ = vec3<f32>();
                                                                    }
                                                                    let _e5479 = phi_8098_;
                                                                    let _e5481 = phi_8101_;
                                                                    let _e5483 = phi_8103_;
                                                                    let _e5485 = phi_8105_;
                                                                    let _e5487 = phi_8107_;
                                                                    phi_8097_ = _e5479;
                                                                    phi_8100_ = _e5481;
                                                                    phi_8102_ = _e5483;
                                                                    phi_8104_ = _e5485;
                                                                    phi_8106_ = _e5487;
                                                                    if !(_e5068) {
                                                                        break;
                                                                    }
                                                                }
                                                                let _e5490 = local_32;
                                                                if (_e5490 > 0f) {
                                                                    let _e5493 = local_33;
                                                                    let _e5495 = local_34;
                                                                    phi_8520_ = (_e5493 / _e5495);
                                                                } else {
                                                                    phi_8520_ = 0f;
                                                                }
                                                                let _e5498 = phi_8520_;
                                                                let _e5500 = (1f + (1.2f * _e5498));
                                                                if (_e5500 != _e5500) {
                                                                    phi_23199_ = true;
                                                                } else {
                                                                    phi_23199_ = (0f >= _e5500);
                                                                }
                                                                let _e5504 = phi_23199_;
                                                                let _e5509 = ((_e5010 - 1.15f) * -2.8571432f);
                                                                let _e5511 = select(_e5509, 0f, (_e5509 < 0f));
                                                                let _e5513 = select(_e5511, 1f, (_e5511 > 1f));
                                                                let _e5519 = (1f / _e5022);
                                                                if (_e5519 != _e5519) {
                                                                    phi_23214_ = true;
                                                                } else {
                                                                    phi_23214_ = (4f <= _e5519);
                                                                }
                                                                let _e5523 = phi_23214_;
                                                                let _e5526 = (((exp((_e5010 * -3.5714285f)) * ((_e5513 * _e5513) * (3f - (2f * _e5513)))) * select(_e5519, 4f, _e5523)) * 0.12f);
                                                                let _e5532 = sqrt((((_e4971 * _e4971) + (_e4972 * _e4972)) + (_e4973 * _e4973)));
                                                                let _e5539 = (((_e4971 * _e4937) + (_e4972 * _e4940)) + (_e4973 * _e4943));
                                                                let _e5540 = (1f - _e5022);
                                                                let _e5556 = ((_e5526 * (0.6f + (0.4f * (1f - _e5024)))) + exp((_e5532 * -16.666668f)));
                                                                let _e5562 = (((_e5526 * _e5024) * 1.8f) * select(_e5500, 0f, _e5504));
                                                                let _e5569 = (_e4892 * (1f - ((((0.75f * _e5540) * _e5540) * exp(((_e5539 * -28.571428f) * (_e5539 * 28.571428f)))) * exp((_e5532 * -1.4285715f)))));
                                                                phi_8613_ = vec3<f32>();
                                                                phi_8614_ = vec3<f32>((((1.1633643f * _e5556) + (0.86689025f * _e5562)) * _e5569), (((0.97722596f * _e5556) + (1.0155001f * _e5562)) * _e5569), (((0.74455315f * _e5556) + (1.2384146f * _e5562)) * _e5569));
                                                            }
                                                            let _e5596 = phi_8613_;
                                                            let _e5598 = phi_8614_;
                                                            phi_8615_ = _e5596;
                                                            phi_8616_ = _e5598;
                                                        }
                                                        let _e5600 = phi_8615_;
                                                        let _e5602 = phi_8616_;
                                                        phi_7461_ = (_e4866 + select(_e5602, _e5600, vec3(select(_e4874, true, _e4898))));
                                                        phi_7464_ = (_e4868 + 1u);
                                                    } else {
                                                        phi_7461_ = vec3<f32>();
                                                        phi_7464_ = u32();
                                                    }
                                                    let _e5609 = phi_7461_;
                                                    let _e5611 = phi_7464_;
                                                    phi_7460_ = _e5609;
                                                    phi_7463_ = _e5611;
                                                    if !(_e4869) {
                                                        break;
                                                    }
                                                }
                                                phi_30727_ = _e4004;
                                                if _e4004 {
                                                    break;
                                                }
                                                phi_8635_ = vec3<f32>(0f, 0f, 0f);
                                                phi_8638_ = 0u;
                                                phi_8640_ = _e4864;
                                                loop {
                                                    let _e5614 = phi_8635_;
                                                    let _e5616 = phi_8638_;
                                                    let _e5618 = phi_8640_;
                                                    local_44 = _e5618;
                                                    local_45 = _e5618;
                                                    local_46 = _e5618;
                                                    local_47 = _e5618;
                                                    local_51 = _e5614;
                                                    local_52 = _e5614;
                                                    local_53 = _e5614;
                                                    let _e5619 = (_e5616 < bitcast<u32>(_e3730));
                                                    if _e5619 {
                                                        let _e5621 = (bitcast<u32>(_e3726) + (_e5616 * 18u));
                                                        let _e5623 = global_6.member[_e5621];
                                                        let _e5628 = select(select(u32(_e5623), 0u, (_e5623 < 0f)), 4294967295u, (_e5623 > 4294967000f));
                                                        let _e5631 = global_6.member[(_e5621 + 1u)];
                                                        let _e5634 = global_6.member[(_e5621 + 2u)];
                                                        let _e5637 = global_6.member[(_e5621 + 3u)];
                                                        let _e5640 = global_6.member[(_e5621 + 4u)];
                                                        let _e5643 = global_6.member[(_e5621 + 5u)];
                                                        let _e5646 = global_6.member[(_e5621 + 6u)];
                                                        let _e5649 = global_6.member[(_e5621 + 7u)];
                                                        let _e5652 = global_6.member[(_e5621 + 8u)];
                                                        let _e5655 = global_6.member[(_e5621 + 9u)];
                                                        let _e5658 = global_6.member[(_e5621 + 10u)];
                                                        let _e5661 = global_6.member[(_e5621 + 11u)];
                                                        let _e5664 = global_6.member[(_e5621 + 12u)];
                                                        let _e5667 = global_6.member[(_e5621 + 13u)];
                                                        let _e5670 = global_6.member[(_e5621 + 14u)];
                                                        let _e5673 = global_6.member[(_e5621 + 15u)];
                                                        let _e5676 = global_6.member[(_e5621 + 16u)];
                                                        let _e5679 = global_6.member[(_e5621 + 17u)];
                                                        let _e5684 = (((_e1173.x * _e5631) + (_e1173.y * _e5634)) + (_e1173.z * _e5637));
                                                        if (_e5684 < _e5643) {
                                                            phi_9963_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                                        } else {
                                                            let _e5689 = ((_e1173.x / _e5684) - _e5631);
                                                            let _e5690 = ((_e1173.y / _e5684) - _e5634);
                                                            let _e5691 = ((_e1173.z / _e5684) - _e5637);
                                                            let _e5698 = ((((_e5689 * _e5652) + (_e5690 * _e5655)) + (_e5691 * _e5658)) / (_e5640 * _e5670));
                                                            let _e5704 = ((((_e5689 * _e5661) + (_e5690 * _e5664)) + (_e5691 * _e5667)) / _e5640);
                                                            let _e5708 = sqrt(((_e5698 * _e5698) + (_e5704 * _e5704)));
                                                            let _e5711 = vec3<f32>((_e5698 * 2.2f), (_e5704 * 2.2f), 0f);
                                                            let _e5712 = (_e5628 == 0u);
                                                            if _e5712 {
                                                                phi_8948_ = 6u;
                                                                phi_8949_ = 1.2f;
                                                                phi_8950_ = _e5711;
                                                            } else {
                                                                let _e5713 = (_e5628 == 1u);
                                                                if _e5713 {
                                                                    phi_8947_ = vec3<f32>((_e5698 * 1.54f), (_e5704 * 1.54f), 0f);
                                                                } else {
                                                                    phi_8947_ = _e5711;
                                                                }
                                                                let _e5718 = phi_8947_;
                                                                phi_8948_ = select(6u, 5u, _e5713);
                                                                phi_8949_ = select(1f, 0.8f, _e5713);
                                                                phi_8950_ = _e5718;
                                                            }
                                                            let _e5722 = phi_8948_;
                                                            let _e5724 = phi_8949_;
                                                            let _e5726 = phi_8950_;
                                                            if _e5712 {
                                                                phi_8967_ = vec3<f32>(((_e5698 * 3.7400002f) + -7f), ((_e5704 * 3.7400002f) + 2f), 5f);
                                                            } else {
                                                                phi_8967_ = vec3<f32>((_e5698 * 3.3000002f), (_e5704 * 3.3000002f), 0f);
                                                            }
                                                            let _e5736 = phi_8967_;
                                                            phi_8969_ = 0u;
                                                            phi_8972_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_8974_ = 0f;
                                                            phi_8976_ = 0f;
                                                            phi_8978_ = 0f;
                                                            loop {
                                                                let _e5739 = phi_8969_;
                                                                let _e5741 = phi_8972_;
                                                                let _e5743 = phi_8974_;
                                                                let _e5745 = phi_8976_;
                                                                let _e5747 = phi_8978_;
                                                                local_38 = _e5747;
                                                                local_39 = _e5745;
                                                                local_40 = _e5743;
                                                                local_41 = _e5745;
                                                                local_42 = _e5745;
                                                                local_43 = _e5743;
                                                                let _e5748 = (_e5739 < 6u);
                                                                if _e5748 {
                                                                    let _e5749 = (_e5739 == 0u);
                                                                    if _e5749 {
                                                                        phi_9020_ = true;
                                                                        phi_9021_ = false;
                                                                        phi_9022_ = false;
                                                                    } else {
                                                                        if (_e5739 <= 4u) {
                                                                            phi_8998_ = (_e5628 == 2u);
                                                                        } else {
                                                                            phi_8998_ = true;
                                                                        }
                                                                        let _e5753 = phi_8998_;
                                                                        if _e5753 {
                                                                            let _e5754 = (_e5739 == 5u);
                                                                            if _e5754 {
                                                                                if _e5712 {
                                                                                    phi_9014_ = false;
                                                                                } else {
                                                                                    phi_9014_ = (_e5628 == 2u);
                                                                                }
                                                                                let _e5757 = phi_9014_;
                                                                                phi_9016_ = _e5757;
                                                                            } else {
                                                                                phi_9016_ = false;
                                                                            }
                                                                            let _e5759 = phi_9016_;
                                                                            phi_9018_ = select(false, _e5712, _e5754);
                                                                            phi_9019_ = _e5759;
                                                                        } else {
                                                                            phi_9018_ = false;
                                                                            phi_9019_ = false;
                                                                        }
                                                                        let _e5762 = phi_9018_;
                                                                        let _e5764 = phi_9019_;
                                                                        phi_9020_ = select(true, false, _e5753);
                                                                        phi_9021_ = _e5762;
                                                                        phi_9022_ = _e5764;
                                                                    }
                                                                    let _e5767 = phi_9020_;
                                                                    let _e5769 = phi_9021_;
                                                                    let _e5771 = phi_9022_;
                                                                    if select(select(_e5771, false, _e5767), true, select(_e5769, true, _e5767)) {
                                                                        if _e5749 {
                                                                            phi_9069_ = 3u;
                                                                            phi_9070_ = vec3<f32>(((_e5698 * 1.7600001f) + 3f), ((_e5704 * 1.7600001f) + 3f), 3f);
                                                                        } else {
                                                                            if (_e5739 < 4u) {
                                                                                let _e5788 = (_e5739 - 1u);
                                                                                if (_e5788 == 0u) {
                                                                                    phi_23233_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                                } else {
                                                                                    phi_23233_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e5788 == 1u)));
                                                                                }
                                                                                let _e5794 = phi_23233_;
                                                                                phi_9067_ = 4u;
                                                                                phi_9068_ = (_e5726 + _e5794);
                                                                            } else {
                                                                                let _e5776 = (_e5739 == 4u);
                                                                                if _e5776 {
                                                                                    phi_9066_ = (_e5726 + vec3<f32>((_e5741.x * _e5724), (_e5741.y * _e5724), (_e5741.z * _e5724)));
                                                                                } else {
                                                                                    phi_9066_ = _e5736;
                                                                                }
                                                                                let _e5786 = phi_9066_;
                                                                                phi_9067_ = select(select(4u, 5u, _e5712), _e5722, _e5776);
                                                                                phi_9068_ = _e5786;
                                                                            }
                                                                            let _e5797 = phi_9067_;
                                                                            let _e5799 = phi_9068_;
                                                                            phi_9069_ = _e5797;
                                                                            phi_9070_ = _e5799;
                                                                        }
                                                                        let _e5806 = phi_9069_;
                                                                        let _e5808 = phi_9070_;
                                                                        let _e5810 = global_6.member[_e3715];
                                                                        let _e5811 = bitcast<u32>(_e5810);
                                                                        let _e5814 = global_6.member[(_e1189 + 11u)];
                                                                        let _e5817 = global_6.member[(_e1189 + 12u)];
                                                                        let _e5820 = global_6.member[(_e1189 + 13u)];
                                                                        let _e5823 = global_6.member[(_e1189 + 14u)];
                                                                        let _e5826 = global_6.member[(_e1189 + 15u)];
                                                                        let _e5829 = global_6.member[(_e1189 + 16u)];
                                                                        let _e5832 = global_6.member[(_e1189 + 17u)];
                                                                        let _e5835 = global_6.member[(_e1189 + 18u)];
                                                                        let _e5838 = global_6.member[(_e1189 + 19u)];
                                                                        phi_9205_ = 0u;
                                                                        phi_9208_ = 0f;
                                                                        phi_9210_ = 1f;
                                                                        phi_9212_ = 0f;
                                                                        phi_9214_ = _e5808;
                                                                        loop {
                                                                            let _e5840 = phi_9205_;
                                                                            let _e5842 = phi_9208_;
                                                                            let _e5844 = phi_9210_;
                                                                            let _e5846 = phi_9212_;
                                                                            let _e5848 = phi_9214_;
                                                                            local_35 = _e5842;
                                                                            local_36 = _e5846;
                                                                            local_37 = _e5842;
                                                                            let _e5849 = (_e5840 < _e5806);
                                                                            if _e5849 {
                                                                                let _e5851 = (_e5848.x + _e5673);
                                                                                let _e5853 = (_e5848.y + _e5676);
                                                                                let _e5855 = (_e5848.z + _e5679);
                                                                                let _e5856 = floor(_e5851);
                                                                                let _e5857 = floor(_e5853);
                                                                                let _e5858 = floor(_e5855);
                                                                                let _e5867 = bitcast<u32>((select(0i, select(select(i32(_e5856), i32(-2147483648), (_e5856 < -2147483600f)), 2147483647i, (_e5856 > 2147483500f)), (_e5856 == _e5856)) & 255i));
                                                                                let _e5876 = bitcast<u32>((select(0i, select(select(i32(_e5857), i32(-2147483648), (_e5857 < -2147483600f)), 2147483647i, (_e5857 > 2147483500f)), (_e5857 == _e5857)) & 255i));
                                                                                let _e5885 = bitcast<u32>((select(0i, select(select(i32(_e5858), i32(-2147483648), (_e5858 < -2147483600f)), 2147483647i, (_e5858 > 2147483500f)), (_e5858 == _e5858)) & 255i));
                                                                                let _e5886 = (_e5851 - _e5856);
                                                                                let _e5887 = (_e5853 - _e5857);
                                                                                let _e5888 = (_e5855 - _e5858);
                                                                                let _e5895 = (((_e5886 * _e5886) * _e5886) * ((_e5886 * ((_e5886 * 6f) - 15f)) + 10f));
                                                                                let _e5902 = (((_e5887 * _e5887) * _e5887) * ((_e5887 * ((_e5887 * 6f) - 15f)) + 10f));
                                                                                let _e5912 = global_6.member[(_e5811 + _e5867)];
                                                                                let _e5918 = (select(select(u32(_e5912), 0u, (_e5912 < 0f)), 4294967295u, (_e5912 > 4294967000f)) + _e5876);
                                                                                let _e5921 = global_6.member[(_e5811 + _e5918)];
                                                                                let _e5927 = (select(select(u32(_e5921), 0u, (_e5921 < 0f)), 4294967295u, (_e5921 > 4294967000f)) + _e5885);
                                                                                let _e5931 = global_6.member[(_e5811 + (_e5918 + 1u))];
                                                                                let _e5937 = (select(select(u32(_e5931), 0u, (_e5931 < 0f)), 4294967295u, (_e5931 > 4294967000f)) + _e5885);
                                                                                let _e5941 = global_6.member[(_e5811 + (_e5867 + 1u))];
                                                                                let _e5947 = (select(select(u32(_e5941), 0u, (_e5941 < 0f)), 4294967295u, (_e5941 > 4294967000f)) + _e5876);
                                                                                let _e5950 = global_6.member[(_e5811 + _e5947)];
                                                                                let _e5956 = (select(select(u32(_e5950), 0u, (_e5950 < 0f)), 4294967295u, (_e5950 > 4294967000f)) + _e5885);
                                                                                let _e5960 = global_6.member[(_e5811 + (_e5947 + 1u))];
                                                                                let _e5966 = (select(select(u32(_e5960), 0u, (_e5960 < 0f)), 4294967295u, (_e5960 > 4294967000f)) + _e5885);
                                                                                let _e5969 = global_6.member[(_e5811 + _e5927)];
                                                                                let _e5975 = (select(select(u32(_e5969), 0u, (_e5969 < 0f)), 4294967295u, (_e5969 > 4294967000f)) % 12u);
                                                                                let _e5978 = select(-1f, 1f, ((_e5975 & 1u) == 0u));
                                                                                let _e5981 = select(-1f, 1f, ((_e5975 & 2u) == 0u));
                                                                                if (_e5975 < 4u) {
                                                                                    phi_23294_ = ((_e5978 * _e5886) + (_e5981 * _e5887));
                                                                                } else {
                                                                                    if (_e5975 < 8u) {
                                                                                        phi_23288_ = ((_e5978 * _e5886) + (_e5981 * _e5888));
                                                                                    } else {
                                                                                        phi_23288_ = ((_e5978 * _e5887) + (_e5981 * _e5888));
                                                                                    }
                                                                                    let _e5991 = phi_23288_;
                                                                                    phi_23294_ = _e5991;
                                                                                }
                                                                                let _e5996 = phi_23294_;
                                                                                let _e5999 = global_6.member[(_e5811 + _e5956)];
                                                                                let _e6005 = (_e5886 - 1f);
                                                                                let _e6006 = (select(select(u32(_e5999), 0u, (_e5999 < 0f)), 4294967295u, (_e5999 > 4294967000f)) % 12u);
                                                                                let _e6009 = select(-1f, 1f, ((_e6006 & 1u) == 0u));
                                                                                let _e6012 = select(-1f, 1f, ((_e6006 & 2u) == 0u));
                                                                                if (_e6006 < 4u) {
                                                                                    phi_23328_ = ((_e6009 * _e6005) + (_e6012 * _e5887));
                                                                                } else {
                                                                                    if (_e6006 < 8u) {
                                                                                        phi_23322_ = ((_e6009 * _e6005) + (_e6012 * _e5888));
                                                                                    } else {
                                                                                        phi_23322_ = ((_e6009 * _e5887) + (_e6012 * _e5888));
                                                                                    }
                                                                                    let _e6022 = phi_23322_;
                                                                                    phi_23328_ = _e6022;
                                                                                }
                                                                                let _e6027 = phi_23328_;
                                                                                let _e6030 = global_6.member[(_e5811 + _e5937)];
                                                                                let _e6036 = (_e5887 - 1f);
                                                                                let _e6037 = (select(select(u32(_e6030), 0u, (_e6030 < 0f)), 4294967295u, (_e6030 > 4294967000f)) % 12u);
                                                                                let _e6040 = select(-1f, 1f, ((_e6037 & 1u) == 0u));
                                                                                let _e6043 = select(-1f, 1f, ((_e6037 & 2u) == 0u));
                                                                                if (_e6037 < 4u) {
                                                                                    phi_23362_ = ((_e6040 * _e5886) + (_e6043 * _e6036));
                                                                                } else {
                                                                                    if (_e6037 < 8u) {
                                                                                        phi_23356_ = ((_e6040 * _e5886) + (_e6043 * _e5888));
                                                                                    } else {
                                                                                        phi_23356_ = ((_e6040 * _e6036) + (_e6043 * _e5888));
                                                                                    }
                                                                                    let _e6053 = phi_23356_;
                                                                                    phi_23362_ = _e6053;
                                                                                }
                                                                                let _e6058 = phi_23362_;
                                                                                let _e6061 = global_6.member[(_e5811 + _e5966)];
                                                                                let _e6067 = (select(select(u32(_e6061), 0u, (_e6061 < 0f)), 4294967295u, (_e6061 > 4294967000f)) % 12u);
                                                                                let _e6070 = select(-1f, 1f, ((_e6067 & 1u) == 0u));
                                                                                let _e6073 = select(-1f, 1f, ((_e6067 & 2u) == 0u));
                                                                                if (_e6067 < 4u) {
                                                                                    phi_23396_ = ((_e6070 * _e6005) + (_e6073 * _e6036));
                                                                                } else {
                                                                                    if (_e6067 < 8u) {
                                                                                        phi_23390_ = ((_e6070 * _e6005) + (_e6073 * _e5888));
                                                                                    } else {
                                                                                        phi_23390_ = ((_e6070 * _e6036) + (_e6073 * _e5888));
                                                                                    }
                                                                                    let _e6083 = phi_23390_;
                                                                                    phi_23396_ = _e6083;
                                                                                }
                                                                                let _e6088 = phi_23396_;
                                                                                let _e6092 = global_6.member[(_e5811 + (_e5927 + 1u))];
                                                                                let _e6098 = (_e5888 - 1f);
                                                                                let _e6099 = (select(select(u32(_e6092), 0u, (_e6092 < 0f)), 4294967295u, (_e6092 > 4294967000f)) % 12u);
                                                                                let _e6102 = select(-1f, 1f, ((_e6099 & 1u) == 0u));
                                                                                let _e6105 = select(-1f, 1f, ((_e6099 & 2u) == 0u));
                                                                                if (_e6099 < 4u) {
                                                                                    phi_23430_ = ((_e6102 * _e5886) + (_e6105 * _e5887));
                                                                                } else {
                                                                                    if (_e6099 < 8u) {
                                                                                        phi_23424_ = ((_e6102 * _e5886) + (_e6105 * _e6098));
                                                                                    } else {
                                                                                        phi_23424_ = ((_e6102 * _e5887) + (_e6105 * _e6098));
                                                                                    }
                                                                                    let _e6115 = phi_23424_;
                                                                                    phi_23430_ = _e6115;
                                                                                }
                                                                                let _e6120 = phi_23430_;
                                                                                let _e6124 = global_6.member[(_e5811 + (_e5956 + 1u))];
                                                                                let _e6130 = (select(select(u32(_e6124), 0u, (_e6124 < 0f)), 4294967295u, (_e6124 > 4294967000f)) % 12u);
                                                                                let _e6133 = select(-1f, 1f, ((_e6130 & 1u) == 0u));
                                                                                let _e6136 = select(-1f, 1f, ((_e6130 & 2u) == 0u));
                                                                                if (_e6130 < 4u) {
                                                                                    phi_23464_ = ((_e6133 * _e6005) + (_e6136 * _e5887));
                                                                                } else {
                                                                                    if (_e6130 < 8u) {
                                                                                        phi_23458_ = ((_e6133 * _e6005) + (_e6136 * _e6098));
                                                                                    } else {
                                                                                        phi_23458_ = ((_e6133 * _e5887) + (_e6136 * _e6098));
                                                                                    }
                                                                                    let _e6146 = phi_23458_;
                                                                                    phi_23464_ = _e6146;
                                                                                }
                                                                                let _e6151 = phi_23464_;
                                                                                let _e6155 = global_6.member[(_e5811 + (_e5937 + 1u))];
                                                                                let _e6161 = (select(select(u32(_e6155), 0u, (_e6155 < 0f)), 4294967295u, (_e6155 > 4294967000f)) % 12u);
                                                                                let _e6164 = select(-1f, 1f, ((_e6161 & 1u) == 0u));
                                                                                let _e6167 = select(-1f, 1f, ((_e6161 & 2u) == 0u));
                                                                                if (_e6161 < 4u) {
                                                                                    phi_23498_ = ((_e6164 * _e5886) + (_e6167 * _e6036));
                                                                                } else {
                                                                                    if (_e6161 < 8u) {
                                                                                        phi_23492_ = ((_e6164 * _e5886) + (_e6167 * _e6098));
                                                                                    } else {
                                                                                        phi_23492_ = ((_e6164 * _e6036) + (_e6167 * _e6098));
                                                                                    }
                                                                                    let _e6177 = phi_23492_;
                                                                                    phi_23498_ = _e6177;
                                                                                }
                                                                                let _e6182 = phi_23498_;
                                                                                let _e6186 = global_6.member[(_e5811 + (_e5966 + 1u))];
                                                                                let _e6192 = (select(select(u32(_e6186), 0u, (_e6186 < 0f)), 4294967295u, (_e6186 > 4294967000f)) % 12u);
                                                                                let _e6195 = select(-1f, 1f, ((_e6192 & 1u) == 0u));
                                                                                let _e6198 = select(-1f, 1f, ((_e6192 & 2u) == 0u));
                                                                                if (_e6192 < 4u) {
                                                                                    phi_23532_ = ((_e6195 * _e6005) + (_e6198 * _e6036));
                                                                                } else {
                                                                                    if (_e6192 < 8u) {
                                                                                        phi_23526_ = ((_e6195 * _e6005) + (_e6198 * _e6098));
                                                                                    } else {
                                                                                        phi_23526_ = ((_e6195 * _e6036) + (_e6198 * _e6098));
                                                                                    }
                                                                                    let _e6208 = phi_23526_;
                                                                                    phi_23532_ = _e6208;
                                                                                }
                                                                                let _e6213 = phi_23532_;
                                                                                let _e6216 = (_e5996 + (_e5895 * (_e6027 - _e5996)));
                                                                                let _e6222 = (_e6120 + (_e5895 * (_e6151 - _e6120)));
                                                                                let _e6228 = (_e6216 + (_e5902 * ((_e6058 + (_e5895 * (_e6088 - _e6058))) - _e6216)));
                                                                                phi_9206_ = (_e5840 + 1u);
                                                                                phi_9209_ = (_e5842 + _e5844);
                                                                                phi_9211_ = (_e5844 * 0.5f);
                                                                                phi_9213_ = (_e5846 + ((_e6228 + ((((_e5888 * _e5888) * _e5888) * ((_e5888 * ((_e5888 * 6f) - 15f)) + 10f)) * ((_e6222 + (_e5902 * ((_e6182 + (_e5895 * (_e6213 - _e6182))) - _e6222))) - _e6228))) * _e5844));
                                                                                phi_9215_ = vec3<f32>(((((_e5814 * _e5848.x) + (_e5817 * _e5848.y)) + (_e5820 * _e5848.z)) * 2.03f), ((((_e5823 * _e5848.x) + (_e5826 * _e5848.y)) + (_e5829 * _e5848.z)) * 2.03f), ((((_e5832 * _e5848.x) + (_e5835 * _e5848.y)) + (_e5838 * _e5848.z)) * 2.03f));
                                                                            } else {
                                                                                phi_9206_ = u32();
                                                                                phi_9209_ = f32();
                                                                                phi_9211_ = f32();
                                                                                phi_9213_ = f32();
                                                                                phi_9215_ = vec3<f32>();
                                                                            }
                                                                            let _e6260 = phi_9206_;
                                                                            let _e6262 = phi_9209_;
                                                                            let _e6264 = phi_9211_;
                                                                            let _e6266 = phi_9213_;
                                                                            let _e6268 = phi_9215_;
                                                                            phi_9205_ = _e6260;
                                                                            phi_9208_ = _e6262;
                                                                            phi_9210_ = _e6264;
                                                                            phi_9212_ = _e6266;
                                                                            phi_9214_ = _e6268;
                                                                            if !(_e5849) {
                                                                                break;
                                                                            }
                                                                        }
                                                                        let _e6271 = local_35;
                                                                        if (_e6271 > 0f) {
                                                                            let _e6274 = local_36;
                                                                            let _e6276 = local_37;
                                                                            phi_9628_ = (_e6274 / _e6276);
                                                                        } else {
                                                                            phi_9628_ = 0f;
                                                                        }
                                                                        let _e6279 = phi_9628_;
                                                                        if _e5749 {
                                                                            phi_9663_ = _e5741;
                                                                            phi_9664_ = _e5743;
                                                                            phi_9665_ = _e5745;
                                                                        } else {
                                                                            if (_e5739 == 1u) {
                                                                                phi_9660_ = vec3<f32>(_e6279, _e5741.y, _e5741.z);
                                                                                phi_9661_ = _e5743;
                                                                                phi_9662_ = _e5745;
                                                                            } else {
                                                                                if (_e5739 == 2u) {
                                                                                    phi_9657_ = vec3<f32>(_e5741.x, _e6279, _e5741.z);
                                                                                    phi_9658_ = _e5743;
                                                                                    phi_9659_ = _e5745;
                                                                                } else {
                                                                                    if (_e5739 == 3u) {
                                                                                        phi_9654_ = vec3<f32>(_e5741.x, _e5741.y, _e6279);
                                                                                        phi_9655_ = _e5743;
                                                                                        phi_9656_ = _e5745;
                                                                                    } else {
                                                                                        let _e6283 = (_e5739 == 4u);
                                                                                        phi_9654_ = _e5741;
                                                                                        phi_9655_ = select(_e6279, _e5743, _e6283);
                                                                                        phi_9656_ = select(_e5745, _e6279, _e6283);
                                                                                    }
                                                                                    let _e6291 = phi_9654_;
                                                                                    let _e6293 = phi_9655_;
                                                                                    let _e6295 = phi_9656_;
                                                                                    phi_9657_ = _e6291;
                                                                                    phi_9658_ = _e6293;
                                                                                    phi_9659_ = _e6295;
                                                                                }
                                                                                let _e6301 = phi_9657_;
                                                                                let _e6303 = phi_9658_;
                                                                                let _e6305 = phi_9659_;
                                                                                phi_9660_ = _e6301;
                                                                                phi_9661_ = _e6303;
                                                                                phi_9662_ = _e6305;
                                                                            }
                                                                            let _e6311 = phi_9660_;
                                                                            let _e6313 = phi_9661_;
                                                                            let _e6315 = phi_9662_;
                                                                            phi_9663_ = _e6311;
                                                                            phi_9664_ = _e6313;
                                                                            phi_9665_ = _e6315;
                                                                        }
                                                                        let _e6317 = phi_9663_;
                                                                        let _e6319 = phi_9664_;
                                                                        let _e6321 = phi_9665_;
                                                                        phi_9667_ = _e6317;
                                                                        phi_9668_ = _e6319;
                                                                        phi_9669_ = _e6321;
                                                                        phi_9670_ = select(_e5747, _e6279, _e5749);
                                                                    } else {
                                                                        phi_9667_ = _e5741;
                                                                        phi_9668_ = _e5743;
                                                                        phi_9669_ = _e5745;
                                                                        phi_9670_ = _e5747;
                                                                    }
                                                                    let _e6324 = phi_9667_;
                                                                    let _e6326 = phi_9668_;
                                                                    let _e6328 = phi_9669_;
                                                                    let _e6330 = phi_9670_;
                                                                    phi_8970_ = (_e5739 + 1u);
                                                                    phi_8973_ = _e6324;
                                                                    phi_8975_ = _e6326;
                                                                    phi_8977_ = _e6328;
                                                                    phi_8979_ = _e6330;
                                                                } else {
                                                                    phi_8970_ = u32();
                                                                    phi_8973_ = vec3<f32>();
                                                                    phi_8975_ = f32();
                                                                    phi_8977_ = f32();
                                                                    phi_8979_ = f32();
                                                                }
                                                                let _e6333 = phi_8970_;
                                                                let _e6335 = phi_8973_;
                                                                let _e6337 = phi_8975_;
                                                                let _e6339 = phi_8977_;
                                                                let _e6341 = phi_8979_;
                                                                phi_8969_ = _e6333;
                                                                phi_8972_ = _e6335;
                                                                phi_8974_ = _e6337;
                                                                phi_8976_ = _e6339;
                                                                phi_8978_ = _e6341;
                                                                if !(_e5748) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e6344 = local_38;
                                                            let _e6347 = (_e5708 * (1f + (0.45f * _e6344)));
                                                            if _e5712 {
                                                                let _e6450 = local_42;
                                                                let _e6452 = ((_e6450 - -0.25f) * 1.4285715f);
                                                                let _e6454 = select(_e6452, 0f, (_e6452 < 0f));
                                                                let _e6456 = select(_e6454, 1f, (_e6454 > 1f));
                                                                let _e6460 = ((_e6456 * _e6456) * (3f - (2f * _e6456)));
                                                                let _e6462 = ((_e6347 - 1.05f) * -1.1764706f);
                                                                let _e6464 = select(_e6462, 0f, (_e6462 < 0f));
                                                                let _e6466 = select(_e6464, 1f, (_e6464 > 1f));
                                                                let _e6470 = ((_e6466 * _e6466) * (3f - (2f * _e6466)));
                                                                let _e6471 = (_e6460 * _e6470);
                                                                let _e6473 = ((_e6347 - 0.65f) * -1.5384616f);
                                                                let _e6475 = select(_e6473, 0f, (_e6473 < 0f));
                                                                let _e6477 = select(_e6475, 1f, (_e6475 > 1f));
                                                                let _e6482 = (((_e6477 * _e6477) * (3f - (2f * _e6477))) * _e6460);
                                                                let _e6484 = local_43;
                                                                let _e6486 = ((_e6484 - 0.1f) * 2.857143f);
                                                                let _e6488 = select(_e6486, 0f, (_e6486 < 0f));
                                                                let _e6490 = select(_e6488, 1f, (_e6488 > 1f));
                                                                let _e6495 = (((_e6490 * _e6490) * (3f - (2f * _e6490))) * _e6470);
                                                                let _e6500 = ((0.8f * _e6482) * _e6482);
                                                                let _e6509 = (_e5646 * exp((-2f * _e6495)));
                                                                phi_9962_ = type_59(vec3<f32>((((3.1815925f * _e6471) + (0.14634217f * _e6500)) * _e6509), (((0.38179109f * _e6471) + (1.2439084f * _e6500)) * _e6509), (((0.69995034f * _e6471) + (1.0975662f * _e6500)) * _e6509)), (_e5649 * _e6495));
                                                            } else {
                                                                if (_e5628 == 1u) {
                                                                    let _e6420 = local_41;
                                                                    let _e6422 = ((_e6420 - -0.3f) * 1.25f);
                                                                    let _e6424 = select(_e6422, 0f, (_e6422 < 0f));
                                                                    let _e6426 = select(_e6424, 1f, (_e6424 > 1f));
                                                                    let _e6432 = ((_e6347 - 1f) * -1f);
                                                                    let _e6434 = select(_e6432, 0f, (_e6432 < 0f));
                                                                    let _e6436 = select(_e6434, 1f, (_e6434 > 1f));
                                                                    let _e6442 = (((_e6426 * _e6426) * (3f - (2f * _e6426))) * pow(((_e6436 * _e6436) * (3f - (2f * _e6436))), 1.5f));
                                                                    phi_9961_ = type_59((vec3<f32>(0.6481841f, 1.0185751f, 1.8519546f) * (_e5646 * _e6442)), (_e5649 * _e6442));
                                                                } else {
                                                                    if (_e5628 == 2u) {
                                                                        let _e6376 = local_40;
                                                                        let _e6379 = (_e5708 * (1f + (0.15f * _e6376)));
                                                                        let _e6380 = (_e6379 - 0.65f);
                                                                        let _e6384 = exp(((_e6380 * -5.5555553f) * (_e6380 * 5.5555553f)));
                                                                        let _e6386 = ((_e6379 - 0.75f) * -1.8181818f);
                                                                        let _e6388 = select(_e6386, 0f, (_e6386 < 0f));
                                                                        let _e6390 = select(_e6388, 1f, (_e6388 > 1f));
                                                                        let _e6394 = ((_e6390 * _e6390) * (3f - (2f * _e6390)));
                                                                        let _e6399 = (exp(((_e5708 * -25f) * (_e5708 * 25f))) * 3f);
                                                                        phi_9960_ = type_59(vec3<f32>(((((3.1815925f * _e6384) + (_e6394 * 0.10243952f)) + _e6399) * _e5646), ((((0.38179109f * _e6384) + (_e6394 * 0.8707359f)) + _e6399) * _e5646), ((((0.69995034f * _e6384) + (_e6394 * 0.76829636f)) + _e6399) * _e5646)), 0f);
                                                                    } else {
                                                                        let _e6351 = local_39;
                                                                        let _e6353 = ((_e6351 - -0.2f) * 1.6666666f);
                                                                        let _e6355 = select(_e6353, 0f, (_e6353 < 0f));
                                                                        let _e6357 = select(_e6355, 1f, (_e6355 > 1f));
                                                                        let _e6363 = ((_e6347 - 1f) * -1.4285715f);
                                                                        let _e6365 = select(_e6363, 0f, (_e6363 < 0f));
                                                                        let _e6367 = select(_e6365, 1f, (_e6365 > 1f));
                                                                        phi_9960_ = type_59(vec3<f32>(0f, 0f, 0f), (_e5649 * (((_e6357 * _e6357) * (3f - (2f * _e6357))) * ((_e6367 * _e6367) * (3f - (2f * _e6367))))));
                                                                    }
                                                                    let _e6418 = phi_9960_;
                                                                    phi_9961_ = _e6418;
                                                                }
                                                                let _e6448 = phi_9961_;
                                                                phi_9962_ = _e6448;
                                                            }
                                                            let _e6517 = phi_9962_;
                                                            phi_9963_ = _e6517;
                                                        }
                                                        let _e6519 = phi_9963_;
                                                        phi_8636_ = (_e5614 + _e6519.member);
                                                        phi_8639_ = (_e5616 + 1u);
                                                        phi_8641_ = (_e5618 + _e6519.member_1);
                                                    } else {
                                                        phi_8636_ = vec3<f32>();
                                                        phi_8639_ = u32();
                                                        phi_8641_ = f32();
                                                    }
                                                    let _e6526 = phi_8636_;
                                                    let _e6528 = phi_8639_;
                                                    let _e6530 = phi_8641_;
                                                    phi_8635_ = _e6526;
                                                    phi_8638_ = _e6528;
                                                    phi_8640_ = _e6530;
                                                    if !(_e5619) {
                                                        break;
                                                    }
                                                }
                                                phi_30727_ = _e4004;
                                                if _e4004 {
                                                    break;
                                                }
                                                let _e6533 = local_44;
                                                if (_e6533 <= 0f) {
                                                    phi_9992_ = vec3<f32>(1f, 1f, 1f);
                                                } else {
                                                    let _e6536 = local_45;
                                                    let _e6539 = local_46;
                                                    let _e6544 = local_47;
                                                    phi_9992_ = vec3<f32>(exp((_e6539 * -0.81f)), exp(-(_e6536)), exp((_e6544 * -1.25f)));
                                                }
                                                let _e6549 = phi_9992_;
                                                let _e6551 = local_48;
                                                let _e6556 = local_49;
                                                let _e6561 = local_50;
                                                let _e6566 = local_51;
                                                let _e6570 = local_52;
                                                let _e6574 = local_53;
                                                let _e6579 = global_6.member[(_e1189 + 9u)];
                                                phi_30732_ = _e4004;
                                                phi_10039_ = vec3<f32>((_e3706.x + ((((_e6551.x * _e6549.x) + _e6566.x) * _e6579) * _e3708.x)), (_e3706.y + ((((_e6556.y * _e6549.y) + _e6570.y) * _e6579) * _e3708.y)), (_e3706.z + ((((_e6561.z * _e6549.z) + _e6574.z) * _e6579) * _e3708.z)));
                                            } else {
                                                phi_30732_ = _e3704;
                                                phi_10039_ = _e3706;
                                            }
                                            let _e6597 = phi_30732_;
                                            let _e6599 = phi_10039_;
                                            phi_30731_ = _e6597;
                                            phi_10040_ = _e6599;
                                        } else {
                                            phi_30731_ = _e3704;
                                            phi_10040_ = _e3712;
                                        }
                                        let _e6601 = phi_30731_;
                                        let _e6603 = phi_10040_;
                                        phi_30730_ = _e6601;
                                        phi_10041_ = _e6603;
                                    } else {
                                        phi_30730_ = _e1160;
                                        phi_10041_ = _e1337;
                                    }
                                    let _e6605 = phi_30730_;
                                    let _e6607 = phi_10041_;
                                    phi_30729_ = _e6605;
                                    phi_10060_ = vec3<f32>((_e1162.x + (_e1171.x * _e6607.x)), (_e1162.y + (_e1171.y * _e6607.y)), (_e1162.z + (_e1171.z * _e6607.z)));
                                } else {
                                    phi_30729_ = _e1160;
                                    phi_10060_ = _e1162;
                                }
                                let _e6622 = phi_30729_;
                                let _e6624 = phi_10060_;
                                phi_30728_ = _e6622;
                                phi_1371_ = _e6624;
                                phi_1374_ = (_e1164 + 1i);
                            } else {
                                phi_30728_ = _e1160;
                                phi_1371_ = vec3<f32>();
                                phi_1374_ = i32();
                            }
                            let _e6627 = phi_30728_;
                            let _e6629 = phi_1371_;
                            let _e6631 = phi_1374_;
                            phi_30317_ = _e6627;
                            phi_1370_ = _e6629;
                            phi_1373_ = _e6631;
                            phi_30727_ = _e6627;
                            if !(_e1165) {
                                break;
                            }
                        }
                        let _e6634 = phi_30727_;
                        phi_35216_ = _e6634;
                        phi_19906_ = type_49();
                        phi_19905_ = u32();
                        phi_19904_ = vec3<f32>();
                        phi_19903_ = bool();
                        phi_19902_ = bool();
                        phi_19901_ = bool();
                        phi_19900_ = bool();
                        phi_19899_ = vec3<f32>();
                        phi_19897_ = u32();
                        phi_19895_ = type_14();
                        phi_19893_ = vec3<f32>();
                        phi_19892_ = f32();
                        phi_19891_ = f32();
                        phi_19890_ = f32();
                        phi_19888_ = vec3<f32>();
                        if _e6634 {
                            break;
                        }
                        if _e387 {
                            phi_35315_ = _e373;
                            phi_35217_ = _e6634;
                            phi_17102_ = vec3<f32>();
                            phi_17103_ = vec3<f32>();
                            phi_17104_ = f32();
                            phi_17105_ = f32();
                            phi_17106_ = f32();
                            phi_17107_ = vec3<f32>();
                            phi_17108_ = type_14();
                            phi_17109_ = u32();
                            phi_490_ = u32();
                            phi_493_ = vec3<f32>();
                            phi_495_ = vec3<f32>();
                            phi_497_ = vec3<f32>();
                            phi_499_ = vec3<f32>();
                            phi_501_ = bool();
                            phi_503_ = vec3<f32>();
                            phi_505_ = vec3<f32>();
                            phi_507_ = u32();
                            phi_509_ = bool();
                            phi_511_ = f32();
                            phi_513_ = bool();
                            phi_515_ = vec3<f32>();
                            phi_17112_ = false;
                            phi_17114_ = false;
                            phi_17115_ = false;
                            phi_17116_ = false;
                        } else {
                            if _e1158 {
                                phi_35316_ = _e373;
                                phi_35218_ = _e6634;
                                phi_17075_ = vec3<f32>();
                                phi_17076_ = vec3<f32>();
                                phi_17077_ = f32();
                                phi_17078_ = f32();
                                phi_17079_ = f32();
                                phi_17080_ = vec3<f32>();
                                phi_17081_ = type_14();
                                phi_17082_ = u32();
                                phi_17083_ = u32();
                                phi_17084_ = vec3<f32>();
                                phi_17085_ = vec3<f32>();
                                phi_17086_ = vec3<f32>();
                                phi_17087_ = vec3<f32>();
                                phi_17088_ = bool();
                                phi_17089_ = vec3<f32>();
                                phi_17090_ = vec3<f32>();
                                phi_17091_ = u32();
                                phi_17092_ = bool();
                                phi_17093_ = f32();
                                phi_17094_ = bool();
                                phi_17095_ = vec3<f32>();
                                phi_17098_ = false;
                                phi_17099_ = false;
                                phi_17100_ = false;
                                phi_17101_ = false;
                            } else {
                                if ((_e368 & 8u) == 0u) {
                                    phi_33431_ = _e6634;
                                    let _e14516 = local_117;
                                    phi_11381_ = _e14516;
                                    phi_11382_ = _e381;
                                } else {
                                    if (_e1156.member > 100f) {
                                        let _e6641 = global_1.member.member_8[1u];
                                        let _e6643 = global_6.member[_e6641];
                                        let _e6646 = global_6.member[(_e6641 + 1u)];
                                        let _e6649 = global_6.member[(_e6641 + 2u)];
                                        let _e6652 = global_6.member[(_e6641 + 3u)];
                                        let _e6655 = global_6.member[(_e6641 + 4u)];
                                        let _e6658 = global_6.member[(_e6641 + 5u)];
                                        let _e6661 = global_6.member[(_e6641 + 6u)];
                                        let _e6664 = global_6.member[(_e6641 + 7u)];
                                        let _e6667 = global_6.member[(_e6641 + 8u)];
                                        let _e6670 = global_6.member[(_e6641 + 9u)];
                                        let _e6673 = global_6.member[(_e6641 + 10u)];
                                        let _e6676 = global_6.member[(_e6641 + 11u)];
                                        let _e6679 = global_6.member[(_e6641 + 12u)];
                                        let _e6682 = global_6.member[(_e6641 + 13u)];
                                        let _e6685 = global_6.member[(_e6641 + 14u)];
                                        let _e6688 = global_6.member[(_e6641 + 15u)];
                                        let _e6692 = global_6.member[(_e6641 + 16u)];
                                        let _e6695 = global_6.member[(_e6641 + 17u)];
                                        let _e6698 = global_6.member[(_e6641 + 18u)];
                                        let _e6701 = global_6.member[(_e6641 + 19u)];
                                        let _e6704 = global_6.member[(_e6641 + 20u)];
                                        let _e6707 = global_6.member[(_e6641 + 29u)];
                                        let _e6708 = bitcast<u32>(_e6707);
                                        let _e6711 = global_6.member[(_e6641 + 30u)];
                                        let _e6712 = bitcast<u32>(_e6711);
                                        let _e6726 = ((((_e401.x - _e6643) * _e385.x) + ((_e401.y - _e6646) * _e385.y)) + ((_e401.z - _e6649) * _e385.z));
                                        let _e6738 = ((((_e401.x * _e401.x) + (_e401.y * _e401.y)) + (_e401.z * _e401.z)) - (2f * (((_e401.x * _e6643) + (_e401.y * _e6646)) + (_e401.z * _e6649))));
                                        let _e6739 = (_e6738 + _e6661);
                                        let _e6740 = (_e6726 * _e6726);
                                        let _e6741 = (_e6740 - _e6739);
                                        if (_e6741 < 0f) {
                                            phi_23739_ = type_57(0u, type_57().member_1);
                                            phi_23740_ = type_57();
                                            phi_23741_ = true;
                                        } else {
                                            if (_e6726 < 0f) {
                                                phi_23711_ = -(sqrt(_e6741));
                                            } else {
                                                phi_23711_ = sqrt(_e6741);
                                            }
                                            let _e6748 = phi_23711_;
                                            let _e6750 = -((_e6726 + _e6748));
                                            let _e6751 = (_e6750 == 0f);
                                            if _e6751 {
                                                phi_23734_ = type_57(1u, type_52(0f, 0f));
                                                phi_23735_ = type_57();
                                            } else {
                                                let _e6752 = (_e6739 / _e6750);
                                                if (_e6750 < _e6752) {
                                                    phi_23723_ = type_52(_e6750, _e6752);
                                                } else {
                                                    phi_23723_ = type_52(_e6752, _e6750);
                                                }
                                                let _e6757 = phi_23723_;
                                                phi_23734_ = type_57();
                                                phi_23735_ = type_57(1u, _e6757);
                                            }
                                            let _e6760 = phi_23734_;
                                            let _e6762 = phi_23735_;
                                            phi_23739_ = _e6760;
                                            phi_23740_ = _e6762;
                                            phi_23741_ = _e6751;
                                        }
                                        let _e6767 = phi_23739_;
                                        let _e6769 = phi_23740_;
                                        let _e6771 = phi_23741_;
                                        if _e6771 {
                                            phi_23745_ = _e6767;
                                        } else {
                                            phi_23745_ = _e6769;
                                        }
                                        let _e6773 = phi_23745_;
                                        if (bitcast<i32>(_e6773.member) != 0i) {
                                            phi_23632_ = type_57(0u, _e6773.member_1);
                                        } else {
                                            phi_23632_ = type_57(1u, type_57().member_1);
                                        }
                                        let _e6783 = phi_23632_;
                                        if (bitcast<i32>(_e6783.member) != 0i) {
                                            phi_23679_ = type_57(0u, type_57().member_1);
                                        } else {
                                            if (_e6783.member_1.member_1 <= 0f) {
                                                phi_23675_ = type_57(0u, type_57().member_1);
                                            } else {
                                                if (_e6783.member_1.member != _e6783.member_1.member) {
                                                    phi_23783_ = true;
                                                } else {
                                                    phi_23783_ = (0f >= _e6783.member_1.member);
                                                }
                                                let _e6795 = phi_23783_;
                                                let _e6796 = select(_e6783.member_1.member, 0f, _e6795);
                                                let _e6797 = (_e6738 + _e6658);
                                                if (_e6797 > 0f) {
                                                    let _e6799 = (_e6740 - _e6797);
                                                    if (_e6799 < 0f) {
                                                        phi_23891_ = type_57(0u, type_57().member_1);
                                                        phi_23892_ = type_57();
                                                        phi_23893_ = true;
                                                    } else {
                                                        if (_e6726 < 0f) {
                                                            phi_23863_ = -(sqrt(_e6799));
                                                        } else {
                                                            phi_23863_ = sqrt(_e6799);
                                                        }
                                                        let _e6806 = phi_23863_;
                                                        let _e6808 = -((_e6726 + _e6806));
                                                        let _e6809 = (_e6808 == 0f);
                                                        if _e6809 {
                                                            phi_23886_ = type_57(1u, type_52(0f, 0f));
                                                            phi_23887_ = type_57();
                                                        } else {
                                                            let _e6810 = (_e6797 / _e6808);
                                                            if (_e6808 < _e6810) {
                                                                phi_23875_ = type_52(_e6808, _e6810);
                                                            } else {
                                                                phi_23875_ = type_52(_e6810, _e6808);
                                                            }
                                                            let _e6815 = phi_23875_;
                                                            phi_23886_ = type_57();
                                                            phi_23887_ = type_57(1u, _e6815);
                                                        }
                                                        let _e6818 = phi_23886_;
                                                        let _e6820 = phi_23887_;
                                                        phi_23891_ = _e6818;
                                                        phi_23892_ = _e6820;
                                                        phi_23893_ = _e6809;
                                                    }
                                                    let _e6825 = phi_23891_;
                                                    let _e6827 = phi_23892_;
                                                    let _e6829 = phi_23893_;
                                                    if _e6829 {
                                                        phi_23897_ = _e6825;
                                                    } else {
                                                        phi_23897_ = _e6827;
                                                    }
                                                    let _e6831 = phi_23897_;
                                                    if (bitcast<i32>(_e6831.member) != 0i) {
                                                        phi_23831_ = select(_e6831.member_1.member_1, _e6831.member_1.member, (_e6831.member_1.member >= 0f));
                                                    } else {
                                                        phi_23831_ = -1f;
                                                    }
                                                    let _e6842 = phi_23831_;
                                                    if (_e6842 >= 0f) {
                                                        if (_e6783.member_1.member_1 != _e6783.member_1.member_1) {
                                                            phi_23935_ = true;
                                                        } else {
                                                            phi_23935_ = (_e6842 <= _e6783.member_1.member_1);
                                                        }
                                                        let _e6847 = phi_23935_;
                                                        phi_23660_ = select(_e6783.member_1.member_1, _e6842, _e6847);
                                                    } else {
                                                        phi_23660_ = _e6783.member_1.member_1;
                                                    }
                                                    let _e6850 = phi_23660_;
                                                    phi_23662_ = _e6850;
                                                } else {
                                                    phi_23662_ = _e6783.member_1.member_1;
                                                }
                                                let _e6852 = phi_23662_;
                                                if (_e6852 <= _e6796) {
                                                    phi_23671_ = type_57(0u, type_57().member_1);
                                                } else {
                                                    phi_23671_ = type_57(1u, type_52(_e6796, _e6852));
                                                }
                                                let _e6860 = phi_23671_;
                                                phi_23675_ = _e6860;
                                            }
                                            let _e6865 = phi_23675_;
                                            phi_23679_ = _e6865;
                                        }
                                        let _e6870 = phi_23679_;
                                        if (bitcast<i32>(_e6870.member) != 0i) {
                                            let _e6876 = (_e6870.member_1.member < _e1156.member);
                                            if _e6876 {
                                                if (_e6870.member_1.member_1 != _e6870.member_1.member_1) {
                                                    phi_23950_ = true;
                                                } else {
                                                    phi_23950_ = (_e1156.member <= _e6870.member_1.member_1);
                                                }
                                                let _e6882 = phi_23950_;
                                                let _e6883 = select(_e6870.member_1.member_1, _e1156.member, _e6882);
                                                phi_10459_ = vec3<f32>(0f, 0f, 0f);
                                                phi_10462_ = vec3<f32>(0f, 0f, 0f);
                                                phi_10464_ = vec3<f32>(0f, 0f, 0f);
                                                phi_10466_ = vec3<f32>(0f, 0f, 0f);
                                                phi_10468_ = 0u;
                                                loop {
                                                    let _e6885 = phi_10459_;
                                                    let _e6887 = phi_10462_;
                                                    let _e6889 = phi_10464_;
                                                    let _e6891 = phi_10466_;
                                                    let _e6893 = phi_10468_;
                                                    local_54 = _e6889;
                                                    local_55 = _e6887;
                                                    local_56 = _e6885;
                                                    local_57 = _e6885;
                                                    local_58 = _e6885;
                                                    local_59 = _e6891;
                                                    local_60 = _e6891;
                                                    local_61 = _e6891;
                                                    let _e6894 = (_e6893 < 32u);
                                                    if _e6894 {
                                                        let _e6895 = -(_e6726);
                                                        if (_e6870.member_1.member <= _e6883) {
                                                        } else {
                                                            phi_30734_ = true;
                                                            break;
                                                        }
                                                        let _e6898 = select(_e6895, _e6870.member_1.member, (_e6895 < _e6870.member_1.member));
                                                        let _e6900 = select(_e6898, _e6883, (_e6898 > _e6883));
                                                        let _e6901 = (_e6900 - _e6870.member_1.member);
                                                        let _e6902 = (_e6883 - _e6900);
                                                        let _e6903 = (_e6883 - _e6870.member_1.member);
                                                        let _e6904 = (0.000000001f * _e6903);
                                                        if (_e6901 <= _e6904) {
                                                            let _e6979 = (f32(_e6893) * 0.03125f);
                                                            let _e6982 = (f32((_e6893 + 1u)) * 0.03125f);
                                                            let _e6984 = ((_e6979 + _e6982) * 0.5f);
                                                            phi_10575_ = type_52((_e6870.member_1.member + ((_e6903 * _e6984) * _e6984)), (_e6903 * ((_e6982 * _e6982) - (_e6979 * _e6979))));
                                                        } else {
                                                            if (_e6902 <= _e6904) {
                                                                let _e6962 = (f32((31u - _e6893)) * 0.03125f);
                                                                let _e6965 = (f32((32u - _e6893)) * 0.03125f);
                                                                let _e6967 = ((_e6962 + _e6965) * 0.5f);
                                                                phi_10574_ = type_52((_e6883 - ((_e6903 * _e6967) * _e6967)), (_e6903 * ((_e6965 * _e6965) - (_e6962 * _e6962))));
                                                            } else {
                                                                let _e6910 = round(((32f * _e6901) / (_e6901 + _e6902)));
                                                                let _e6915 = select(select(u32(_e6910), 0u, (_e6910 < 0f)), 4294967295u, (_e6910 > 4294967000f));
                                                                if (_e6915 < 1u) {
                                                                    phi_10558_ = 1u;
                                                                } else {
                                                                    phi_10558_ = select(_e6915, 31u, (_e6915 > 31u));
                                                                }
                                                                let _e6920 = phi_10558_;
                                                                if (_e6893 < _e6920) {
                                                                    let _e6941 = ((_e6920 - 1u) - _e6893);
                                                                    let _e6943 = f32(_e6920);
                                                                    let _e6944 = (f32(_e6941) / _e6943);
                                                                    let _e6947 = (f32((_e6941 + 1u)) / _e6943);
                                                                    let _e6949 = ((_e6944 + _e6947) * 0.5f);
                                                                    phi_10573_ = type_52((_e6900 - ((_e6901 * _e6949) * _e6949)), (_e6901 * ((_e6947 * _e6947) - (_e6944 * _e6944))));
                                                                } else {
                                                                    let _e6923 = (_e6893 - _e6920);
                                                                    let _e6925 = f32((32u - _e6920));
                                                                    let _e6926 = (f32(_e6923) / _e6925);
                                                                    let _e6929 = (f32((_e6923 + 1u)) / _e6925);
                                                                    let _e6931 = ((_e6926 + _e6929) * 0.5f);
                                                                    phi_10573_ = type_52((_e6900 + ((_e6902 * _e6931) * _e6931)), (_e6902 * ((_e6929 * _e6929) - (_e6926 * _e6926))));
                                                                }
                                                                let _e6959 = phi_10573_;
                                                                phi_10574_ = _e6959;
                                                            }
                                                            let _e6977 = phi_10574_;
                                                            phi_10575_ = _e6977;
                                                        }
                                                        let _e6994 = phi_10575_;
                                                        let _e6997 = (_e385.x * _e6994.member);
                                                        let _e6998 = (_e385.y * _e6994.member);
                                                        let _e6999 = (_e385.z * _e6994.member);
                                                        let _e7000 = (_e401.x + _e6997);
                                                        let _e7001 = (_e401.y + _e6998);
                                                        let _e7002 = (_e401.z + _e6999);
                                                        let _e7004 = (_e401 + vec3<f32>(_e6997, _e6998, _e6999));
                                                        let _e7019 = ((((_e7004.x * _e7004.x) + (_e7004.y * _e7004.y)) + (_e7004.z * _e7004.z)) - (2f * (((_e7004.x * _e6643) + (_e7004.y * _e6646)) + (_e7004.z * _e6649))));
                                                        let _e7021 = (_e7004.x - _e6643);
                                                        let _e7022 = (_e7004.y - _e6646);
                                                        let _e7023 = (_e7004.z - _e6649);
                                                        let _e7031 = ((_e7019 + _e6658) / (sqrt((((_e7021 * _e7021) + (_e7022 * _e7022)) + (_e7023 * _e7023))) + _e6652));
                                                        let _e7035 = (1f - (abs((_e7031 - 25000f)) * 0.00006666667f));
                                                        if (_e7035 != _e7035) {
                                                            phi_24149_ = true;
                                                        } else {
                                                            phi_24149_ = (0f >= _e7035);
                                                        }
                                                        let _e7039 = phi_24149_;
                                                        let _e7041 = -(_e7031);
                                                        let _e7044 = (exp((_e7041 / _e6673)) * _e6994.member_1);
                                                        let _e7047 = (exp((_e7041 / _e6676)) * _e6994.member_1);
                                                        let _e7049 = (_e7044 * 0.5f);
                                                        let _e7050 = (_e7047 * 0.5f);
                                                        let _e7051 = ((select(_e7035, 0f, _e7039) * _e6994.member_1) * 0.5f);
                                                        let _e7053 = (_e6891.x + _e7049);
                                                        let _e7055 = (_e6891.y + _e7050);
                                                        let _e7057 = (_e6891.z + _e7051);
                                                        let _e7062 = (_e6692 * 1.1f);
                                                        let _e7063 = (_e7062 * _e7055);
                                                        let _e7082 = (_e6692 * _e7047);
                                                        if (_e6712 == 4294967295u) {
                                                            phi_10910_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            let _e7090 = (_e7000 - _e6643);
                                                            let _e7091 = (_e7001 - _e6646);
                                                            let _e7092 = (_e7002 - _e6649);
                                                            let _e7099 = (1f / sqrt((((_e7090 * _e7090) + (_e7091 * _e7091)) + (_e7092 * _e7092))));
                                                            let _e7107 = ((((_e7090 * _e7099) * _e6664) + ((_e7091 * _e7099) * _e6667)) + ((_e7092 * _e7099) * _e6670));
                                                            if (_e7031 != _e7031) {
                                                                phi_24276_ = true;
                                                            } else {
                                                                phi_24276_ = (0f >= _e7031);
                                                            }
                                                            let _e7111 = phi_24276_;
                                                            let _e7115 = sqrt((select(_e7031, 0f, _e7111) / (_e6655 - _e6652)));
                                                            if (_e7115 != _e7115) {
                                                                phi_24291_ = true;
                                                            } else {
                                                                phi_24291_ = (1f <= _e7115);
                                                            }
                                                            let _e7119 = phi_24291_;
                                                            let _e7121 = (select(_e7115, 1f, _e7119) * 31f);
                                                            let _e7123 = select(_e7107, -1f, (_e7107 < -1f));
                                                            let _e7127 = ((select(_e7123, 1f, (_e7123 > 1f)) + 1f) * 15.5f);
                                                            let _e7132 = select(select(u32(_e7121), 0u, (_e7121 < 0f)), 4294967295u, (_e7121 > 4294967000f));
                                                            let _e7134 = select(_e7132, 30u, (30u < _e7132));
                                                            let _e7139 = select(select(u32(_e7127), 0u, (_e7127 < 0f)), 4294967295u, (_e7127 > 4294967000f));
                                                            let _e7141 = select(_e7139, 30u, (30u < _e7139));
                                                            let _e7143 = (_e7121 - f32(_e7134));
                                                            let _e7145 = (_e7127 - f32(_e7141));
                                                            let _e7149 = (_e6712 + (3u * ((_e7134 * 32u) + _e7141)));
                                                            let _e7154 = (_e6712 + (3u * (((_e7134 + 1u) * 32u) + _e7141)));
                                                            let _e7156 = global_6.member[_e7149];
                                                            let _e7159 = global_6.member[(_e7149 + 1u)];
                                                            let _e7162 = global_6.member[(_e7149 + 2u)];
                                                            let _e7163 = (1f - _e7145);
                                                            let _e7169 = global_6.member[(_e7149 + 3u)];
                                                            let _e7172 = global_6.member[(_e7149 + 4u)];
                                                            let _e7175 = global_6.member[(_e7149 + 5u)];
                                                            let _e7182 = (1f - _e7143);
                                                            let _e7187 = global_6.member[_e7154];
                                                            let _e7190 = global_6.member[(_e7154 + 1u)];
                                                            let _e7193 = global_6.member[(_e7154 + 2u)];
                                                            let _e7199 = global_6.member[(_e7154 + 3u)];
                                                            let _e7202 = global_6.member[(_e7154 + 4u)];
                                                            let _e7205 = global_6.member[(_e7154 + 5u)];
                                                            phi_10910_ = vec3<f32>(((((_e7156 * _e7163) + (_e7169 * _e7145)) * _e7182) + (((_e7187 * _e7163) + (_e7199 * _e7145)) * _e7143)), ((((_e7159 * _e7163) + (_e7172 * _e7145)) * _e7182) + (((_e7190 * _e7163) + (_e7202 * _e7145)) * _e7143)), ((((_e7162 * _e7163) + (_e7175 * _e7145)) * _e7182) + (((_e7193 * _e7163) + (_e7205 * _e7145)) * _e7143)));
                                                        }
                                                        let _e7220 = phi_10910_;
                                                        let _e7234 = (_e7019 + _e6661);
                                                        if (_e7234 > 0f) {
                                                            let _e7243 = ((((_e7000 - _e6643) * _e6664) + ((_e7001 - _e6646) * _e6667)) + ((_e7002 - _e6649) * _e6670));
                                                            let _e7245 = ((_e7243 * _e7243) - _e7234);
                                                            if (_e7243 >= 0f) {
                                                                phi_10984_ = vec3<f32>();
                                                                phi_10985_ = true;
                                                            } else {
                                                                let _e7247 = (_e7245 <= 0f);
                                                                if _e7247 {
                                                                    phi_10983_ = vec3<f32>();
                                                                } else {
                                                                    let _e7250 = (-(_e7243) - sqrt(_e7245));
                                                                    phi_10983_ = vec3<f32>((_e7000 + (_e6664 * _e7250)), (_e7001 + (_e6667 * _e7250)), (_e7002 + (_e6670 * _e7250)));
                                                                }
                                                                let _e7259 = phi_10983_;
                                                                phi_10984_ = _e7259;
                                                                phi_10985_ = _e7247;
                                                            }
                                                            let _e7261 = phi_10984_;
                                                            let _e7263 = phi_10985_;
                                                            phi_10990_ = _e7261;
                                                            phi_10991_ = select(true, false, _e7263);
                                                        } else {
                                                            phi_10990_ = _e7004;
                                                            phi_10991_ = true;
                                                        }
                                                        let _e7266 = phi_10990_;
                                                        let _e7268 = phi_10991_;
                                                        if _e7268 {
                                                            let _e7270 = (_e7266.x - _e6643);
                                                            let _e7272 = (_e7266.y - _e6646);
                                                            let _e7274 = (_e7266.z - _e6649);
                                                            let _e7280 = sqrt((((_e7270 * _e7270) + (_e7272 * _e7272)) + (_e7274 * _e7274)));
                                                            let _e7281 = (1f / _e7280);
                                                            let _e7299 = ((((((_e7266.x * _e7266.x) + (_e7266.y * _e7266.y)) + (_e7266.z * _e7266.z)) - (2f * (((_e7266.x * _e6643) + (_e7266.y * _e6646)) + (_e7266.z * _e6649)))) + _e6658) / (_e7280 + _e6652));
                                                            if (_e7299 != _e7299) {
                                                                phi_24415_ = true;
                                                            } else {
                                                                phi_24415_ = (0f >= _e7299);
                                                            }
                                                            let _e7303 = phi_24415_;
                                                            let _e7307 = sqrt((select(_e7299, 0f, _e7303) / (_e6655 - _e6652)));
                                                            if (_e7307 != _e7307) {
                                                                phi_24430_ = true;
                                                            } else {
                                                                phi_24430_ = (1f <= _e7307);
                                                            }
                                                            let _e7311 = phi_24430_;
                                                            let _e7313 = (select(_e7307, 1f, _e7311) * 63f);
                                                            let _e7318 = ((((_e7270 * _e7281) * _e6664) + ((_e7272 * _e7281) * _e6667)) + ((_e7274 * _e7281) * _e6670));
                                                            let _e7320 = select(_e7318, -1f, (_e7318 < -1f));
                                                            let _e7324 = ((select(_e7320, 1f, (_e7320 > 1f)) + 1f) * 127.5f);
                                                            let _e7329 = select(select(u32(_e7313), 0u, (_e7313 < 0f)), 4294967295u, (_e7313 > 4294967000f));
                                                            let _e7331 = select(_e7329, 62u, (62u < _e7329));
                                                            let _e7336 = select(select(u32(_e7324), 0u, (_e7324 < 0f)), 4294967295u, (_e7324 > 4294967000f));
                                                            let _e7338 = select(_e7336, 254u, (254u < _e7336));
                                                            let _e7340 = (_e7313 - f32(_e7331));
                                                            let _e7342 = (_e7324 - f32(_e7338));
                                                            let _e7346 = (_e6708 + (3u * ((_e7331 * 256u) + _e7338)));
                                                            let _e7351 = (_e6708 + (3u * (((_e7331 + 1u) * 256u) + _e7338)));
                                                            let _e7353 = global_6.member[_e7346];
                                                            let _e7356 = global_6.member[(_e7346 + 1u)];
                                                            let _e7359 = global_6.member[(_e7346 + 2u)];
                                                            let _e7360 = (1f - _e7342);
                                                            let _e7366 = global_6.member[(_e7346 + 3u)];
                                                            let _e7369 = global_6.member[(_e7346 + 4u)];
                                                            let _e7372 = global_6.member[(_e7346 + 5u)];
                                                            let _e7379 = (1f - _e7340);
                                                            let _e7384 = global_6.member[_e7351];
                                                            let _e7387 = global_6.member[(_e7351 + 1u)];
                                                            let _e7390 = global_6.member[(_e7351 + 2u)];
                                                            let _e7396 = global_6.member[(_e7351 + 3u)];
                                                            let _e7399 = global_6.member[(_e7351 + 4u)];
                                                            let _e7402 = global_6.member[(_e7351 + 5u)];
                                                            phi_11282_ = vec3<f32>(((((_e7353 * _e7360) + (_e7366 * _e7342)) * _e7379) + (((_e7384 * _e7360) + (_e7396 * _e7342)) * _e7340)), ((((_e7356 * _e7360) + (_e7369 * _e7342)) * _e7379) + (((_e7387 * _e7360) + (_e7399 * _e7342)) * _e7340)), ((((_e7359 * _e7360) + (_e7372 * _e7342)) * _e7379) + (((_e7390 * _e7360) + (_e7402 * _e7342)) * _e7340)));
                                                        } else {
                                                            phi_11282_ = vec3<f32>(0f, 0f, 0f);
                                                        }
                                                        let _e7417 = phi_11282_;
                                                        let _e7418 = (vec3<f32>(_e7053, _e7055, _e7057) + _e7417);
                                                        let _e7424 = (_e7062 * _e7418.y);
                                                        let _e7436 = exp(-((((_e6682 * _e7418.x) + _e7424) + (_e6698 * _e7418.z))));
                                                        let _e7438 = exp(-((((_e6685 * _e7418.x) + _e7424) + (_e6701 * _e7418.z))));
                                                        let _e7440 = exp(-((((_e6688 * _e7418.x) + _e7424) + (_e6704 * _e7418.z))));
                                                        phi_10460_ = vec3<f32>((_e6885.x + ((exp(-((((_e6682 * _e7053) + _e7063) + (_e6698 * _e7057)))) * ((_e6682 * _e7044) + _e7082)) * _e7220.x)), (_e6885.y + ((exp(-((((_e6685 * _e7053) + _e7063) + (_e6701 * _e7057)))) * ((_e6685 * _e7044) + _e7082)) * _e7220.y)), (_e6885.z + ((exp(-((((_e6688 * _e7053) + _e7063) + (_e6704 * _e7057)))) * ((_e6688 * _e7044) + _e7082)) * _e7220.z)));
                                                        phi_10463_ = vec3<f32>((_e6887.x + (_e7436 * _e7047)), (_e6887.y + (_e7438 * _e7047)), (_e6887.z + (_e7440 * _e7047)));
                                                        phi_10465_ = vec3<f32>((_e6889.x + (_e7436 * _e7044)), (_e6889.y + (_e7438 * _e7044)), (_e6889.z + (_e7440 * _e7044)));
                                                        phi_10467_ = vec3<f32>((_e7053 + _e7049), (_e7055 + _e7050), (_e7057 + _e7051));
                                                        phi_10469_ = (_e6893 + 1u);
                                                    } else {
                                                        phi_10460_ = vec3<f32>();
                                                        phi_10463_ = vec3<f32>();
                                                        phi_10465_ = vec3<f32>();
                                                        phi_10467_ = vec3<f32>();
                                                        phi_10469_ = u32();
                                                    }
                                                    let _e7467 = phi_10460_;
                                                    let _e7469 = phi_10463_;
                                                    let _e7471 = phi_10465_;
                                                    let _e7473 = phi_10467_;
                                                    let _e7475 = phi_10469_;
                                                    phi_10459_ = _e7467;
                                                    phi_10462_ = _e7469;
                                                    phi_10464_ = _e7471;
                                                    phi_10466_ = _e7473;
                                                    phi_10468_ = _e7475;
                                                    phi_30734_ = _e6634;
                                                    if !(_e6894) {
                                                        break;
                                                    }
                                                }
                                                let _e7478 = phi_30734_;
                                                phi_35216_ = _e7478;
                                                phi_19906_ = type_49();
                                                phi_19905_ = u32();
                                                phi_19904_ = vec3<f32>();
                                                phi_19903_ = bool();
                                                phi_19902_ = bool();
                                                phi_19901_ = bool();
                                                phi_19900_ = bool();
                                                phi_19899_ = vec3<f32>();
                                                phi_19897_ = u32();
                                                phi_19895_ = type_14();
                                                phi_19893_ = vec3<f32>();
                                                phi_19892_ = f32();
                                                phi_19891_ = f32();
                                                phi_19890_ = f32();
                                                phi_19888_ = vec3<f32>();
                                                if _e7478 {
                                                    break;
                                                }
                                                let _e7480 = local_54;
                                                let _e7481 = (_e7480 * vec3<f32>(_e6682, _e6685, _e6688));
                                                let _e7483 = local_55;
                                                let _e7484 = (_e7483 * _e6692);
                                                let _e7489 = (((_e385.x * _e6664) + (_e385.y * _e6667)) + (_e385.z * _e6670));
                                                let _e7491 = (1f + (_e7489 * _e7489));
                                                let _e7492 = (0.059683103f * _e7491);
                                                let _e7499 = (_e6695 * _e6695);
                                                let _e7510 = ((0.119366206f * ((1f - _e7499) * _e7491)) / ((2f + _e7499) * pow(((1f + _e7499) - ((2f * _e6695) * _e7489)), 1.5f)));
                                                let _e7521 = local_56;
                                                let _e7525 = local_57;
                                                let _e7529 = local_58;
                                                let _e7538 = local_59;
                                                let _e7545 = local_60;
                                                let _e7547 = ((_e6692 * 1.1f) * _e7545.y);
                                                let _e7552 = local_61;
                                                phi_33435_ = _e7478;
                                                phi_11339_ = type_53(vec3<f32>(exp(-((((_e6682 * _e7538.x) + _e7547) + (_e6698 * _e7552.z)))), exp(-((((_e6685 * _e7538.x) + _e7547) + (_e6701 * _e7552.z)))), exp(-((((_e6688 * _e7538.x) + _e7547) + (_e6704 * _e7552.z))))), (vec3<f32>(((((_e7481.x * _e7492) + (_e7484.x * _e7510)) + _e7521.x) * _e6679), ((((_e7481.y * _e7492) + (_e7484.y * _e7510)) + _e7525.y) * _e6679), ((((_e7481.z * _e7492) + (_e7484.z * _e7510)) + _e7529.z) * _e6679)) * 0.38317f));
                                            } else {
                                                phi_33435_ = _e6634;
                                                phi_11339_ = type_53();
                                            }
                                            let _e7569 = phi_33435_;
                                            let _e7571 = phi_11339_;
                                            phi_33434_ = _e7569;
                                            phi_11341_ = _e7571;
                                            phi_11342_ = select(true, false, _e6876);
                                        } else {
                                            phi_33434_ = _e6634;
                                            phi_11341_ = type_53();
                                            phi_11342_ = true;
                                        }
                                        let _e7574 = phi_33434_;
                                        let _e7576 = phi_11341_;
                                        let _e7578 = phi_11342_;
                                        if _e7578 {
                                            phi_11347_ = type_53(vec3<f32>(1f, 1f, 1f), vec3<f32>(0f, 0f, 0f));
                                        } else {
                                            phi_11347_ = _e7576;
                                        }
                                        let _e7580 = phi_11347_;
                                        let _e7593 = local_62;
                                        let _e7597 = local_63;
                                        let _e7601 = local_64;
                                        phi_33432_ = _e7574;
                                        phi_11379_ = vec3<f32>((_e7593.x + (_e381.x * _e7580.member_1.x)), (_e7597.y + (_e381.y * _e7580.member_1.y)), (_e7601.z + (_e381.z * _e7580.member_1.z)));
                                        phi_11380_ = (_e381 * _e7580.member);
                                    } else {
                                        phi_33432_ = _e6634;
                                        let _e14511 = local_116;
                                        phi_11379_ = _e14511;
                                        phi_11380_ = _e381;
                                    }
                                    let _e7607 = phi_33432_;
                                    let _e7609 = phi_11379_;
                                    let _e7611 = phi_11380_;
                                    phi_33431_ = _e7607;
                                    phi_11381_ = _e7609;
                                    phi_11382_ = _e7611;
                                }
                                let _e7613 = phi_33431_;
                                let _e7615 = phi_11381_;
                                let _e7617 = phi_11382_;
                                let _e7620 = (_e385.x * _e1156.member);
                                let _e7622 = (_e385.y * _e1156.member);
                                let _e7624 = (_e385.z * _e1156.member);
                                let _e7626 = (_e401.x + _e7620);
                                let _e7628 = (_e401.y + _e7622);
                                let _e7630 = (_e401.z + _e7624);
                                let _e7635 = global_2.member[_e1156.member_1];
                                phi_11413_ = 0i;
                                phi_11416_ = _e7635.member_6.y;
                                loop {
                                    let _e7639 = phi_11413_;
                                    let _e7641 = phi_11416_;
                                    local_65 = _e7641;
                                    local_129 = _e7641;
                                    if (_e7639 < 4i) {
                                        let _e7644 = global_4.member[_e7641];
                                        let _e7646 = (_e7644.member != 6u);
                                        if _e7646 {
                                            phi_11485_ = i32();
                                            phi_11486_ = u32();
                                        } else {
                                            let _e7650 = (abs(_e7626) / _e7644.member_6.z);
                                            let _e7661 = (abs(_e7630) / _e7644.member_6.w);
                                            if ((select(select(u32(_e7650), 0u, (_e7650 < 0f)), 4294967295u, (_e7650 > 4294967000f)) % 2u) == 0u) {
                                                phi_11470_ = (_e7626 > 0f);
                                            } else {
                                                phi_11470_ = (_e7626 <= 0f);
                                            }
                                            let _e7672 = phi_11470_;
                                            if ((select(select(u32(_e7661), 0u, (_e7661 < 0f)), 4294967295u, (_e7661 > 4294967000f)) % 2u) == 0u) {
                                                phi_11476_ = (_e7630 > 0f);
                                            } else {
                                                phi_11476_ = (_e7630 <= 0f);
                                            }
                                            let _e7676 = phi_11476_;
                                            if (_e7672 != _e7676) {
                                                phi_11483_ = _e7644.member_1;
                                            } else {
                                                phi_11483_ = _e7644.member_2;
                                            }
                                            let _e7681 = phi_11483_;
                                            phi_11485_ = (_e7639 + 1i);
                                            phi_11486_ = _e7681;
                                        }
                                        let _e7684 = phi_11485_;
                                        let _e7686 = phi_11486_;
                                        phi_11414_ = _e7684;
                                        phi_11417_ = _e7686;
                                        phi_11488_ = select(true, false, _e7646);
                                    } else {
                                        phi_11414_ = i32();
                                        phi_11417_ = u32();
                                        phi_11488_ = false;
                                    }
                                    let _e7689 = phi_11414_;
                                    let _e7691 = phi_11417_;
                                    let _e7693 = phi_11488_;
                                    phi_11413_ = _e7689;
                                    phi_11416_ = _e7691;
                                    if !(_e7693) {
                                        break;
                                    }
                                }
                                let _e7696 = local_65;
                                let _e7698 = global_4.member[_e7696];
                                let _e7700 = (_e7698.member == 4u);
                                if _e7700 {
                                    phi_35317_ = _e373;
                                    phi_35219_ = _e7613;
                                    phi_17059_ = u32();
                                    phi_17060_ = vec3<f32>();
                                    phi_17061_ = vec3<f32>();
                                    phi_17062_ = vec3<f32>();
                                    phi_17063_ = vec3<f32>();
                                    phi_17064_ = bool();
                                    phi_17065_ = vec3<f32>();
                                    phi_17066_ = vec3<f32>();
                                    phi_17067_ = u32();
                                    phi_17068_ = bool();
                                    phi_17069_ = f32();
                                    phi_17070_ = bool();
                                    phi_17071_ = vec3<f32>();
                                    phi_17072_ = false;
                                    phi_17073_ = false;
                                    phi_17074_ = false;
                                } else {
                                    let _e7701 = -(_e385.x);
                                    let _e7702 = -(_e385.y);
                                    let _e7703 = -(_e385.z);
                                    let _e7704 = -(_e385);
                                    if ((((_e1156.member_2.x * _e7701) + (_e1156.member_2.y * _e7702)) + (_e1156.member_2.z * _e7703)) < 0f) {
                                        phi_11533_ = -(_e1156.member_2);
                                    } else {
                                        phi_11533_ = _e1156.member_2;
                                    }
                                    let _e7717 = phi_11533_;
                                    if ((((_e1156.member_3.x * _e7701) + (_e1156.member_3.y * _e7702)) + (_e1156.member_3.z * _e7703)) < 0f) {
                                        phi_11550_ = -(_e1156.member_3);
                                    } else {
                                        phi_11550_ = _e1156.member_3;
                                    }
                                    let _e7730 = phi_11550_;
                                    let _e7733 = global_1.member.member_4[2u];
                                    let _e7734 = abs(_e7626);
                                    let _e7735 = abs(_e7628);
                                    let _e7736 = abs(_e7630);
                                    let _e7738 = select(_e7735, _e7734, (_e7734 > _e7735));
                                    let _e7742 = (0.00002f * (1f + select(_e7736, _e7738, (_e7738 > _e7736))));
                                    if (_e7733 != _e7733) {
                                        phi_24606_ = true;
                                    } else {
                                        phi_24606_ = (_e7742 >= _e7733);
                                    }
                                    let _e7746 = phi_24606_;
                                    let _e7748 = (select(_e7733, _e7742, _e7746) * 2f);
                                    let _e7755 = (_e7626 + (_e7717.x * _e7748));
                                    let _e7756 = (_e7628 + (_e7717.y * _e7748));
                                    let _e7757 = (_e7630 + (_e7717.z * _e7748));
                                    let _e7758 = vec3<f32>(_e7755, _e7756, _e7757);
                                    if _e395 {
                                        let _e7761 = global_1.member.member_4[0u];
                                        phi_11581_ = _e7761;
                                    } else {
                                        phi_11581_ = 0f;
                                    }
                                    let _e7763 = phi_11581_;
                                    let _e7766 = select(-1f, 1f, (_e7730.z >= 0f));
                                    let _e7768 = (-1f / (_e7766 + _e7730.z));
                                    let _e7772 = ((_e7730.x * _e7730.y) * _e7768);
                                    let _e7776 = (1f + (((_e7766 * _e7730.x) * _e7730.x) * _e7768));
                                    let _e7777 = (_e7766 * _e7772);
                                    let _e7779 = (-(_e7766) * _e7730.x);
                                    let _e7782 = (_e7766 + ((_e7730.y * _e7730.y) * _e7768));
                                    let _e7783 = -(_e7730.y);
                                    if (_e7698.member == 0u) {
                                        phi_32075_ = vec3<f32>(0f, 0f, 0f);
                                        phi_31884_ = 1.5f;
                                        phi_31806_ = vec3<f32>(0f, 0f, 0f);
                                        phi_31730_ = vec3<f32>(_e7698.member_4.x, _e7698.member_4.y, _e7698.member_4.z);
                                        phi_31652_ = 0u;
                                        phi_31571_ = 1f;
                                        phi_31495_ = 0u;
                                        phi_31480_ = _e373;
                                        phi_12272_ = vec3<f32>();
                                        phi_12273_ = vec3<f32>();
                                        phi_12274_ = u32();
                                        phi_12275_ = vec3<f32>();
                                        phi_12276_ = true;
                                        phi_12277_ = false;
                                        phi_12278_ = false;
                                        phi_12279_ = false;
                                    } else {
                                        if (_e7698.member == 1u) {
                                            let _e8332 = (_e7698.member_6.y * _e7698.member_6.y);
                                            if (_e8332 != _e8332) {
                                                phi_25298_ = true;
                                            } else {
                                                phi_25298_ = (_e7763 >= _e8332);
                                            }
                                            let _e8336 = phi_25298_;
                                            let _e8337 = select(_e8332, _e7763, _e8336);
                                            let _e8338 = (_e8337 >= 0.001f);
                                            if _e8338 {
                                                phi_32132_ = vec3<f32>(_e7698.member_4.x, _e7698.member_4.y, _e7698.member_4.z);
                                                phi_31927_ = _e7698.member_6.x;
                                                phi_31782_ = vec3<f32>(0f, 0f, 0f);
                                                phi_31474_ = _e373;
                                                phi_11656_ = vec3<f32>();
                                                phi_11657_ = true;
                                            } else {
                                                let _e8340 = ((_e373 * 747796405u) + 2891336453u);
                                                let _e8348 = (((_e8340 >> bitcast<u32>((((_e8340 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e8340) * 277803737u);
                                                let _e8351 = ((_e8348 >> bitcast<u32>(22i)) ^ _e8348);
                                                let _e8361 = abs((((_e7701 * _e7730.x) + (_e7702 * _e7730.y)) + (_e7703 * _e7730.z)));
                                                let _e8364 = (1f / _e7698.member_6.x);
                                                let _e8368 = ((_e8364 * _e8364) * (1f - (_e8361 * _e8361)));
                                                if (_e8368 >= 1f) {
                                                    phi_25338_ = 1f;
                                                } else {
                                                    let _e8371 = sqrt((1f - _e8368));
                                                    if (_e8361 != _e8361) {
                                                        phi_25349_ = true;
                                                    } else {
                                                        phi_25349_ = (_e8371 <= _e8361);
                                                    }
                                                    let _e8375 = phi_25349_;
                                                    let _e8380 = pow(((1f - _e7698.member_6.x) / (1f + _e7698.member_6.x)), 2f);
                                                    phi_25338_ = (_e8380 + ((1f - _e8380) * pow((1f - select(_e8361, _e8371, _e8375)), 5f)));
                                                }
                                                let _e8387 = phi_25338_;
                                                let _e8388 = ((f32((_e8351 >> bitcast<u32>(8i))) * 0.000000059604645f) < _e8387);
                                                if _e8388 {
                                                    let _e8401 = (((_e7730.x * _e385.x) + (_e7730.y * _e385.y)) + (_e7730.z * _e385.z));
                                                    phi_31783_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_11654_ = (_e385 - vec3<f32>(((_e7730.x * 2f) * _e8401), ((_e7730.y * 2f) * _e8401), ((_e7730.z * 2f) * _e8401)));
                                                } else {
                                                    phi_31783_ = vec3<f32>(_e7698.member_4.x, _e7698.member_4.y, _e7698.member_4.z);
                                                    phi_11654_ = vec3<f32>();
                                                }
                                                let _e8408 = phi_31783_;
                                                let _e8410 = phi_11654_;
                                                phi_32132_ = vec3<f32>(0f, 0f, 0f);
                                                phi_31927_ = 1.5f;
                                                phi_31782_ = _e8408;
                                                phi_31474_ = _e8351;
                                                phi_11656_ = _e8410;
                                                phi_11657_ = select(true, false, _e8388);
                                            }
                                            let _e8420 = phi_32132_;
                                            let _e8422 = phi_31927_;
                                            let _e8424 = phi_31782_;
                                            let _e8426 = phi_31474_;
                                            let _e8428 = phi_11656_;
                                            let _e8430 = phi_11657_;
                                            phi_32078_ = _e8420;
                                            phi_31887_ = _e8422;
                                            phi_31809_ = vec3<f32>(0f, 0f, 0f);
                                            phi_31733_ = _e8424;
                                            phi_31655_ = select(0u, 1u, _e8338);
                                            phi_31574_ = select(1f, _e8337, _e8338);
                                            phi_31498_ = select(0u, 2u, _e8338);
                                            phi_31425_ = _e8426;
                                            phi_12142_ = _e7758;
                                            phi_12143_ = _e8428;
                                            phi_12144_ = vec3<f32>(1f, 1f, 1f);
                                            phi_12145_ = _e8430;
                                        } else {
                                            if (_e7698.member == 3u) {
                                                phi_31428_ = _e373;
                                                phi_11821_ = vec3<f32>();
                                                phi_11822_ = vec3<f32>();
                                                phi_11823_ = vec3<f32>();
                                                phi_11824_ = true;
                                                phi_11825_ = false;
                                            } else {
                                                let _e7787 = (_e7698.member == 5u);
                                                if _e7787 {
                                                    phi_31429_ = _e373;
                                                    phi_11817_ = vec3<f32>();
                                                    phi_11818_ = vec3<f32>();
                                                    phi_11819_ = vec3<f32>();
                                                    phi_11820_ = false;
                                                } else {
                                                    let _e7788 = (_e7698.member == 2u);
                                                    if _e7788 {
                                                        let _e7793 = (((_e385.x * _e1156.member_3.x) + (_e385.y * _e1156.member_3.y)) + (_e385.z * _e1156.member_3.z));
                                                        let _e7794 = (_e7793 > 0f);
                                                        if _e7794 {
                                                            phi_11693_ = -(_e1156.member_3);
                                                        } else {
                                                            phi_11693_ = _e1156.member_3;
                                                        }
                                                        let _e7797 = phi_11693_;
                                                        if _e7794 {
                                                            phi_11699_ = _e7698.member_6.x;
                                                        } else {
                                                            phi_11699_ = (1f / _e7698.member_6.x);
                                                        }
                                                        let _e7802 = phi_11699_;
                                                        let _e7803 = abs(_e7793);
                                                        let _e7804 = (_e7802 * _e7802);
                                                        let _e7807 = (_e7804 * (1f - (_e7803 * _e7803)));
                                                        if (_e7807 >= 1f) {
                                                            phi_24668_ = 1f;
                                                        } else {
                                                            let _e7810 = sqrt((1f - _e7807));
                                                            if (_e7803 != _e7803) {
                                                                phi_24679_ = true;
                                                            } else {
                                                                phi_24679_ = (_e7810 <= _e7803);
                                                            }
                                                            let _e7814 = phi_24679_;
                                                            let _e7819 = pow(((1f - _e7698.member_6.x) / (1f + _e7698.member_6.x)), 2f);
                                                            phi_24668_ = (_e7819 + ((1f - _e7819) * pow((1f - select(_e7803, _e7810, _e7814)), 5f)));
                                                        }
                                                        let _e7826 = phi_24668_;
                                                        let _e7834 = (((_e385.x * _e7797.x) + (_e385.y * _e7797.y)) + (_e385.z * _e7797.z));
                                                        let _e7838 = (1f - (_e7804 * (1f - (_e7834 * _e7834))));
                                                        if (_e7838 > 0f) {
                                                            let _e7841 = ((_e373 * 747796405u) + 2891336453u);
                                                            let _e7849 = (((_e7841 >> bitcast<u32>((((_e7841 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e7841) * 277803737u);
                                                            let _e7852 = ((_e7849 >> bitcast<u32>(22i)) ^ _e7849);
                                                            let _e7857 = ((f32((_e7852 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e7826);
                                                            if _e7857 {
                                                                let _e7867 = sqrt(_e7838);
                                                                let _e7871 = (((_e385.x - (_e7797.x * _e7834)) * _e7802) - (_e7797.x * _e7867));
                                                                let _e7872 = (((_e385.y - (_e7797.y * _e7834)) * _e7802) - (_e7797.y * _e7867));
                                                                let _e7873 = (((_e385.z - (_e7797.z * _e7834)) * _e7802) - (_e7797.z * _e7867));
                                                                let _e7880 = (1f / sqrt((((_e7871 * _e7871) + (_e7872 * _e7872)) + (_e7873 * _e7873))));
                                                                phi_11789_ = vec3<f32>((_e7626 - (_e7797.x * _e7748)), (_e7628 - (_e7797.y * _e7748)), (_e7630 - (_e7797.z * _e7748)));
                                                                phi_11790_ = vec3<f32>((_e7871 * _e7880), (_e7872 * _e7880), (_e7873 * _e7880));
                                                                phi_11791_ = vec3<f32>(_e7698.member_4.x, _e7698.member_4.y, _e7698.member_4.z);
                                                            } else {
                                                                phi_11789_ = vec3<f32>();
                                                                phi_11790_ = vec3<f32>();
                                                                phi_11791_ = vec3<f32>();
                                                            }
                                                            let _e7898 = phi_11789_;
                                                            let _e7900 = phi_11790_;
                                                            let _e7902 = phi_11791_;
                                                            phi_31432_ = _e7852;
                                                            phi_11793_ = _e7898;
                                                            phi_11794_ = _e7900;
                                                            phi_11795_ = _e7902;
                                                            phi_11796_ = select(true, false, _e7857);
                                                        } else {
                                                            phi_31432_ = _e373;
                                                            phi_11793_ = vec3<f32>();
                                                            phi_11794_ = vec3<f32>();
                                                            phi_11795_ = vec3<f32>();
                                                            phi_11796_ = true;
                                                        }
                                                        let _e7905 = phi_31432_;
                                                        let _e7907 = phi_11793_;
                                                        let _e7909 = phi_11794_;
                                                        let _e7911 = phi_11795_;
                                                        let _e7913 = phi_11796_;
                                                        if _e7913 {
                                                            let _e7921 = (((_e1156.member_3.x * _e385.x) + (_e1156.member_3.y * _e385.y)) + (_e1156.member_3.z * _e385.z));
                                                            phi_11811_ = vec3<f32>((_e7626 + (_e7797.x * _e7748)), (_e7628 + (_e7797.y * _e7748)), (_e7630 + (_e7797.z * _e7748)));
                                                            phi_11812_ = (_e385 - vec3<f32>(((_e1156.member_3.x * 2f) * _e7921), ((_e1156.member_3.y * 2f) * _e7921), ((_e1156.member_3.z * 2f) * _e7921)));
                                                        } else {
                                                            phi_11811_ = _e7907;
                                                            phi_11812_ = _e7909;
                                                        }
                                                        let _e7935 = phi_11811_;
                                                        let _e7937 = phi_11812_;
                                                        phi_31430_ = _e7905;
                                                        phi_11814_ = _e7935;
                                                        phi_11815_ = _e7937;
                                                        phi_11816_ = select(_e7911, vec3<f32>(1f, 1f, 1f), vec3(_e7913));
                                                    } else {
                                                        phi_31430_ = _e373;
                                                        phi_11814_ = vec3<f32>();
                                                        phi_11815_ = vec3<f32>();
                                                        phi_11816_ = vec3<f32>();
                                                    }
                                                    let _e7941 = phi_31430_;
                                                    let _e7943 = phi_11814_;
                                                    let _e7945 = phi_11815_;
                                                    let _e7947 = phi_11816_;
                                                    phi_31429_ = _e7941;
                                                    phi_11817_ = _e7943;
                                                    phi_11818_ = _e7945;
                                                    phi_11819_ = _e7947;
                                                    phi_11820_ = _e7788;
                                                }
                                                let _e7949 = phi_31429_;
                                                let _e7951 = phi_11817_;
                                                let _e7953 = phi_11818_;
                                                let _e7955 = phi_11819_;
                                                let _e7957 = phi_11820_;
                                                phi_31428_ = _e7949;
                                                phi_11821_ = _e7951;
                                                phi_11822_ = _e7953;
                                                phi_11823_ = _e7955;
                                                phi_11824_ = _e7787;
                                                phi_11825_ = _e7957;
                                            }
                                            let _e7959 = phi_31428_;
                                            let _e7961 = phi_11821_;
                                            let _e7963 = phi_11822_;
                                            let _e7965 = phi_11823_;
                                            let _e7967 = phi_11824_;
                                            let _e7969 = phi_11825_;
                                            if _e7967 {
                                                let _e7972 = (_e7698.member_6.y * _e7698.member_6.y);
                                                if (_e7972 != _e7972) {
                                                    phi_24752_ = true;
                                                } else {
                                                    phi_24752_ = (_e7763 >= _e7972);
                                                }
                                                let _e7976 = phi_24752_;
                                                let _e7977 = select(_e7972, _e7763, _e7976);
                                                let _e7989 = select(2u, 0u, (_e7698.member == 5u));
                                                let _e7990 = (_e7977 < 0.001f);
                                                if _e7990 {
                                                    let _e7998 = (((_e7730.x * _e385.x) + (_e7730.y * _e385.y)) + (_e7730.z * _e385.z));
                                                    let _e8009 = abs((((_e7701 * _e7730.x) + (_e7702 * _e7730.y)) + (_e7703 * _e7730.z)));
                                                    if (_e7989 == 0u) {
                                                        let _e8291 = (1f - _e8009);
                                                        if (_e8291 != _e8291) {
                                                            phi_25283_ = true;
                                                        } else {
                                                            phi_25283_ = (0f >= _e8291);
                                                        }
                                                        let _e8295 = phi_25283_;
                                                        let _e8297 = pow(select(_e8291, 0f, _e8295), 5f);
                                                        let _e8298 = (1f - _e8297);
                                                        phi_12128_ = vec3<f32>(((_e7698.member_4.x * _e8298) + _e8297), ((_e7698.member_4.y * _e8298) + _e8297), ((_e7698.member_4.z * _e8298) + _e8297));
                                                    } else {
                                                        if (_e7989 == 1u) {
                                                            let _e8273 = (0.44444448f * (1f - (_e8009 * _e8009)));
                                                            if (_e8273 >= 1f) {
                                                                phi_25244_ = 1f;
                                                            } else {
                                                                let _e8276 = sqrt((1f - _e8273));
                                                                if (_e8009 != _e8009) {
                                                                    phi_25255_ = true;
                                                                } else {
                                                                    phi_25255_ = (_e8276 <= _e8009);
                                                                }
                                                                let _e8280 = phi_25255_;
                                                                phi_25244_ = (0.040000003f + (0.96f * pow((1f - select(_e8009, _e8276, _e8280)), 5f)));
                                                            }
                                                            let _e8287 = phi_25244_;
                                                            phi_12127_ = vec3(_e8287);
                                                        } else {
                                                            let _e8013 = select(_e8009, 0f, (_e8009 < 0f));
                                                            let _e8015 = select(_e8013, 1f, (_e8013 > 1f));
                                                            let _e8017 = (1f - (_e8015 * _e8015));
                                                            let _e8024 = ((_e7698.member_4.x * _e7698.member_4.x) - (_e7698.member_5.x * _e7698.member_5.x));
                                                            let _e8025 = ((_e7698.member_4.y * _e7698.member_4.y) - (_e7698.member_5.y * _e7698.member_5.y));
                                                            let _e8026 = ((_e7698.member_4.z * _e7698.member_4.z) - (_e7698.member_5.z * _e7698.member_5.z));
                                                            let _e8028 = (_e7698.member_4.x * (_e7698.member_5.x + _e7698.member_5.x));
                                                            let _e8030 = (_e7698.member_4.y * (_e7698.member_5.y + _e7698.member_5.y));
                                                            let _e8032 = (_e7698.member_4.z * (_e7698.member_5.z + _e7698.member_5.z));
                                                            let _e8039 = ((_e8024 * _e8024) + (_e8028 * _e8028));
                                                            let _e8040 = ((_e8025 * _e8025) + (_e8030 * _e8030));
                                                            let _e8041 = ((_e8026 * _e8026) + (_e8032 * _e8032));
                                                            let _e8046 = ((_e8017 * _e8024) / _e8039);
                                                            let _e8047 = ((_e8017 * _e8025) / _e8040);
                                                            let _e8048 = ((_e8017 * _e8026) / _e8041);
                                                            let _e8049 = (1f - _e8046);
                                                            let _e8050 = (1f - _e8047);
                                                            let _e8051 = (1f - _e8048);
                                                            let _e8053 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e8046, _e8047, _e8048));
                                                            let _e8054 = (_e8017 * _e8028);
                                                            let _e8055 = (_e8017 * _e8030);
                                                            let _e8056 = (_e8017 * _e8032);
                                                            let _e8061 = (vec3<f32>(_e8054, _e8055, _e8056) / vec3<f32>(_e8039, _e8040, _e8041));
                                                            let _e8074 = ((_e8053.x * _e8053.x) + (_e8061.x * _e8061.x));
                                                            let _e8075 = ((_e8053.y * _e8053.y) + (_e8061.y * _e8061.y));
                                                            let _e8076 = ((_e8053.z * _e8053.z) + (_e8061.z * _e8061.z));
                                                            if (_e8074 != _e8074) {
                                                                phi_24914_ = true;
                                                            } else {
                                                                phi_24914_ = (0f >= _e8074);
                                                            }
                                                            let _e8080 = phi_24914_;
                                                            let _e8082 = sqrt(select(_e8074, 0f, _e8080));
                                                            if (_e8075 != _e8075) {
                                                                phi_24929_ = true;
                                                            } else {
                                                                phi_24929_ = (0f >= _e8075);
                                                            }
                                                            let _e8086 = phi_24929_;
                                                            let _e8088 = sqrt(select(_e8075, 0f, _e8086));
                                                            if (_e8076 != _e8076) {
                                                                phi_24944_ = true;
                                                            } else {
                                                                phi_24944_ = (0f >= _e8076);
                                                            }
                                                            let _e8092 = phi_24944_;
                                                            let _e8094 = sqrt(select(_e8076, 0f, _e8092));
                                                            let _e8098 = ((_e8082 + _e8049) * 0.5f);
                                                            let _e8099 = ((_e8088 + _e8050) * 0.5f);
                                                            let _e8100 = ((_e8094 + _e8051) * 0.5f);
                                                            if (_e8098 != _e8098) {
                                                                phi_24971_ = true;
                                                            } else {
                                                                phi_24971_ = (0f >= _e8098);
                                                            }
                                                            let _e8104 = phi_24971_;
                                                            let _e8106 = sqrt(select(_e8098, 0f, _e8104));
                                                            if (_e8099 != _e8099) {
                                                                phi_24986_ = true;
                                                            } else {
                                                                phi_24986_ = (0f >= _e8099);
                                                            }
                                                            let _e8110 = phi_24986_;
                                                            let _e8112 = sqrt(select(_e8099, 0f, _e8110));
                                                            if (_e8100 != _e8100) {
                                                                phi_25001_ = true;
                                                            } else {
                                                                phi_25001_ = (0f >= _e8100);
                                                            }
                                                            let _e8116 = phi_25001_;
                                                            let _e8118 = sqrt(select(_e8100, 0f, _e8116));
                                                            let _e8119 = vec3<f32>(_e8106, _e8112, _e8118);
                                                            let _e8123 = ((_e8082 - _e8049) * 0.5f);
                                                            let _e8124 = ((_e8088 - _e8050) * 0.5f);
                                                            let _e8125 = ((_e8094 - _e8051) * 0.5f);
                                                            if (_e8123 != _e8123) {
                                                                phi_25028_ = true;
                                                            } else {
                                                                phi_25028_ = (0f >= _e8123);
                                                            }
                                                            let _e8129 = phi_25028_;
                                                            let _e8131 = sqrt(select(_e8123, 0f, _e8129));
                                                            if (_e8124 != _e8124) {
                                                                phi_25043_ = true;
                                                            } else {
                                                                phi_25043_ = (0f >= _e8124);
                                                            }
                                                            let _e8135 = phi_25043_;
                                                            let _e8137 = sqrt(select(_e8124, 0f, _e8135));
                                                            if (_e8125 != _e8125) {
                                                                phi_25058_ = true;
                                                            } else {
                                                                phi_25058_ = (0f >= _e8125);
                                                            }
                                                            let _e8141 = phi_25058_;
                                                            let _e8143 = sqrt(select(_e8125, 0f, _e8141));
                                                            if ((_e8054 / _e8039) < 0f) {
                                                                phi_12004_ = -(_e8131);
                                                            } else {
                                                                phi_12004_ = _e8131;
                                                            }
                                                            let _e8147 = phi_12004_;
                                                            if ((_e8055 / _e8040) < 0f) {
                                                                phi_12012_ = -(_e8137);
                                                            } else {
                                                                phi_12012_ = _e8137;
                                                            }
                                                            let _e8151 = phi_12012_;
                                                            if ((_e8056 / _e8041) < 0f) {
                                                                phi_12020_ = -(_e8143);
                                                            } else {
                                                                phi_12020_ = _e8143;
                                                            }
                                                            let _e8155 = phi_12020_;
                                                            let _e8159 = (_e7698.member_5.x * _e8015);
                                                            let _e8160 = (_e7698.member_5.y * _e8015);
                                                            let _e8161 = (_e7698.member_5.z * _e8015);
                                                            let _e8171 = vec3<f32>(((_e7698.member_4.x * _e8106) - (_e7698.member_5.x * _e8147)), ((_e7698.member_4.y * _e8112) - (_e7698.member_5.y * _e8151)), ((_e7698.member_4.z * _e8118) - (_e7698.member_5.z * _e8155)));
                                                            let _e8178 = ((_e7698.member_4.x * _e8147) + (_e7698.member_5.x * _e8106));
                                                            let _e8179 = ((_e7698.member_4.y * _e8151) + (_e7698.member_5.y * _e8112));
                                                            let _e8180 = ((_e7698.member_4.z * _e8155) + (_e7698.member_5.z * _e8118));
                                                            let _e8182 = vec3<f32>((_e7698.member_4.x * _e8015), (_e7698.member_4.y * _e8015), (_e7698.member_4.z * _e8015));
                                                            let _e8183 = (_e8182 - _e8119);
                                                            let _e8184 = (_e8159 - _e8147);
                                                            let _e8185 = (_e8160 - _e8151);
                                                            let _e8186 = (_e8161 - _e8155);
                                                            let _e8199 = (_e8182 + _e8119);
                                                            let _e8200 = (_e8159 + _e8147);
                                                            let _e8201 = (_e8160 + _e8151);
                                                            let _e8202 = (_e8161 + _e8155);
                                                            let _e8212 = ((_e8199.x * _e8199.x) + (_e8200 * _e8200));
                                                            let _e8213 = ((_e8199.y * _e8199.y) + (_e8201 * _e8201));
                                                            let _e8214 = ((_e8199.z * _e8199.z) + (_e8202 * _e8202));
                                                            let _e8224 = vec3(_e8015);
                                                            let _e8225 = (_e8224 - _e8171);
                                                            let _e8226 = -(vec3<f32>(_e8178, _e8179, _e8180));
                                                            let _e8242 = (_e8224 + _e8171);
                                                            let _e8252 = ((_e8242.x * _e8242.x) + (_e8178 * _e8178));
                                                            let _e8253 = ((_e8242.y * _e8242.y) + (_e8179 * _e8179));
                                                            let _e8254 = ((_e8242.z * _e8242.z) + (_e8180 * _e8180));
                                                            phi_12127_ = vec3<f32>((((((_e8183.x * _e8183.x) + (_e8184 * _e8184)) / select(0.000000000000000000000000000001f, _e8212, (_e8212 > 0.000000000000000000000000000001f))) + (((_e8225.x * _e8225.x) + (_e8226.x * _e8226.x)) / select(0.000000000000000000000000000001f, _e8252, (_e8252 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e8183.y * _e8183.y) + (_e8185 * _e8185)) / select(0.000000000000000000000000000001f, _e8213, (_e8213 > 0.000000000000000000000000000001f))) + (((_e8225.y * _e8225.y) + (_e8226.y * _e8226.y)) / select(0.000000000000000000000000000001f, _e8253, (_e8253 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e8183.z * _e8183.z) + (_e8186 * _e8186)) / select(0.000000000000000000000000000001f, _e8214, (_e8214 > 0.000000000000000000000000000001f))) + (((_e8225.z * _e8225.z) + (_e8226.z * _e8226.z)) / select(0.000000000000000000000000000001f, _e8254, (_e8254 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                        }
                                                        let _e8290 = phi_12127_;
                                                        phi_12128_ = _e8290;
                                                    }
                                                    let _e8307 = phi_12128_;
                                                    phi_12129_ = (_e385 - vec3<f32>(((_e7730.x * 2f) * _e7998), ((_e7730.y * 2f) * _e7998), ((_e7730.z * 2f) * _e7998)));
                                                    phi_12130_ = _e8307;
                                                } else {
                                                    phi_12129_ = vec3<f32>();
                                                    phi_12130_ = vec3<f32>();
                                                }
                                                let _e8309 = phi_12129_;
                                                let _e8311 = phi_12130_;
                                                phi_31811_ = vec3<f32>(_e7698.member_5.x, _e7698.member_5.y, _e7698.member_5.z);
                                                phi_31735_ = vec3<f32>(_e7698.member_4.x, _e7698.member_4.y, _e7698.member_4.z);
                                                phi_31657_ = _e7989;
                                                phi_31576_ = _e7977;
                                                phi_12132_ = _e8309;
                                                phi_12133_ = _e8311;
                                                phi_12134_ = _e7990;
                                            } else {
                                                phi_31811_ = vec3<f32>(0f, 0f, 0f);
                                                phi_31735_ = vec3<f32>(0f, 0f, 0f);
                                                phi_31657_ = 0u;
                                                phi_31576_ = 1f;
                                                phi_12132_ = _e7963;
                                                phi_12133_ = _e7965;
                                                phi_12134_ = _e7969;
                                            }
                                            let _e8313 = phi_31811_;
                                            let _e8315 = phi_31735_;
                                            let _e8317 = phi_31657_;
                                            let _e8319 = phi_31576_;
                                            let _e8321 = phi_12132_;
                                            let _e8323 = phi_12133_;
                                            let _e8325 = phi_12134_;
                                            phi_32078_ = vec3<f32>(0f, 0f, 0f);
                                            phi_31887_ = 1.5f;
                                            phi_31809_ = _e8313;
                                            phi_31733_ = _e8315;
                                            phi_31655_ = _e8317;
                                            phi_31574_ = _e8319;
                                            phi_31498_ = select(0u, 1u, _e7967);
                                            phi_31425_ = _e7959;
                                            phi_12142_ = select(_e7961, _e7758, vec3(_e7967));
                                            phi_12143_ = _e8321;
                                            phi_12144_ = _e8323;
                                            phi_12145_ = select(true, false, _e8325);
                                        }
                                        let _e8435 = phi_32078_;
                                        let _e8437 = phi_31887_;
                                        let _e8439 = phi_31809_;
                                        let _e8441 = phi_31733_;
                                        let _e8443 = phi_31655_;
                                        let _e8445 = phi_31574_;
                                        let _e8447 = phi_31498_;
                                        let _e8449 = phi_31425_;
                                        let _e8451 = phi_12142_;
                                        let _e8453 = phi_12143_;
                                        let _e8455 = phi_12144_;
                                        let _e8457 = phi_12145_;
                                        let _e8458 = select(true, false, _e8457);
                                        if _e8458 {
                                            let _e8459 = (_e393 >= _e365);
                                            if _e8459 {
                                                phi_31482_ = _e8449;
                                                phi_12260_ = vec3<f32>();
                                                phi_12261_ = vec3<f32>();
                                                phi_12262_ = u32();
                                                phi_12263_ = false;
                                                phi_12264_ = false;
                                            } else {
                                                let _e8471 = global_1.member.member_4[1u];
                                                if (sqrt((((_e8455.x * _e8455.x) + (_e8455.y * _e8455.y)) + (_e8455.z * _e8455.z))) <= _e8471) {
                                                    phi_31484_ = _e8449;
                                                    phi_12225_ = _e7617;
                                                    phi_12226_ = false;
                                                } else {
                                                    let _e8475 = global_1.member.member_7[1u];
                                                    if (_e8475 == 1u) {
                                                        let _e8480 = global_1.member.member_4[3u];
                                                        if (f32(_e393) >= _e8480) {
                                                            let _e8483 = select(_e8455.y, _e8455.x, (_e8455.x > _e8455.y));
                                                            let _e8485 = select(_e8455.z, _e8483, (_e8483 > _e8455.z));
                                                            if (_e8485 != _e8485) {
                                                                phi_25415_ = true;
                                                            } else {
                                                                phi_25415_ = (1f <= _e8485);
                                                            }
                                                            let _e8489 = phi_25415_;
                                                            let _e8490 = select(_e8485, 1f, _e8489);
                                                            let _e8492 = ((_e8449 * 747796405u) + 2891336453u);
                                                            let _e8500 = (((_e8492 >> bitcast<u32>((((_e8492 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e8492) * 277803737u);
                                                            let _e8503 = ((_e8500 >> bitcast<u32>(22i)) ^ _e8500);
                                                            let _e8508 = ((f32((_e8503 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e8490);
                                                            if _e8508 {
                                                                phi_12214_ = vec3<f32>();
                                                            } else {
                                                                phi_12214_ = vec3<f32>((_e7617.x / _e8490), (_e7617.y / _e8490), (_e7617.z / _e8490));
                                                            }
                                                            let _e8517 = phi_12214_;
                                                            phi_31487_ = _e8503;
                                                            phi_12216_ = _e8517;
                                                            phi_12217_ = select(true, false, _e8508);
                                                        } else {
                                                            phi_31487_ = _e8449;
                                                            phi_12216_ = _e7617;
                                                            phi_12217_ = true;
                                                        }
                                                        let _e8520 = phi_31487_;
                                                        let _e8522 = phi_12216_;
                                                        let _e8524 = phi_12217_;
                                                        phi_31486_ = _e8520;
                                                        phi_12218_ = _e8522;
                                                        phi_12219_ = _e8524;
                                                    } else {
                                                        phi_31486_ = _e8449;
                                                        phi_12218_ = _e7617;
                                                        phi_12219_ = true;
                                                    }
                                                    let _e8526 = phi_31486_;
                                                    let _e8528 = phi_12218_;
                                                    let _e8530 = phi_12219_;
                                                    phi_31484_ = _e8526;
                                                    phi_12225_ = select(_e7617, _e8528, vec3(_e8530));
                                                    phi_12226_ = select(false, true, _e8530);
                                                }
                                                let _e8535 = phi_31484_;
                                                let _e8537 = phi_12225_;
                                                let _e8539 = phi_12226_;
                                                if _e8539 {
                                                    phi_12256_ = (_e8537 * _e8455);
                                                    phi_12257_ = (_e8453 * (1f / sqrt((((_e8453.x * _e8453.x) + (_e8453.y * _e8453.y)) + (_e8453.z * _e8453.z)))));
                                                    phi_12258_ = (_e393 + 1u);
                                                } else {
                                                    phi_12256_ = vec3<f32>();
                                                    phi_12257_ = vec3<f32>();
                                                    phi_12258_ = u32();
                                                }
                                                let _e8554 = phi_12256_;
                                                let _e8556 = phi_12257_;
                                                let _e8558 = phi_12258_;
                                                phi_31482_ = _e8535;
                                                phi_12260_ = _e8554;
                                                phi_12261_ = _e8556;
                                                phi_12262_ = _e8558;
                                                phi_12263_ = _e8539;
                                                phi_12264_ = select(true, false, _e8539);
                                            }
                                            let _e8561 = phi_31482_;
                                            let _e8563 = phi_12260_;
                                            let _e8565 = phi_12261_;
                                            let _e8567 = phi_12262_;
                                            let _e8569 = phi_12263_;
                                            let _e8571 = phi_12264_;
                                            phi_31481_ = _e8561;
                                            phi_12265_ = _e8563;
                                            phi_12266_ = _e8565;
                                            phi_12267_ = _e8567;
                                            phi_12269_ = _e8569;
                                            phi_12270_ = _e8571;
                                            phi_12271_ = _e8459;
                                        } else {
                                            phi_31481_ = _e8449;
                                            phi_12265_ = vec3<f32>();
                                            phi_12266_ = vec3<f32>();
                                            phi_12267_ = u32();
                                            phi_12269_ = false;
                                            phi_12270_ = false;
                                            phi_12271_ = false;
                                        }
                                        let _e8573 = phi_31481_;
                                        let _e8575 = phi_12265_;
                                        let _e8577 = phi_12266_;
                                        let _e8579 = phi_12267_;
                                        let _e8581 = phi_12269_;
                                        let _e8583 = phi_12270_;
                                        let _e8585 = phi_12271_;
                                        phi_32075_ = _e8435;
                                        phi_31884_ = _e8437;
                                        phi_31806_ = _e8439;
                                        phi_31730_ = _e8441;
                                        phi_31652_ = _e8443;
                                        phi_31571_ = _e8445;
                                        phi_31495_ = _e8447;
                                        phi_31480_ = _e8573;
                                        phi_12272_ = _e8575;
                                        phi_12273_ = _e8577;
                                        phi_12274_ = _e8579;
                                        phi_12275_ = _e8451;
                                        phi_12276_ = select(true, false, _e8458);
                                        phi_12277_ = _e8581;
                                        phi_12278_ = _e8583;
                                        phi_12279_ = _e8585;
                                    }
                                    let _e8593 = phi_32075_;
                                    let _e8595 = phi_31884_;
                                    let _e8597 = phi_31806_;
                                    let _e8599 = phi_31730_;
                                    let _e8601 = phi_31652_;
                                    let _e8603 = phi_31571_;
                                    let _e8605 = phi_31495_;
                                    let _e8607 = phi_31480_;
                                    let _e8609 = phi_12272_;
                                    let _e8611 = phi_12273_;
                                    let _e8613 = phi_12274_;
                                    let _e8615 = phi_12275_;
                                    let _e8617 = phi_12276_;
                                    let _e8619 = phi_12277_;
                                    let _e8621 = phi_12278_;
                                    let _e8623 = phi_12279_;
                                    if _e8617 {
                                        let _e8626 = global_1.member.member_1[3u];
                                        phi_33411_ = _e7613;
                                        phi_31479_ = _e8607;
                                        phi_12288_ = _e1157;
                                        phi_12291_ = _e7615;
                                        phi_12293_ = vec3<f32>(0f, 0f, 0f);
                                        phi_12295_ = 0u;
                                        phi_12297_ = _e389;
                                        phi_12299_ = _e391;
                                        loop {
                                            let _e8628 = phi_33411_;
                                            let _e8630 = phi_31479_;
                                            let _e8632 = phi_12288_;
                                            let _e8634 = phi_12291_;
                                            let _e8636 = phi_12293_;
                                            let _e8638 = phi_12295_;
                                            let _e8640 = phi_12297_;
                                            let _e8642 = phi_12299_;
                                            local_119 = _e8632;
                                            local_120 = _e8634;
                                            local_121 = _e8636;
                                            local_122 = _e8640;
                                            local_123 = _e8642;
                                            local_124 = _e8632;
                                            local_125 = _e8634;
                                            local_126 = _e8636;
                                            local_127 = _e8640;
                                            local_128 = _e8642;
                                            let _e8644 = (_e8638 < (_e8626 + 3u));
                                            if _e8644 {
                                                if (_e8638 < _e8626) {
                                                    let _e10632 = global_5.member[_e8638];
                                                    let _e10637 = (_e10632.member.x - _e7755);
                                                    let _e10638 = (_e10632.member.y - _e7756);
                                                    let _e10639 = (_e10632.member.z - _e7757);
                                                    let _e10644 = (((_e10637 * _e10637) + (_e10638 * _e10638)) + (_e10639 * _e10639));
                                                    let _e10645 = sqrt(_e10644);
                                                    let _e10646 = (_e10637 / _e10645);
                                                    let _e10647 = (_e10638 / _e10645);
                                                    let _e10648 = (_e10639 / _e10645);
                                                    if (_e10632.member.w < 0.5f) {
                                                        phi_12396_ = 1f;
                                                    } else {
                                                        let _e10672 = (((_e10632.member_2.x * -(_e10646)) + (_e10632.member_2.y * -(_e10647))) + (_e10632.member_2.z * -(_e10648)));
                                                        if (_e10632.member_3.x == _e10632.member_2.w) {
                                                            phi_12395_ = select(1f, 0f, (_e10672 < _e10632.member_3.x));
                                                        } else {
                                                            let _e10676 = ((_e10672 - _e10632.member_3.x) / (_e10632.member_2.w - _e10632.member_3.x));
                                                            let _e10678 = select(_e10676, 0f, (_e10676 < 0f));
                                                            let _e10680 = select(_e10678, 1f, (_e10678 > 1f));
                                                            phi_12395_ = ((_e10680 * _e10680) * (3f - (2f * _e10680)));
                                                        }
                                                        let _e10688 = phi_12395_;
                                                        phi_12396_ = _e10688;
                                                    }
                                                    let _e10690 = phi_12396_;
                                                    phi_33771_ = _e8630;
                                                    phi_15334_ = 0f;
                                                    phi_15335_ = vec3<f32>(_e10646, _e10647, _e10648);
                                                    phi_15336_ = vec3<f32>(((_e10632.member_1.x * _e10690) / _e10644), ((_e10632.member_1.y * _e10690) / _e10644), ((_e10632.member_1.z * _e10690) / _e10644));
                                                    phi_15337_ = _e10645;
                                                } else {
                                                    if (_e8638 == _e8626) {
                                                        if ((_e368 & 4u) == 0u) {
                                                            phi_33815_ = _e8630;
                                                            phi_14438_ = 0f;
                                                            phi_14439_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_14440_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            let _e9381 = global_1.member.member_5;
                                                            let _e9389 = global_1.member.member_6[3u];
                                                            let _e9391 = sin((_e9389 * 0.5f));
                                                            let _e9393 = ((12.566371f * _e9391) * _e9391);
                                                            let _e9398 = (((_e9381.x * _e7730.x) + (_e9381.y * _e7730.y)) + (_e9381.z * _e7730.z));
                                                            if (_e9398 != _e9398) {
                                                                phi_25899_ = true;
                                                            } else {
                                                                phi_25899_ = (0f >= _e9398);
                                                            }
                                                            let _e9402 = phi_25899_;
                                                            let _e9404 = (select(_e9398, 0f, _e9402) * 0.31830987f);
                                                            if (_e8605 == 0u) {
                                                                if (_e9398 <= 0f) {
                                                                    phi_26696_ = vec3<f32>(0f, 0f, 0f);
                                                                } else {
                                                                    phi_26696_ = (_e8599 * (_e9398 * 0.31830987f));
                                                                }
                                                                let _e9883 = phi_26696_;
                                                                phi_12806_ = type_59(_e9883, _e9404);
                                                                phi_12807_ = type_59();
                                                                phi_12808_ = true;
                                                            } else {
                                                                let _e9413 = (((_e7704.x * _e7776) + (_e7704.y * _e7777)) + (_e7704.z * _e7779));
                                                                let _e9418 = (((_e7704.x * _e7772) + (_e7704.y * _e7782)) + (_e7704.z * _e7783));
                                                                let _e9423 = (((_e7704.x * _e7730.x) + (_e7704.y * _e7730.y)) + (_e7704.z * _e7730.z));
                                                                let _e9428 = (((_e9381.x * _e7776) + (_e9381.y * _e7777)) + (_e9381.z * _e7779));
                                                                let _e9433 = (((_e9381.x * _e7772) + (_e9381.y * _e7782)) + (_e9381.z * _e7783));
                                                                if (_e9423 <= 0f) {
                                                                    phi_12770_ = type_59();
                                                                    phi_12771_ = true;
                                                                } else {
                                                                    let _e9435 = (_e9398 <= 0f);
                                                                    if _e9435 {
                                                                        phi_12769_ = type_59();
                                                                    } else {
                                                                        let _e9436 = (_e9413 + _e9428);
                                                                        let _e9437 = (_e9418 + _e9433);
                                                                        let _e9438 = (_e9423 + _e9398);
                                                                        let _e9445 = (1f / sqrt((((_e9436 * _e9436) + (_e9437 * _e9437)) + (_e9438 * _e9438))));
                                                                        let _e9448 = (_e9438 * _e9445);
                                                                        if (_e9448 <= 0f) {
                                                                            phi_26009_ = 0f;
                                                                        } else {
                                                                            let _e9450 = (_e8603 * _e8603);
                                                                            let _e9454 = ((((_e9450 - 1f) * _e9448) * _e9448) + 1f);
                                                                            phi_26009_ = (_e9450 / ((3.1415927f * _e9454) * _e9454));
                                                                        }
                                                                        let _e9459 = phi_26009_;
                                                                        let _e9460 = (_e9423 * _e9423);
                                                                        if (_e9460 >= 1f) {
                                                                            phi_26037_ = 0f;
                                                                        } else {
                                                                            phi_26037_ = ((sqrt((1f + (((_e8603 * _e8603) * ((_e9413 * _e9413) + (_e9418 * _e9418))) / _e9460))) - 1f) * 0.5f);
                                                                        }
                                                                        let _e9473 = phi_26037_;
                                                                        let _e9474 = (1f + _e9473);
                                                                        let _e9475 = (_e9398 * _e9398);
                                                                        if (_e9475 >= 1f) {
                                                                            phi_26065_ = 0f;
                                                                        } else {
                                                                            phi_26065_ = ((sqrt((1f + (((_e8603 * _e8603) * ((_e9428 * _e9428) + (_e9433 * _e9433))) / _e9475))) - 1f) * 0.5f);
                                                                        }
                                                                        let _e9488 = phi_26065_;
                                                                        let _e9495 = (((_e9413 * (_e9436 * _e9445)) + (_e9418 * (_e9437 * _e9445))) + (_e9423 * _e9448));
                                                                        if (_e8601 == 0u) {
                                                                            let _e9790 = (1f - _e9495);
                                                                            if (_e9790 != _e9790) {
                                                                                phi_26568_ = true;
                                                                            } else {
                                                                                phi_26568_ = (0f >= _e9790);
                                                                            }
                                                                            let _e9794 = phi_26568_;
                                                                            let _e9796 = pow(select(_e9790, 0f, _e9794), 5f);
                                                                            let _e9797 = (1f - _e9796);
                                                                            phi_12757_ = vec3<f32>(((_e8599.x * _e9797) + _e9796), ((_e8599.y * _e9797) + _e9796), ((_e8599.z * _e9797) + _e9796));
                                                                        } else {
                                                                            if (_e8601 == 1u) {
                                                                                let _e9763 = (1f / _e8595);
                                                                                let _e9767 = ((_e9763 * _e9763) * (1f - (_e9495 * _e9495)));
                                                                                if (_e9767 >= 1f) {
                                                                                    phi_26529_ = 1f;
                                                                                } else {
                                                                                    let _e9770 = sqrt((1f - _e9767));
                                                                                    if (_e9495 != _e9495) {
                                                                                        phi_26540_ = true;
                                                                                    } else {
                                                                                        phi_26540_ = (_e9770 <= _e9495);
                                                                                    }
                                                                                    let _e9774 = phi_26540_;
                                                                                    let _e9779 = pow(((1f - _e8595) / (1f + _e8595)), 2f);
                                                                                    phi_26529_ = (_e9779 + ((1f - _e9779) * pow((1f - select(_e9495, _e9770, _e9774)), 5f)));
                                                                                }
                                                                                let _e9786 = phi_26529_;
                                                                                phi_12756_ = vec3(_e9786);
                                                                            } else {
                                                                                let _e9499 = select(_e9495, 0f, (_e9495 < 0f));
                                                                                let _e9501 = select(_e9499, 1f, (_e9499 > 1f));
                                                                                let _e9503 = (1f - (_e9501 * _e9501));
                                                                                let _e9516 = ((_e8599.x * _e8599.x) - (_e8597.x * _e8597.x));
                                                                                let _e9517 = ((_e8599.y * _e8599.y) - (_e8597.y * _e8597.y));
                                                                                let _e9518 = ((_e8599.z * _e8599.z) - (_e8597.z * _e8597.z));
                                                                                let _e9520 = (_e8599.x * (_e8597.x + _e8597.x));
                                                                                let _e9522 = (_e8599.y * (_e8597.y + _e8597.y));
                                                                                let _e9524 = (_e8599.z * (_e8597.z + _e8597.z));
                                                                                let _e9531 = ((_e9516 * _e9516) + (_e9520 * _e9520));
                                                                                let _e9532 = ((_e9517 * _e9517) + (_e9522 * _e9522));
                                                                                let _e9533 = ((_e9518 * _e9518) + (_e9524 * _e9524));
                                                                                let _e9538 = ((_e9503 * _e9516) / _e9531);
                                                                                let _e9539 = ((_e9503 * _e9517) / _e9532);
                                                                                let _e9540 = ((_e9503 * _e9518) / _e9533);
                                                                                let _e9541 = (1f - _e9538);
                                                                                let _e9542 = (1f - _e9539);
                                                                                let _e9543 = (1f - _e9540);
                                                                                let _e9545 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e9538, _e9539, _e9540));
                                                                                let _e9546 = (_e9503 * _e9520);
                                                                                let _e9547 = (_e9503 * _e9522);
                                                                                let _e9548 = (_e9503 * _e9524);
                                                                                let _e9553 = (vec3<f32>(_e9546, _e9547, _e9548) / vec3<f32>(_e9531, _e9532, _e9533));
                                                                                let _e9566 = ((_e9545.x * _e9545.x) + (_e9553.x * _e9553.x));
                                                                                let _e9567 = ((_e9545.y * _e9545.y) + (_e9553.y * _e9553.y));
                                                                                let _e9568 = ((_e9545.z * _e9545.z) + (_e9553.z * _e9553.z));
                                                                                if (_e9566 != _e9566) {
                                                                                    phi_26199_ = true;
                                                                                } else {
                                                                                    phi_26199_ = (0f >= _e9566);
                                                                                }
                                                                                let _e9572 = phi_26199_;
                                                                                let _e9574 = sqrt(select(_e9566, 0f, _e9572));
                                                                                if (_e9567 != _e9567) {
                                                                                    phi_26214_ = true;
                                                                                } else {
                                                                                    phi_26214_ = (0f >= _e9567);
                                                                                }
                                                                                let _e9578 = phi_26214_;
                                                                                let _e9580 = sqrt(select(_e9567, 0f, _e9578));
                                                                                if (_e9568 != _e9568) {
                                                                                    phi_26229_ = true;
                                                                                } else {
                                                                                    phi_26229_ = (0f >= _e9568);
                                                                                }
                                                                                let _e9584 = phi_26229_;
                                                                                let _e9586 = sqrt(select(_e9568, 0f, _e9584));
                                                                                let _e9590 = ((_e9574 + _e9541) * 0.5f);
                                                                                let _e9591 = ((_e9580 + _e9542) * 0.5f);
                                                                                let _e9592 = ((_e9586 + _e9543) * 0.5f);
                                                                                if (_e9590 != _e9590) {
                                                                                    phi_26256_ = true;
                                                                                } else {
                                                                                    phi_26256_ = (0f >= _e9590);
                                                                                }
                                                                                let _e9596 = phi_26256_;
                                                                                let _e9598 = sqrt(select(_e9590, 0f, _e9596));
                                                                                if (_e9591 != _e9591) {
                                                                                    phi_26271_ = true;
                                                                                } else {
                                                                                    phi_26271_ = (0f >= _e9591);
                                                                                }
                                                                                let _e9602 = phi_26271_;
                                                                                let _e9604 = sqrt(select(_e9591, 0f, _e9602));
                                                                                if (_e9592 != _e9592) {
                                                                                    phi_26286_ = true;
                                                                                } else {
                                                                                    phi_26286_ = (0f >= _e9592);
                                                                                }
                                                                                let _e9608 = phi_26286_;
                                                                                let _e9610 = sqrt(select(_e9592, 0f, _e9608));
                                                                                let _e9611 = vec3<f32>(_e9598, _e9604, _e9610);
                                                                                let _e9615 = ((_e9574 - _e9541) * 0.5f);
                                                                                let _e9616 = ((_e9580 - _e9542) * 0.5f);
                                                                                let _e9617 = ((_e9586 - _e9543) * 0.5f);
                                                                                if (_e9615 != _e9615) {
                                                                                    phi_26313_ = true;
                                                                                } else {
                                                                                    phi_26313_ = (0f >= _e9615);
                                                                                }
                                                                                let _e9621 = phi_26313_;
                                                                                let _e9623 = sqrt(select(_e9615, 0f, _e9621));
                                                                                if (_e9616 != _e9616) {
                                                                                    phi_26328_ = true;
                                                                                } else {
                                                                                    phi_26328_ = (0f >= _e9616);
                                                                                }
                                                                                let _e9627 = phi_26328_;
                                                                                let _e9629 = sqrt(select(_e9616, 0f, _e9627));
                                                                                if (_e9617 != _e9617) {
                                                                                    phi_26343_ = true;
                                                                                } else {
                                                                                    phi_26343_ = (0f >= _e9617);
                                                                                }
                                                                                let _e9633 = phi_26343_;
                                                                                let _e9635 = sqrt(select(_e9617, 0f, _e9633));
                                                                                if ((_e9546 / _e9531) < 0f) {
                                                                                    phi_12633_ = -(_e9623);
                                                                                } else {
                                                                                    phi_12633_ = _e9623;
                                                                                }
                                                                                let _e9639 = phi_12633_;
                                                                                if ((_e9547 / _e9532) < 0f) {
                                                                                    phi_12641_ = -(_e9629);
                                                                                } else {
                                                                                    phi_12641_ = _e9629;
                                                                                }
                                                                                let _e9643 = phi_12641_;
                                                                                if ((_e9548 / _e9533) < 0f) {
                                                                                    phi_12649_ = -(_e9635);
                                                                                } else {
                                                                                    phi_12649_ = _e9635;
                                                                                }
                                                                                let _e9647 = phi_12649_;
                                                                                let _e9651 = (_e8597.x * _e9501);
                                                                                let _e9652 = (_e8597.y * _e9501);
                                                                                let _e9653 = (_e8597.z * _e9501);
                                                                                let _e9663 = vec3<f32>(((_e8599.x * _e9598) - (_e8597.x * _e9639)), ((_e8599.y * _e9604) - (_e8597.y * _e9643)), ((_e8599.z * _e9610) - (_e8597.z * _e9647)));
                                                                                let _e9670 = ((_e8599.x * _e9639) + (_e8597.x * _e9598));
                                                                                let _e9671 = ((_e8599.y * _e9643) + (_e8597.y * _e9604));
                                                                                let _e9672 = ((_e8599.z * _e9647) + (_e8597.z * _e9610));
                                                                                let _e9674 = vec3<f32>((_e8599.x * _e9501), (_e8599.y * _e9501), (_e8599.z * _e9501));
                                                                                let _e9675 = (_e9674 - _e9611);
                                                                                let _e9676 = (_e9651 - _e9639);
                                                                                let _e9677 = (_e9652 - _e9643);
                                                                                let _e9678 = (_e9653 - _e9647);
                                                                                let _e9691 = (_e9674 + _e9611);
                                                                                let _e9692 = (_e9651 + _e9639);
                                                                                let _e9693 = (_e9652 + _e9643);
                                                                                let _e9694 = (_e9653 + _e9647);
                                                                                let _e9704 = ((_e9691.x * _e9691.x) + (_e9692 * _e9692));
                                                                                let _e9705 = ((_e9691.y * _e9691.y) + (_e9693 * _e9693));
                                                                                let _e9706 = ((_e9691.z * _e9691.z) + (_e9694 * _e9694));
                                                                                let _e9716 = vec3(_e9501);
                                                                                let _e9717 = (_e9716 - _e9663);
                                                                                let _e9718 = -(vec3<f32>(_e9670, _e9671, _e9672));
                                                                                let _e9734 = (_e9716 + _e9663);
                                                                                let _e9744 = ((_e9734.x * _e9734.x) + (_e9670 * _e9670));
                                                                                let _e9745 = ((_e9734.y * _e9734.y) + (_e9671 * _e9671));
                                                                                let _e9746 = ((_e9734.z * _e9734.z) + (_e9672 * _e9672));
                                                                                phi_12756_ = vec3<f32>((((((_e9675.x * _e9675.x) + (_e9676 * _e9676)) / select(0.000000000000000000000000000001f, _e9704, (_e9704 > 0.000000000000000000000000000001f))) + (((_e9717.x * _e9717.x) + (_e9718.x * _e9718.x)) / select(0.000000000000000000000000000001f, _e9744, (_e9744 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e9675.y * _e9675.y) + (_e9677 * _e9677)) / select(0.000000000000000000000000000001f, _e9705, (_e9705 > 0.000000000000000000000000000001f))) + (((_e9717.y * _e9717.y) + (_e9718.y * _e9718.y)) / select(0.000000000000000000000000000001f, _e9745, (_e9745 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e9675.z * _e9675.z) + (_e9678 * _e9678)) / select(0.000000000000000000000000000001f, _e9706, (_e9706 > 0.000000000000000000000000000001f))) + (((_e9717.z * _e9717.z) + (_e9718.z * _e9718.z)) / select(0.000000000000000000000000000001f, _e9746, (_e9746 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                                            }
                                                                            let _e9789 = phi_12756_;
                                                                            phi_12757_ = _e9789;
                                                                        }
                                                                        let _e9809 = phi_12757_;
                                                                        let _e9811 = (4f * _e9423);
                                                                        phi_12769_ = type_59((_e9809 * (((1f / (_e9474 + _e9488)) * _e9459) / _e9811)), ((_e9459 / _e9474) / _e9811));
                                                                    }
                                                                    let _e9818 = phi_12769_;
                                                                    phi_12770_ = _e9818;
                                                                    phi_12771_ = _e9435;
                                                                }
                                                                let _e9820 = phi_12770_;
                                                                let _e9822 = phi_12771_;
                                                                if _e9822 {
                                                                    phi_12776_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                                                } else {
                                                                    phi_12776_ = _e9820;
                                                                }
                                                                let _e9824 = phi_12776_;
                                                                let _e9827 = (_e8605 == 1u);
                                                                if _e9827 {
                                                                    phi_12804_ = _e9824;
                                                                    phi_12805_ = type_59();
                                                                } else {
                                                                    if (_e9423 != _e9423) {
                                                                        phi_26605_ = true;
                                                                    } else {
                                                                        phi_26605_ = (0f >= _e9423);
                                                                    }
                                                                    let _e9831 = phi_26605_;
                                                                    let _e9832 = select(_e9423, 0f, _e9831);
                                                                    let _e9833 = (1f / _e8595);
                                                                    let _e9837 = ((_e9833 * _e9833) * (1f - (_e9832 * _e9832)));
                                                                    if (_e9837 >= 1f) {
                                                                        phi_26624_ = 1f;
                                                                    } else {
                                                                        let _e9840 = sqrt((1f - _e9837));
                                                                        if (_e9832 != _e9832) {
                                                                            phi_26635_ = true;
                                                                        } else {
                                                                            phi_26635_ = (_e9840 <= _e9832);
                                                                        }
                                                                        let _e9844 = phi_26635_;
                                                                        let _e9849 = pow(((1f - _e8595) / (1f + _e8595)), 2f);
                                                                        phi_26624_ = (_e9849 + ((1f - _e9849) * pow((1f - select(_e9832, _e9840, _e9844)), 5f)));
                                                                    }
                                                                    let _e9856 = phi_26624_;
                                                                    if (_e9398 <= 0f) {
                                                                        phi_26674_ = vec3<f32>(0f, 0f, 0f);
                                                                    } else {
                                                                        phi_26674_ = (_e8593 * (_e9398 * 0.31830987f));
                                                                    }
                                                                    let _e9861 = phi_26674_;
                                                                    let _e9862 = (1f - _e9856);
                                                                    phi_12804_ = type_59();
                                                                    phi_12805_ = type_59((_e9824.member + vec3<f32>((_e9861.x * _e9862), (_e9861.y * _e9862), (_e9861.z * _e9862))), ((_e9856 * _e9824.member_1) + (_e9862 * _e9404)));
                                                                }
                                                                let _e9876 = phi_12804_;
                                                                let _e9878 = phi_12805_;
                                                                phi_12806_ = _e9876;
                                                                phi_12807_ = _e9878;
                                                                phi_12808_ = _e9827;
                                                            }
                                                            let _e9886 = phi_12806_;
                                                            let _e9888 = phi_12807_;
                                                            let _e9890 = phi_12808_;
                                                            if _e9890 {
                                                                phi_12812_ = _e9886;
                                                            } else {
                                                                phi_12812_ = _e9888;
                                                            }
                                                            let _e9892 = phi_12812_;
                                                            if ((_e9892.member_1 * _e9393) < 0.001f) {
                                                                let _e10319 = global_1.member.member_6;
                                                                let _e10325 = global_1.member.member_8[2u];
                                                                if (_e10325 == 4294967295u) {
                                                                    phi_13551_ = vec3<f32>(1f, 1f, 1f);
                                                                } else {
                                                                    let _e10328 = global_6.member[_e10325];
                                                                    let _e10331 = global_6.member[(_e10325 + 1u)];
                                                                    let _e10334 = global_6.member[(_e10325 + 2u)];
                                                                    let _e10337 = global_6.member[(_e10325 + 3u)];
                                                                    let _e10340 = global_6.member[(_e10325 + 4u)];
                                                                    let _e10343 = global_6.member[(_e10325 + 5u)];
                                                                    let _e10346 = global_6.member[(_e10325 + 6u)];
                                                                    let _e10349 = global_6.member[(_e10325 + 7u)];
                                                                    let _e10352 = global_6.member[(_e10325 + 8u)];
                                                                    let _e10355 = global_6.member[(_e10325 + 9u)];
                                                                    let _e10358 = global_6.member[(_e10325 + 13u)];
                                                                    let _e10361 = global_6.member[(_e10325 + 14u)];
                                                                    let _e10364 = global_6.member[(_e10325 + 15u)];
                                                                    let _e10367 = global_6.member[(_e10325 + 16u)];
                                                                    let _e10370 = global_6.member[(_e10325 + 18u)];
                                                                    let _e10373 = global_6.member[(_e10325 + 19u)];
                                                                    let _e10376 = global_6.member[(_e10325 + 20u)];
                                                                    let _e10379 = global_6.member[(_e10325 + 29u)];
                                                                    let _e10380 = bitcast<u32>(_e10379);
                                                                    let _e10393 = (((((_e7755 * _e7755) + (_e7756 * _e7756)) + (_e7757 * _e7757)) - (2f * (((_e7755 * _e10328) + (_e7756 * _e10331)) + (_e7757 * _e10334)))) + _e10346);
                                                                    if (_e10393 > 0f) {
                                                                        let _e10402 = ((((_e7755 - _e10328) * _e10349) + ((_e7756 - _e10331) * _e10352)) + ((_e7757 - _e10334) * _e10355));
                                                                        let _e10404 = ((_e10402 * _e10402) - _e10393);
                                                                        if (_e10402 >= 0f) {
                                                                            phi_13251_ = vec3<f32>();
                                                                            phi_13252_ = true;
                                                                        } else {
                                                                            let _e10406 = (_e10404 <= 0f);
                                                                            if _e10406 {
                                                                                phi_13250_ = vec3<f32>();
                                                                            } else {
                                                                                let _e10409 = (-(_e10402) - sqrt(_e10404));
                                                                                phi_13250_ = vec3<f32>((_e7755 + (_e10349 * _e10409)), (_e7756 + (_e10352 * _e10409)), (_e7757 + (_e10355 * _e10409)));
                                                                            }
                                                                            let _e10418 = phi_13250_;
                                                                            phi_13251_ = _e10418;
                                                                            phi_13252_ = _e10406;
                                                                        }
                                                                        let _e10420 = phi_13251_;
                                                                        let _e10422 = phi_13252_;
                                                                        phi_13257_ = _e10420;
                                                                        phi_13258_ = select(true, false, _e10422);
                                                                    } else {
                                                                        phi_13257_ = _e7758;
                                                                        phi_13258_ = true;
                                                                    }
                                                                    let _e10425 = phi_13257_;
                                                                    let _e10427 = phi_13258_;
                                                                    if _e10427 {
                                                                        let _e10429 = (_e10425.x - _e10328);
                                                                        let _e10431 = (_e10425.y - _e10331);
                                                                        let _e10433 = (_e10425.z - _e10334);
                                                                        let _e10439 = sqrt((((_e10429 * _e10429) + (_e10431 * _e10431)) + (_e10433 * _e10433)));
                                                                        let _e10440 = (1f / _e10439);
                                                                        let _e10458 = ((((((_e10425.x * _e10425.x) + (_e10425.y * _e10425.y)) + (_e10425.z * _e10425.z)) - (2f * (((_e10425.x * _e10328) + (_e10425.y * _e10331)) + (_e10425.z * _e10334)))) + _e10343) / (_e10439 + _e10337));
                                                                        if (_e10458 != _e10458) {
                                                                            phi_27130_ = true;
                                                                        } else {
                                                                            phi_27130_ = (0f >= _e10458);
                                                                        }
                                                                        let _e10462 = phi_27130_;
                                                                        let _e10466 = sqrt((select(_e10458, 0f, _e10462) / (_e10340 - _e10337)));
                                                                        if (_e10466 != _e10466) {
                                                                            phi_27145_ = true;
                                                                        } else {
                                                                            phi_27145_ = (1f <= _e10466);
                                                                        }
                                                                        let _e10470 = phi_27145_;
                                                                        let _e10472 = (select(_e10466, 1f, _e10470) * 63f);
                                                                        let _e10477 = ((((_e10429 * _e10440) * _e10349) + ((_e10431 * _e10440) * _e10352)) + ((_e10433 * _e10440) * _e10355));
                                                                        let _e10479 = select(_e10477, -1f, (_e10477 < -1f));
                                                                        let _e10483 = ((select(_e10479, 1f, (_e10479 > 1f)) + 1f) * 127.5f);
                                                                        let _e10488 = select(select(u32(_e10472), 0u, (_e10472 < 0f)), 4294967295u, (_e10472 > 4294967000f));
                                                                        let _e10490 = select(_e10488, 62u, (62u < _e10488));
                                                                        let _e10495 = select(select(u32(_e10483), 0u, (_e10483 < 0f)), 4294967295u, (_e10483 > 4294967000f));
                                                                        let _e10497 = select(_e10495, 254u, (254u < _e10495));
                                                                        let _e10499 = (_e10472 - f32(_e10490));
                                                                        let _e10501 = (_e10483 - f32(_e10497));
                                                                        let _e10505 = (_e10380 + (3u * ((_e10490 * 256u) + _e10497)));
                                                                        let _e10510 = (_e10380 + (3u * (((_e10490 + 1u) * 256u) + _e10497)));
                                                                        let _e10512 = global_6.member[_e10505];
                                                                        let _e10515 = global_6.member[(_e10505 + 1u)];
                                                                        let _e10518 = global_6.member[(_e10505 + 2u)];
                                                                        let _e10519 = (1f - _e10501);
                                                                        let _e10525 = global_6.member[(_e10505 + 3u)];
                                                                        let _e10528 = global_6.member[(_e10505 + 4u)];
                                                                        let _e10531 = global_6.member[(_e10505 + 5u)];
                                                                        let _e10538 = (1f - _e10499);
                                                                        let _e10543 = global_6.member[_e10510];
                                                                        let _e10546 = global_6.member[(_e10510 + 1u)];
                                                                        let _e10549 = global_6.member[(_e10510 + 2u)];
                                                                        let _e10555 = global_6.member[(_e10510 + 3u)];
                                                                        let _e10558 = global_6.member[(_e10510 + 4u)];
                                                                        let _e10561 = global_6.member[(_e10510 + 5u)];
                                                                        phi_13549_ = vec3<f32>(((((_e10512 * _e10519) + (_e10525 * _e10501)) * _e10538) + (((_e10543 * _e10519) + (_e10555 * _e10501)) * _e10499)), ((((_e10515 * _e10519) + (_e10528 * _e10501)) * _e10538) + (((_e10546 * _e10519) + (_e10558 * _e10501)) * _e10499)), ((((_e10518 * _e10519) + (_e10531 * _e10501)) * _e10538) + (((_e10549 * _e10519) + (_e10561 * _e10501)) * _e10499)));
                                                                    } else {
                                                                        phi_13549_ = vec3<f32>(0f, 0f, 0f);
                                                                    }
                                                                    let _e10576 = phi_13549_;
                                                                    let _e10583 = ((_e10367 * 1.1f) * _e10576.y);
                                                                    phi_13551_ = vec3<f32>(exp(-((((_e10358 * _e10576.x) + _e10583) + (_e10370 * _e10576.z)))), exp(-((((_e10361 * _e10576.x) + _e10583) + (_e10373 * _e10576.z)))), exp(-((((_e10364 * _e10576.x) + _e10583) + (_e10376 * _e10576.z)))));
                                                                }
                                                                let _e10602 = phi_13551_;
                                                                phi_33816_ = _e8630;
                                                                phi_14435_ = 0f;
                                                                phi_14436_ = vec3<f32>(_e9381.x, _e9381.y, _e9381.z);
                                                                phi_14437_ = (vec3<f32>(_e10319.x, _e10319.y, _e10319.z) * _e10602);
                                                            } else {
                                                                let _e9897 = ((_e8630 * 747796405u) + 2891336453u);
                                                                let _e9905 = (((_e9897 >> bitcast<u32>((((_e9897 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e9897) * 277803737u);
                                                                let _e9908 = ((_e9905 >> bitcast<u32>(22i)) ^ _e9905);
                                                                let _e9914 = (((f32((_e9908 >> bitcast<u32>(8i))) * 0.000000059604645f) * _e9393) * 0.15915494f);
                                                                let _e9915 = (1f - _e9914);
                                                                let _e9917 = (_e9914 * (2f - _e9914));
                                                                if (_e9917 != _e9917) {
                                                                    phi_26728_ = true;
                                                                } else {
                                                                    phi_26728_ = (0f >= _e9917);
                                                                }
                                                                let _e9921 = phi_26728_;
                                                                let _e9923 = sqrt(select(_e9917, 0f, _e9921));
                                                                let _e9925 = ((_e9908 * 747796405u) + 2891336453u);
                                                                let _e9933 = (((_e9925 >> bitcast<u32>((((_e9925 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e9925) * 277803737u);
                                                                let _e9936 = ((_e9933 >> bitcast<u32>(22i)) ^ _e9933);
                                                                let _e9940 = (f32((_e9936 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                                                let _e9942 = select(-1f, 1f, (_e9381.z >= 0f));
                                                                let _e9944 = (-1f / (_e9942 + _e9381.z));
                                                                let _e9946 = ((_e9381.x * _e9381.y) * _e9944);
                                                                let _e9959 = (_e9923 * cos(_e9940));
                                                                let _e9964 = (_e9923 * sin(_e9940));
                                                                let _e9974 = ((((1f + (((_e9942 * _e9381.x) * _e9381.x) * _e9944)) * _e9959) + (_e9946 * _e9964)) + (_e9381.x * _e9915));
                                                                let _e9975 = ((((_e9942 * _e9946) * _e9959) + ((_e9942 + ((_e9381.y * _e9381.y) * _e9944)) * _e9964)) + (_e9381.y * _e9915));
                                                                let _e9976 = ((((-(_e9942) * _e9381.x) * _e9959) + (-(_e9381.y) * _e9964)) + (_e9381.z * _e9915));
                                                                let _e9978 = (_e9974 - _e9381.x);
                                                                let _e9979 = (_e9975 - _e9381.y);
                                                                let _e9980 = (_e9976 - _e9381.z);
                                                                let _e9987 = (sqrt((((_e9978 * _e9978) + (_e9979 * _e9979)) + (_e9980 * _e9980))) * 0.5f);
                                                                let _e9988 = (_e9987 != _e9987);
                                                                if _e9988 {
                                                                    phi_26799_ = true;
                                                                } else {
                                                                    phi_26799_ = (1f <= _e9987);
                                                                }
                                                                let _e9991 = phi_26799_;
                                                                if ((2f * asin(select(_e9987, 1f, _e9991))) >= _e9389) {
                                                                    phi_13632_ = 0f;
                                                                } else {
                                                                    phi_13632_ = (1f / _e9393);
                                                                }
                                                                let _e9998 = phi_13632_;
                                                                if (_e9998 > 0f) {
                                                                    if _e9988 {
                                                                        phi_26814_ = true;
                                                                    } else {
                                                                        phi_26814_ = (1f <= _e9987);
                                                                    }
                                                                    let _e10002 = phi_26814_;
                                                                    let _e10005 = (2f * asin(select(_e9987, 1f, _e10002)));
                                                                    if (_e10005 >= _e9389) {
                                                                        phi_14433_ = vec3<f32>(0f, 0f, 0f);
                                                                    } else {
                                                                        let _e10009 = (1f - pow((_e10005 / _e9389), 2f));
                                                                        if (_e10009 != _e10009) {
                                                                            phi_26829_ = true;
                                                                        } else {
                                                                            phi_26829_ = (0f >= _e10009);
                                                                        }
                                                                        let _e10013 = phi_26829_;
                                                                        let _e10016 = global_1.member.member_6;
                                                                        let _e10022 = global_1.member.member_8[2u];
                                                                        if (_e10022 == 4294967295u) {
                                                                            phi_14406_ = vec3<f32>(1f, 1f, 1f);
                                                                        } else {
                                                                            let _e10025 = global_6.member[_e10022];
                                                                            let _e10028 = global_6.member[(_e10022 + 1u)];
                                                                            let _e10031 = global_6.member[(_e10022 + 2u)];
                                                                            let _e10034 = global_6.member[(_e10022 + 3u)];
                                                                            let _e10037 = global_6.member[(_e10022 + 4u)];
                                                                            let _e10040 = global_6.member[(_e10022 + 5u)];
                                                                            let _e10043 = global_6.member[(_e10022 + 6u)];
                                                                            let _e10046 = global_6.member[(_e10022 + 7u)];
                                                                            let _e10049 = global_6.member[(_e10022 + 8u)];
                                                                            let _e10052 = global_6.member[(_e10022 + 9u)];
                                                                            let _e10055 = global_6.member[(_e10022 + 13u)];
                                                                            let _e10058 = global_6.member[(_e10022 + 14u)];
                                                                            let _e10061 = global_6.member[(_e10022 + 15u)];
                                                                            let _e10064 = global_6.member[(_e10022 + 16u)];
                                                                            let _e10067 = global_6.member[(_e10022 + 18u)];
                                                                            let _e10070 = global_6.member[(_e10022 + 19u)];
                                                                            let _e10073 = global_6.member[(_e10022 + 20u)];
                                                                            let _e10076 = global_6.member[(_e10022 + 29u)];
                                                                            let _e10077 = bitcast<u32>(_e10076);
                                                                            let _e10090 = (((((_e7755 * _e7755) + (_e7756 * _e7756)) + (_e7757 * _e7757)) - (2f * (((_e7755 * _e10025) + (_e7756 * _e10028)) + (_e7757 * _e10031)))) + _e10043);
                                                                            if (_e10090 > 0f) {
                                                                                let _e10099 = ((((_e7755 - _e10025) * _e10046) + ((_e7756 - _e10028) * _e10049)) + ((_e7757 - _e10031) * _e10052));
                                                                                let _e10101 = ((_e10099 * _e10099) - _e10090);
                                                                                if (_e10099 >= 0f) {
                                                                                    phi_14106_ = vec3<f32>();
                                                                                    phi_14107_ = true;
                                                                                } else {
                                                                                    let _e10103 = (_e10101 <= 0f);
                                                                                    if _e10103 {
                                                                                        phi_14105_ = vec3<f32>();
                                                                                    } else {
                                                                                        let _e10106 = (-(_e10099) - sqrt(_e10101));
                                                                                        phi_14105_ = vec3<f32>((_e7755 + (_e10046 * _e10106)), (_e7756 + (_e10049 * _e10106)), (_e7757 + (_e10052 * _e10106)));
                                                                                    }
                                                                                    let _e10115 = phi_14105_;
                                                                                    phi_14106_ = _e10115;
                                                                                    phi_14107_ = _e10103;
                                                                                }
                                                                                let _e10117 = phi_14106_;
                                                                                let _e10119 = phi_14107_;
                                                                                phi_14112_ = _e10117;
                                                                                phi_14113_ = select(true, false, _e10119);
                                                                            } else {
                                                                                phi_14112_ = _e7758;
                                                                                phi_14113_ = true;
                                                                            }
                                                                            let _e10122 = phi_14112_;
                                                                            let _e10124 = phi_14113_;
                                                                            if _e10124 {
                                                                                let _e10126 = (_e10122.x - _e10025);
                                                                                let _e10128 = (_e10122.y - _e10028);
                                                                                let _e10130 = (_e10122.z - _e10031);
                                                                                let _e10136 = sqrt((((_e10126 * _e10126) + (_e10128 * _e10128)) + (_e10130 * _e10130)));
                                                                                let _e10137 = (1f / _e10136);
                                                                                let _e10155 = ((((((_e10122.x * _e10122.x) + (_e10122.y * _e10122.y)) + (_e10122.z * _e10122.z)) - (2f * (((_e10122.x * _e10025) + (_e10122.y * _e10028)) + (_e10122.z * _e10031)))) + _e10040) / (_e10136 + _e10034));
                                                                                if (_e10155 != _e10155) {
                                                                                    phi_26953_ = true;
                                                                                } else {
                                                                                    phi_26953_ = (0f >= _e10155);
                                                                                }
                                                                                let _e10159 = phi_26953_;
                                                                                let _e10163 = sqrt((select(_e10155, 0f, _e10159) / (_e10037 - _e10034)));
                                                                                if (_e10163 != _e10163) {
                                                                                    phi_26968_ = true;
                                                                                } else {
                                                                                    phi_26968_ = (1f <= _e10163);
                                                                                }
                                                                                let _e10167 = phi_26968_;
                                                                                let _e10169 = (select(_e10163, 1f, _e10167) * 63f);
                                                                                let _e10174 = ((((_e10126 * _e10137) * _e10046) + ((_e10128 * _e10137) * _e10049)) + ((_e10130 * _e10137) * _e10052));
                                                                                let _e10176 = select(_e10174, -1f, (_e10174 < -1f));
                                                                                let _e10180 = ((select(_e10176, 1f, (_e10176 > 1f)) + 1f) * 127.5f);
                                                                                let _e10185 = select(select(u32(_e10169), 0u, (_e10169 < 0f)), 4294967295u, (_e10169 > 4294967000f));
                                                                                let _e10187 = select(_e10185, 62u, (62u < _e10185));
                                                                                let _e10192 = select(select(u32(_e10180), 0u, (_e10180 < 0f)), 4294967295u, (_e10180 > 4294967000f));
                                                                                let _e10194 = select(_e10192, 254u, (254u < _e10192));
                                                                                let _e10196 = (_e10169 - f32(_e10187));
                                                                                let _e10198 = (_e10180 - f32(_e10194));
                                                                                let _e10202 = (_e10077 + (3u * ((_e10187 * 256u) + _e10194)));
                                                                                let _e10207 = (_e10077 + (3u * (((_e10187 + 1u) * 256u) + _e10194)));
                                                                                let _e10209 = global_6.member[_e10202];
                                                                                let _e10212 = global_6.member[(_e10202 + 1u)];
                                                                                let _e10215 = global_6.member[(_e10202 + 2u)];
                                                                                let _e10216 = (1f - _e10198);
                                                                                let _e10222 = global_6.member[(_e10202 + 3u)];
                                                                                let _e10225 = global_6.member[(_e10202 + 4u)];
                                                                                let _e10228 = global_6.member[(_e10202 + 5u)];
                                                                                let _e10235 = (1f - _e10196);
                                                                                let _e10240 = global_6.member[_e10207];
                                                                                let _e10243 = global_6.member[(_e10207 + 1u)];
                                                                                let _e10246 = global_6.member[(_e10207 + 2u)];
                                                                                let _e10252 = global_6.member[(_e10207 + 3u)];
                                                                                let _e10255 = global_6.member[(_e10207 + 4u)];
                                                                                let _e10258 = global_6.member[(_e10207 + 5u)];
                                                                                phi_14404_ = vec3<f32>(((((_e10209 * _e10216) + (_e10222 * _e10198)) * _e10235) + (((_e10240 * _e10216) + (_e10252 * _e10198)) * _e10196)), ((((_e10212 * _e10216) + (_e10225 * _e10198)) * _e10235) + (((_e10243 * _e10216) + (_e10255 * _e10198)) * _e10196)), ((((_e10215 * _e10216) + (_e10228 * _e10198)) * _e10235) + (((_e10246 * _e10216) + (_e10258 * _e10198)) * _e10196)));
                                                                            } else {
                                                                                phi_14404_ = vec3<f32>(0f, 0f, 0f);
                                                                            }
                                                                            let _e10273 = phi_14404_;
                                                                            let _e10280 = ((_e10064 * 1.1f) * _e10273.y);
                                                                            phi_14406_ = vec3<f32>(exp(-((((_e10055 * _e10273.x) + _e10280) + (_e10067 * _e10273.z)))), exp(-((((_e10058 * _e10273.x) + _e10280) + (_e10070 * _e10273.z)))), exp(-((((_e10061 * _e10273.x) + _e10280) + (_e10073 * _e10273.z)))));
                                                                        }
                                                                        let _e10299 = phi_14406_;
                                                                        let _e10310 = (((1f - (0.6f * (1f - sqrt(select(_e10009, 0f, _e10013))))) * 1.25f) / _e9393);
                                                                        phi_14433_ = vec3<f32>(((_e10016.x * _e10299.x) * _e10310), ((_e10016.y * _e10299.y) * _e10310), ((_e10016.z * _e10299.z) * _e10310));
                                                                    }
                                                                    let _e10316 = phi_14433_;
                                                                    phi_14434_ = _e10316;
                                                                } else {
                                                                    phi_14434_ = vec3<f32>(0f, 0f, 0f);
                                                                }
                                                                let _e10318 = phi_14434_;
                                                                phi_33816_ = _e9936;
                                                                phi_14435_ = _e9998;
                                                                phi_14436_ = vec3<f32>(_e9974, _e9975, _e9976);
                                                                phi_14437_ = _e10318;
                                                            }
                                                            let _e10606 = phi_33816_;
                                                            let _e10608 = phi_14435_;
                                                            let _e10610 = phi_14436_;
                                                            let _e10612 = phi_14437_;
                                                            phi_33815_ = _e10606;
                                                            phi_14438_ = _e10608;
                                                            phi_14439_ = _e10610;
                                                            phi_14440_ = _e10612;
                                                        }
                                                        let _e10614 = phi_33815_;
                                                        let _e10616 = phi_14438_;
                                                        let _e10618 = phi_14439_;
                                                        let _e10620 = phi_14440_;
                                                        phi_33772_ = _e10614;
                                                        phi_15330_ = _e10616;
                                                        phi_15331_ = _e10618;
                                                        phi_15332_ = _e10620;
                                                        phi_15333_ = 1000000000000000000000000000000f;
                                                    } else {
                                                        if (_e8638 == (_e8626 + 1u)) {
                                                            if ((_e368 & 1u) == 0u) {
                                                                phi_33781_ = _e8630;
                                                                phi_14964_ = 0f;
                                                                phi_14965_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_14966_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_14967_ = 1000000000000000000000000000000f;
                                                            } else {
                                                                if (_e371 > 0u) {
                                                                    let _e8871 = global_1.member.member_2[1u];
                                                                    let _e8873 = ((_e8630 * 747796405u) + 2891336453u);
                                                                    let _e8881 = (((_e8873 >> bitcast<u32>((((_e8873 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e8873) * 277803737u);
                                                                    let _e8884 = ((_e8881 >> bitcast<u32>(22i)) ^ _e8881);
                                                                    phi_14461_ = 0u;
                                                                    phi_14464_ = _e371;
                                                                    loop {
                                                                        let _e8890 = phi_14461_;
                                                                        let _e8892 = phi_14464_;
                                                                        local_71 = _e8890;
                                                                        local_72 = _e8890;
                                                                        let _e8894 = ((_e8890 + 1u) < _e8892);
                                                                        if _e8894 {
                                                                            let _e8896 = ((_e8890 + _e8892) / 2u);
                                                                            let _e8899 = global_6.member[(_e8871 + _e8896)];
                                                                            let _e8900 = (_e8899 <= (f32((_e8884 >> bitcast<u32>(8i))) * 0.000000059604645f));
                                                                            phi_14462_ = select(_e8890, _e8896, _e8900);
                                                                            phi_14465_ = select(_e8896, _e8892, _e8900);
                                                                        } else {
                                                                            phi_14462_ = u32();
                                                                            phi_14465_ = u32();
                                                                        }
                                                                        let _e8904 = phi_14462_;
                                                                        let _e8906 = phi_14465_;
                                                                        phi_14461_ = _e8904;
                                                                        phi_14464_ = _e8906;
                                                                        if !(_e8894) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e8909 = local_71;
                                                                    let _e8910 = (_e8871 + _e8909);
                                                                    let _e8913 = global_6.member[(_e8910 + 1u)];
                                                                    let _e8915 = global_6.member[_e8910];
                                                                    let _e8919 = global_1.member.member_2[0u];
                                                                    let _e8921 = local_72;
                                                                    let _e8924 = global_6.member[(_e8919 + _e8921)];
                                                                    let _e8927 = ((_e8884 * 747796405u) + 2891336453u);
                                                                    let _e8935 = (((_e8927 >> bitcast<u32>((((_e8927 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e8927) * 277803737u);
                                                                    let _e8938 = ((_e8935 >> bitcast<u32>(22i)) ^ _e8935);
                                                                    let _e8941 = f32((_e8938 >> bitcast<u32>(8i)));
                                                                    let _e8942 = (_e8941 * 0.000000059604645f);
                                                                    let _e8944 = ((_e8938 * 747796405u) + 2891336453u);
                                                                    let _e8952 = (((_e8944 >> bitcast<u32>((((_e8944 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e8944) * 277803737u);
                                                                    let _e8955 = ((_e8952 >> bitcast<u32>(22i)) ^ _e8952);
                                                                    let _e8958 = f32((_e8955 >> bitcast<u32>(8i)));
                                                                    let _e8961 = global_2.member[bitcast<u32>(_e8924)];
                                                                    let _e8964 = (_e8961.member_6.x == 1u);
                                                                    if _e8964 {
                                                                        let _e9106 = (_e8961.member.x - _e7755);
                                                                        let _e9107 = (_e8961.member.y - _e7756);
                                                                        let _e9108 = (_e8961.member.z - _e7757);
                                                                        let _e9113 = (((_e9106 * _e9106) + (_e9107 * _e9107)) + (_e9108 * _e9108));
                                                                        let _e9114 = (_e8961.member.w * _e8961.member.w);
                                                                        let _e9116 = (_e9113 <= (_e9114 * 1.000001f));
                                                                        if _e9116 {
                                                                            let _e9225 = (_e8941 * 0.00000037450704f);
                                                                            let _e9227 = ((_e8958 * 0.00000011920929f) - 1f);
                                                                            let _e9229 = (1f - (_e9227 * _e9227));
                                                                            if (_e9229 != _e9229) {
                                                                                phi_25796_ = true;
                                                                            } else {
                                                                                phi_25796_ = (0f >= _e9229);
                                                                            }
                                                                            let _e9233 = phi_25796_;
                                                                            let _e9235 = sqrt(select(_e9229, 0f, _e9233));
                                                                            let _e9237 = (_e9235 * cos(_e9225));
                                                                            let _e9239 = (_e9235 * sin(_e9225));
                                                                            let _e9244 = (_e8961.member.x + (_e9237 * _e8961.member.w));
                                                                            let _e9245 = (_e8961.member.y + (_e9239 * _e8961.member.w));
                                                                            let _e9246 = (_e8961.member.z + (_e9227 * _e8961.member.w));
                                                                            if _e9116 {
                                                                                let _e9259 = (_e9244 - _e7755);
                                                                                let _e9260 = (_e9245 - _e7756);
                                                                                let _e9261 = (_e9246 - _e7757);
                                                                                let _e9266 = (((_e9259 * _e9259) + (_e9260 * _e9260)) + (_e9261 * _e9261));
                                                                                phi_25865_ = (_e9266 / (((abs((((_e9237 * _e9259) + (_e9239 * _e9260)) + (_e9227 * _e9261))) / sqrt(_e9266)) * 12.566371f) * _e9114));
                                                                            } else {
                                                                                let _e9248 = (_e9114 / _e9113);
                                                                                if (_e9248 != _e9248) {
                                                                                    phi_25884_ = true;
                                                                                } else {
                                                                                    phi_25884_ = (1f <= _e9248);
                                                                                }
                                                                                let _e9252 = phi_25884_;
                                                                                let _e9253 = select(_e9248, 1f, _e9252);
                                                                                phi_25865_ = (0.15915494f / (_e9253 / (1f + sqrt((1f - _e9253)))));
                                                                            }
                                                                            let _e9279 = phi_25865_;
                                                                            phi_14677_ = type_64(vec3<f32>(_e9244, _e9245, _e9246), vec3<f32>(_e9237, _e9239, _e9227), _e9279);
                                                                        } else {
                                                                            let _e9117 = (_e9114 / _e9113);
                                                                            let _e9118 = (_e9117 != _e9117);
                                                                            if _e9118 {
                                                                                phi_25648_ = true;
                                                                            } else {
                                                                                phi_25648_ = (1f <= _e9117);
                                                                            }
                                                                            let _e9121 = phi_25648_;
                                                                            let _e9122 = select(_e9117, 1f, _e9121);
                                                                            if _e9118 {
                                                                                phi_25671_ = true;
                                                                            } else {
                                                                                phi_25671_ = (1f <= _e9117);
                                                                            }
                                                                            let _e9126 = phi_25671_;
                                                                            let _e9127 = select(_e9117, 1f, _e9126);
                                                                            let _e9131 = (_e9127 / (1f + sqrt((1f - _e9127))));
                                                                            if (_e9122 < 0.00068523f) {
                                                                                phi_14612_ = (_e9122 * _e8942);
                                                                            } else {
                                                                                let _e9134 = (1f - (_e8942 * _e9131));
                                                                                let _e9136 = (1f - (_e9134 * _e9134));
                                                                                if (_e9136 != _e9136) {
                                                                                    phi_25686_ = true;
                                                                                } else {
                                                                                    phi_25686_ = (0f >= _e9136);
                                                                                }
                                                                                let _e9140 = phi_25686_;
                                                                                phi_14612_ = select(_e9136, 0f, _e9140);
                                                                            }
                                                                            let _e9144 = phi_14612_;
                                                                            let _e9145 = (1f - _e9144);
                                                                            if (_e9145 != _e9145) {
                                                                                phi_25701_ = true;
                                                                            } else {
                                                                                phi_25701_ = (0f >= _e9145);
                                                                            }
                                                                            let _e9149 = phi_25701_;
                                                                            let _e9154 = (1f - (_e9144 / _e9122));
                                                                            if (_e9154 != _e9154) {
                                                                                phi_25716_ = true;
                                                                            } else {
                                                                                phi_25716_ = (0f >= _e9154);
                                                                            }
                                                                            let _e9158 = phi_25716_;
                                                                            let _e9162 = ((_e9144 / sqrt(_e9122)) + (sqrt(select(_e9145, 0f, _e9149)) * sqrt(select(_e9154, 0f, _e9158))));
                                                                            let _e9164 = (1f - (_e9162 * _e9162));
                                                                            if (_e9164 != _e9164) {
                                                                                phi_25731_ = true;
                                                                            } else {
                                                                                phi_25731_ = (0f >= _e9164);
                                                                            }
                                                                            let _e9168 = phi_25731_;
                                                                            let _e9170 = sqrt(select(_e9164, 0f, _e9168));
                                                                            let _e9171 = (_e8958 * 0.00000037450704f);
                                                                            let _e9172 = sqrt(_e9113);
                                                                            let _e9173 = (_e9106 / _e9172);
                                                                            let _e9174 = (_e9107 / _e9172);
                                                                            let _e9175 = (_e9108 / _e9172);
                                                                            let _e9177 = select(-1f, 1f, (_e9175 >= 0f));
                                                                            let _e9179 = (-1f / (_e9177 + _e9175));
                                                                            let _e9181 = ((_e9173 * _e9174) * _e9179);
                                                                            let _e9194 = (_e9170 * cos(_e9171));
                                                                            let _e9199 = (_e9170 * sin(_e9171));
                                                                            let _e9212 = -(((((1f + (((_e9177 * _e9173) * _e9173) * _e9179)) * _e9194) + (_e9181 * _e9199)) + (_e9173 * _e9162)));
                                                                            let _e9213 = -(((((_e9177 * _e9181) * _e9194) + ((_e9177 + ((_e9174 * _e9174) * _e9179)) * _e9199)) + (_e9174 * _e9162)));
                                                                            let _e9214 = -(((((-(_e9177) * _e9173) * _e9194) + (-(_e9174) * _e9199)) + (_e9175 * _e9162)));
                                                                            phi_14677_ = type_64(vec3<f32>((_e8961.member.x + (_e9212 * _e8961.member.w)), (_e8961.member.y + (_e9213 * _e8961.member.w)), (_e8961.member.z + (_e9214 * _e8961.member.w))), vec3<f32>(_e9212, _e9213, _e9214), (0.15915494f / _e9131));
                                                                        }
                                                                        let _e9282 = phi_14677_;
                                                                        phi_14863_ = _e9282;
                                                                    } else {
                                                                        let _e8965 = sqrt(_e8942);
                                                                        let _e8966 = (1f - _e8965);
                                                                        let _e8967 = ((_e8958 * 0.000000059604645f) * _e8965);
                                                                        let _e8989 = (_e8965 - _e8967);
                                                                        let _e8993 = (((_e8961.member.x * _e8966) + (_e8961.member_1.x * _e8967)) + (_e8961.member_2.x * _e8989));
                                                                        let _e8994 = (((_e8961.member.y * _e8966) + (_e8961.member_1.y * _e8967)) + (_e8961.member_2.y * _e8989));
                                                                        let _e8995 = (((_e8961.member.z * _e8966) + (_e8961.member_1.z * _e8967)) + (_e8961.member_2.z * _e8989));
                                                                        let _e8997 = (_e8961.member_1.x - _e8961.member.x);
                                                                        let _e8998 = (_e8961.member_1.y - _e8961.member.y);
                                                                        let _e8999 = (_e8961.member_1.z - _e8961.member.z);
                                                                        let _e9000 = (_e8961.member_2.x - _e8961.member.x);
                                                                        let _e9001 = (_e8961.member_2.y - _e8961.member.y);
                                                                        let _e9002 = (_e8961.member_2.z - _e8961.member.z);
                                                                        let _e9005 = ((_e8998 * _e9002) - (_e9001 * _e8999));
                                                                        let _e9008 = ((_e8999 * _e9000) - (_e9002 * _e8997));
                                                                        let _e9011 = ((_e8997 * _e9001) - (_e9000 * _e8998));
                                                                        let _e9017 = sqrt((((_e9005 * _e9005) + (_e9008 * _e9008)) + (_e9011 * _e9011)));
                                                                        let _e9018 = (1f / _e9017);
                                                                        let _e9019 = (_e9005 * _e9018);
                                                                        let _e9020 = (_e9008 * _e9018);
                                                                        let _e9021 = (_e9011 * _e9018);
                                                                        if _e8964 {
                                                                            let _e9054 = (_e8961.member.x - _e7755);
                                                                            let _e9055 = (_e8961.member.y - _e7756);
                                                                            let _e9056 = (_e8961.member.z - _e7757);
                                                                            let _e9061 = (((_e9054 * _e9054) + (_e9055 * _e9055)) + (_e9056 * _e9056));
                                                                            let _e9062 = (_e8961.member.w * _e8961.member.w);
                                                                            if (_e9061 <= (_e9062 * 1.000001f)) {
                                                                                let _e9076 = (_e8993 - _e7755);
                                                                                let _e9077 = (_e8994 - _e7756);
                                                                                let _e9078 = (_e8995 - _e7757);
                                                                                let _e9083 = (((_e9076 * _e9076) + (_e9077 * _e9077)) + (_e9078 * _e9078));
                                                                                phi_25614_ = (_e9083 / (((abs((((_e9019 * _e9076) + (_e9020 * _e9077)) + (_e9021 * _e9078))) / sqrt(_e9083)) * 12.566371f) * _e9062));
                                                                            } else {
                                                                                let _e9065 = (_e9062 / _e9061);
                                                                                if (_e9065 != _e9065) {
                                                                                    phi_25633_ = true;
                                                                                } else {
                                                                                    phi_25633_ = (1f <= _e9065);
                                                                                }
                                                                                let _e9069 = phi_25633_;
                                                                                let _e9070 = select(_e9065, 1f, _e9069);
                                                                                phi_25614_ = (0.15915494f / (_e9070 / (1f + sqrt((1f - _e9070)))));
                                                                            }
                                                                            let _e9096 = phi_25614_;
                                                                            phi_14861_ = _e9096;
                                                                        } else {
                                                                            let _e9023 = (_e8993 - _e7755);
                                                                            let _e9024 = (_e8994 - _e7756);
                                                                            let _e9025 = (_e8995 - _e7757);
                                                                            let _e9030 = (((_e9023 * _e9023) + (_e9024 * _e9024)) + (_e9025 * _e9025));
                                                                            let _e9038 = (abs((((_e9019 * _e9023) + (_e9020 * _e9024)) + (_e9021 * _e9025))) / sqrt(_e9030));
                                                                            if (_e9038 <= 0f) {
                                                                                phi_14860_ = 0f;
                                                                            } else {
                                                                                if (_e8961.member_6.x == 0u) {
                                                                                    phi_14857_ = (_e9017 * 0.5f);
                                                                                } else {
                                                                                    phi_14857_ = ((12.566371f * _e8961.member.w) * _e8961.member.w);
                                                                                }
                                                                                let _e9047 = phi_14857_;
                                                                                phi_14860_ = (_e9030 / (_e9038 * _e9047));
                                                                            }
                                                                            let _e9051 = phi_14860_;
                                                                            phi_14861_ = _e9051;
                                                                        }
                                                                        let _e9098 = phi_14861_;
                                                                        phi_14863_ = type_64(vec3<f32>(_e8993, _e8994, _e8995), vec3<f32>(_e9019, _e9020, _e9021), _e9098);
                                                                    }
                                                                    let _e9284 = phi_14863_;
                                                                    let _e9289 = (_e9284.member.x - _e7755);
                                                                    let _e9291 = (_e9284.member.y - _e7756);
                                                                    let _e9293 = (_e9284.member.z - _e7757);
                                                                    let _e9299 = sqrt((((_e9289 * _e9289) + (_e9291 * _e9291)) + (_e9293 * _e9293)));
                                                                    let _e9300 = (_e9289 / _e9299);
                                                                    let _e9301 = (_e9291 / _e9299);
                                                                    let _e9302 = (_e9293 / _e9299);
                                                                    if (_e9284.member_2 > 0f) {
                                                                        let _e9314 = global_4.member[_e8961.member_6.y];
                                                                        if (_e9314.member == 4u) {
                                                                            if (_e9314.member_1 == 1u) {
                                                                                phi_14941_ = true;
                                                                            } else {
                                                                                phi_14941_ = ((((-(_e9300) * _e9284.member_1.x) + (-(_e9301) * _e9284.member_1.y)) + (-(_e9302) * _e9284.member_1.z)) > 0f);
                                                                            }
                                                                            let _e9329 = phi_14941_;
                                                                            if _e9329 {
                                                                                phi_14950_ = vec3<f32>(_e9314.member_4.x, _e9314.member_4.y, _e9314.member_4.z);
                                                                            } else {
                                                                                phi_14950_ = vec3<f32>();
                                                                            }
                                                                            let _e9336 = phi_14950_;
                                                                            phi_14952_ = _e9336;
                                                                            phi_14953_ = select(true, false, _e9329);
                                                                        } else {
                                                                            phi_14952_ = vec3<f32>();
                                                                            phi_14953_ = true;
                                                                        }
                                                                        let _e9339 = phi_14952_;
                                                                        let _e9341 = phi_14953_;
                                                                        phi_14958_ = ((_e8913 - _e8915) * _e9284.member_2);
                                                                        phi_14959_ = select(_e9339, vec3<f32>(0f, 0f, 0f), vec3(_e9341));
                                                                    } else {
                                                                        phi_14958_ = 0f;
                                                                        phi_14959_ = vec3<f32>(0f, 0f, 0f);
                                                                    }
                                                                    let _e9345 = phi_14958_;
                                                                    let _e9347 = phi_14959_;
                                                                    phi_33782_ = _e8955;
                                                                    phi_14960_ = _e9345;
                                                                    phi_14961_ = vec3<f32>(_e9300, _e9301, _e9302);
                                                                    phi_14962_ = _e9347;
                                                                    phi_14963_ = ((_e9299 * 0.9999f) - _e7748);
                                                                } else {
                                                                    phi_33782_ = _e8630;
                                                                    phi_14960_ = 0f;
                                                                    phi_14961_ = vec3<f32>(0f, 0f, 0f);
                                                                    phi_14962_ = vec3<f32>(0f, 0f, 0f);
                                                                    phi_14963_ = 1000000000000000000000000000000f;
                                                                }
                                                                let _e9349 = phi_33782_;
                                                                let _e9351 = phi_14960_;
                                                                let _e9353 = phi_14961_;
                                                                let _e9355 = phi_14962_;
                                                                let _e9357 = phi_14963_;
                                                                phi_33781_ = _e9349;
                                                                phi_14964_ = _e9351;
                                                                phi_14965_ = _e9353;
                                                                phi_14966_ = _e9355;
                                                                phi_14967_ = _e9357;
                                                            }
                                                            let _e9359 = phi_33781_;
                                                            let _e9361 = phi_14964_;
                                                            let _e9363 = phi_14965_;
                                                            let _e9365 = phi_14966_;
                                                            let _e9367 = phi_14967_;
                                                            phi_33773_ = _e9359;
                                                            phi_15326_ = _e9361;
                                                            phi_15327_ = _e9363;
                                                            phi_15328_ = _e9365;
                                                            phi_15329_ = _e9367;
                                                        } else {
                                                            if ((_e368 & 2u) == 0u) {
                                                                phi_33774_ = _e8630;
                                                                phi_15323_ = 0f;
                                                                phi_15324_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_15325_ = vec3<f32>(0f, 0f, 0f);
                                                            } else {
                                                                let _e8652 = ((_e8630 * 747796405u) + 2891336453u);
                                                                let _e8660 = (((_e8652 >> bitcast<u32>((((_e8652 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e8652) * 277803737u);
                                                                let _e8663 = ((_e8660 >> bitcast<u32>(22i)) ^ _e8660);
                                                                let _e8667 = (f32((_e8663 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                                let _e8669 = ((_e8663 * 747796405u) + 2891336453u);
                                                                let _e8677 = (((_e8669 >> bitcast<u32>((((_e8669 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e8669) * 277803737u);
                                                                let _e8680 = ((_e8677 >> bitcast<u32>(22i)) ^ _e8677);
                                                                let _e8684 = (f32((_e8680 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                                let _e8687 = global_1.member.member_3[1u];
                                                                let _e8690 = global_1.member.member_3[2u];
                                                                let _e8693 = global_1.member.member_2[3u];
                                                                phi_14988_ = 0u;
                                                                phi_14991_ = _e8690;
                                                                loop {
                                                                    let _e8695 = phi_14988_;
                                                                    let _e8697 = phi_14991_;
                                                                    local_66 = _e8695;
                                                                    local_69 = _e8695;
                                                                    local_70 = _e8695;
                                                                    let _e8699 = ((_e8695 + 1u) < _e8697);
                                                                    if _e8699 {
                                                                        let _e8701 = ((_e8695 + _e8697) / 2u);
                                                                        let _e8704 = global_6.member[(_e8693 + _e8701)];
                                                                        let _e8705 = (_e8704 <= _e8667);
                                                                        phi_14989_ = select(_e8695, _e8701, _e8705);
                                                                        phi_14992_ = select(_e8701, _e8697, _e8705);
                                                                    } else {
                                                                        phi_14989_ = u32();
                                                                        phi_14992_ = u32();
                                                                    }
                                                                    let _e8709 = phi_14989_;
                                                                    let _e8711 = phi_14992_;
                                                                    phi_14988_ = _e8709;
                                                                    phi_14991_ = _e8711;
                                                                    if !(_e8699) {
                                                                        break;
                                                                    }
                                                                }
                                                                let _e8715 = global_1.member.member_3[0u];
                                                                let _e8716 = (_e8687 + 1u);
                                                                let _e8718 = local_66;
                                                                let _e8720 = (_e8715 + (_e8718 * _e8716));
                                                                phi_15029_ = 0u;
                                                                phi_15032_ = _e8687;
                                                                loop {
                                                                    let _e8722 = phi_15029_;
                                                                    let _e8724 = phi_15032_;
                                                                    local_67 = _e8722;
                                                                    local_68 = _e8722;
                                                                    let _e8726 = ((_e8722 + 1u) < _e8724);
                                                                    if _e8726 {
                                                                        let _e8728 = ((_e8722 + _e8724) / 2u);
                                                                        let _e8731 = global_6.member[(_e8720 + _e8728)];
                                                                        let _e8732 = (_e8731 <= _e8684);
                                                                        phi_15030_ = select(_e8722, _e8728, _e8732);
                                                                        phi_15033_ = select(_e8728, _e8724, _e8732);
                                                                    } else {
                                                                        phi_15030_ = u32();
                                                                        phi_15033_ = u32();
                                                                    }
                                                                    let _e8736 = phi_15030_;
                                                                    let _e8738 = phi_15033_;
                                                                    phi_15029_ = _e8736;
                                                                    phi_15032_ = _e8738;
                                                                    if !(_e8726) {
                                                                        break;
                                                                    }
                                                                }
                                                                let _e8741 = local_67;
                                                                let _e8744 = local_68;
                                                                let _e8745 = (_e8720 + _e8744);
                                                                let _e8747 = global_6.member[_e8745];
                                                                let _e8750 = global_6.member[(_e8745 + 1u)];
                                                                if (_e8750 > _e8747) {
                                                                    let _e8754 = ((_e8684 - _e8747) / (_e8750 - _e8747));
                                                                    let _e8756 = select(_e8754, 0f, (_e8754 < 0f));
                                                                    phi_15109_ = select(_e8756, 0.99999f, (_e8756 > 0.99999f));
                                                                } else {
                                                                    phi_15109_ = 0.5f;
                                                                }
                                                                let _e8760 = phi_15109_;
                                                                let _e8762 = f32(_e8687);
                                                                let _e8765 = local_69;
                                                                let _e8768 = local_70;
                                                                let _e8769 = (_e8693 + _e8768);
                                                                let _e8771 = global_6.member[_e8769];
                                                                let _e8774 = global_6.member[(_e8769 + 1u)];
                                                                if (_e8774 > _e8771) {
                                                                    let _e8778 = ((_e8667 - _e8771) / (_e8774 - _e8771));
                                                                    let _e8780 = select(_e8778, 0f, (_e8778 < 0f));
                                                                    phi_15158_ = select(_e8780, 0.99999f, (_e8780 > 0.99999f));
                                                                } else {
                                                                    phi_15158_ = 0.5f;
                                                                }
                                                                let _e8784 = phi_15158_;
                                                                let _e8786 = f32(_e8690);
                                                                let _e8789 = ((((f32(_e8741) + _e8760) / _e8762) - 0.5f) * 6.2831855f);
                                                                let _e8790 = (((f32(_e8765) + _e8784) / _e8786) * 3.1415927f);
                                                                let _e8791 = sin(_e8790);
                                                                let _e8793 = (_e8791 * cos(_e8789));
                                                                let _e8794 = cos(_e8790);
                                                                let _e8796 = (_e8791 * sin(_e8789));
                                                                let _e8799 = ((atan2(_e8796, _e8793) * 0.15915494f) + 0.5f);
                                                                let _e8803 = select(_e8794, -1f, (_e8794 < -1f));
                                                                let _e8806 = acos(select(_e8803, 1f, (_e8803 > 1f)));
                                                                let _e8808 = ((_e8799 - floor(_e8799)) * _e8762);
                                                                let _e8813 = select(select(u32(_e8808), 0u, (_e8808 < 0f)), 4294967295u, (_e8808 > 4294967000f));
                                                                let _e8814 = (_e8687 - 1u);
                                                                let _e8817 = ((_e8806 * 0.31830987f) * _e8786);
                                                                let _e8822 = select(select(u32(_e8817), 0u, (_e8817 < 0f)), 4294967295u, (_e8817 > 4294967000f));
                                                                let _e8823 = (_e8690 - 1u);
                                                                let _e8825 = select(_e8822, _e8823, (_e8823 < _e8822));
                                                                let _e8826 = sin(_e8806);
                                                                if (_e8826 <= 0f) {
                                                                    phi_15314_ = 0f;
                                                                } else {
                                                                    let _e8830 = (_e8693 + _e8825);
                                                                    let _e8833 = global_6.member[(_e8830 + 1u)];
                                                                    let _e8835 = global_6.member[_e8830];
                                                                    let _e8838 = ((_e8715 + (_e8825 * _e8716)) + select(_e8813, _e8814, (_e8814 < _e8813)));
                                                                    let _e8841 = global_6.member[(_e8838 + 1u)];
                                                                    let _e8843 = global_6.member[_e8838];
                                                                    phi_15314_ = ((((_e8833 - _e8835) * _e8786) * ((_e8841 - _e8843) * _e8762)) / (19.73921f * _e8826));
                                                                }
                                                                let _e8850 = phi_15314_;
                                                                let _e8851 = (_e8850 > 0f);
                                                                if _e8851 {
                                                                    phi_15321_ = vec3<f32>(_e8793, _e8794, _e8796);
                                                                } else {
                                                                    phi_15321_ = vec3<f32>(0f, 0f, 0f);
                                                                }
                                                                let _e8854 = phi_15321_;
                                                                phi_33774_ = _e8680;
                                                                phi_15323_ = select(0f, _e8850, _e8851);
                                                                phi_15324_ = _e8854;
                                                                phi_15325_ = select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e8851));
                                                            }
                                                            let _e8859 = phi_33774_;
                                                            let _e8861 = phi_15323_;
                                                            let _e8863 = phi_15324_;
                                                            let _e8865 = phi_15325_;
                                                            phi_33773_ = _e8859;
                                                            phi_15326_ = _e8861;
                                                            phi_15327_ = _e8863;
                                                            phi_15328_ = _e8865;
                                                            phi_15329_ = 1000000000000000000000000000000f;
                                                        }
                                                        let _e9369 = phi_33773_;
                                                        let _e9371 = phi_15326_;
                                                        let _e9373 = phi_15327_;
                                                        let _e9375 = phi_15328_;
                                                        let _e9377 = phi_15329_;
                                                        phi_33772_ = _e9369;
                                                        phi_15330_ = _e9371;
                                                        phi_15331_ = _e9373;
                                                        phi_15332_ = _e9375;
                                                        phi_15333_ = _e9377;
                                                    }
                                                    let _e10622 = phi_33772_;
                                                    let _e10624 = phi_15330_;
                                                    let _e10626 = phi_15331_;
                                                    let _e10628 = phi_15332_;
                                                    let _e10630 = phi_15333_;
                                                    phi_33771_ = _e10622;
                                                    phi_15334_ = _e10624;
                                                    phi_15335_ = _e10626;
                                                    phi_15336_ = _e10628;
                                                    phi_15337_ = _e10630;
                                                }
                                                let _e10699 = phi_33771_;
                                                let _e10701 = phi_15334_;
                                                let _e10703 = phi_15335_;
                                                let _e10705 = phi_15336_;
                                                let _e10707 = phi_15337_;
                                                if ((((_e10705.x * _e10705.x) + (_e10705.y * _e10705.y)) + (_e10705.z * _e10705.z)) > 0f) {
                                                    let _e10724 = (((_e10703.x * _e7730.x) + (_e10703.y * _e7730.y)) + (_e10703.z * _e7730.z));
                                                    if (_e10724 != _e10724) {
                                                        phi_27198_ = true;
                                                    } else {
                                                        phi_27198_ = (0f >= _e10724);
                                                    }
                                                    let _e10728 = phi_27198_;
                                                    let _e10730 = (select(_e10724, 0f, _e10728) * 0.31830987f);
                                                    if (_e8605 == 0u) {
                                                        if (_e10724 <= 0f) {
                                                            phi_27995_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            phi_27995_ = (_e8599 * (_e10724 * 0.31830987f));
                                                        }
                                                        let _e11209 = phi_27995_;
                                                        phi_15733_ = type_59(_e11209, _e10730);
                                                        phi_15734_ = type_59();
                                                        phi_15735_ = true;
                                                    } else {
                                                        let _e10739 = (((_e7704.x * _e7776) + (_e7704.y * _e7777)) + (_e7704.z * _e7779));
                                                        let _e10744 = (((_e7704.x * _e7772) + (_e7704.y * _e7782)) + (_e7704.z * _e7783));
                                                        let _e10749 = (((_e7704.x * _e7730.x) + (_e7704.y * _e7730.y)) + (_e7704.z * _e7730.z));
                                                        let _e10754 = (((_e10703.x * _e7776) + (_e10703.y * _e7777)) + (_e10703.z * _e7779));
                                                        let _e10759 = (((_e10703.x * _e7772) + (_e10703.y * _e7782)) + (_e10703.z * _e7783));
                                                        if (_e10749 <= 0f) {
                                                            phi_15697_ = type_59();
                                                            phi_15698_ = true;
                                                        } else {
                                                            let _e10761 = (_e10724 <= 0f);
                                                            if _e10761 {
                                                                phi_15696_ = type_59();
                                                            } else {
                                                                let _e10762 = (_e10739 + _e10754);
                                                                let _e10763 = (_e10744 + _e10759);
                                                                let _e10764 = (_e10749 + _e10724);
                                                                let _e10771 = (1f / sqrt((((_e10762 * _e10762) + (_e10763 * _e10763)) + (_e10764 * _e10764))));
                                                                let _e10774 = (_e10764 * _e10771);
                                                                if (_e10774 <= 0f) {
                                                                    phi_27308_ = 0f;
                                                                } else {
                                                                    let _e10776 = (_e8603 * _e8603);
                                                                    let _e10780 = ((((_e10776 - 1f) * _e10774) * _e10774) + 1f);
                                                                    phi_27308_ = (_e10776 / ((3.1415927f * _e10780) * _e10780));
                                                                }
                                                                let _e10785 = phi_27308_;
                                                                let _e10786 = (_e10749 * _e10749);
                                                                if (_e10786 >= 1f) {
                                                                    phi_27336_ = 0f;
                                                                } else {
                                                                    phi_27336_ = ((sqrt((1f + (((_e8603 * _e8603) * ((_e10739 * _e10739) + (_e10744 * _e10744))) / _e10786))) - 1f) * 0.5f);
                                                                }
                                                                let _e10799 = phi_27336_;
                                                                let _e10800 = (1f + _e10799);
                                                                let _e10801 = (_e10724 * _e10724);
                                                                if (_e10801 >= 1f) {
                                                                    phi_27364_ = 0f;
                                                                } else {
                                                                    phi_27364_ = ((sqrt((1f + (((_e8603 * _e8603) * ((_e10754 * _e10754) + (_e10759 * _e10759))) / _e10801))) - 1f) * 0.5f);
                                                                }
                                                                let _e10814 = phi_27364_;
                                                                let _e10821 = (((_e10739 * (_e10762 * _e10771)) + (_e10744 * (_e10763 * _e10771))) + (_e10749 * _e10774));
                                                                if (_e8601 == 0u) {
                                                                    let _e11116 = (1f - _e10821);
                                                                    if (_e11116 != _e11116) {
                                                                        phi_27867_ = true;
                                                                    } else {
                                                                        phi_27867_ = (0f >= _e11116);
                                                                    }
                                                                    let _e11120 = phi_27867_;
                                                                    let _e11122 = pow(select(_e11116, 0f, _e11120), 5f);
                                                                    let _e11123 = (1f - _e11122);
                                                                    phi_15684_ = vec3<f32>(((_e8599.x * _e11123) + _e11122), ((_e8599.y * _e11123) + _e11122), ((_e8599.z * _e11123) + _e11122));
                                                                } else {
                                                                    if (_e8601 == 1u) {
                                                                        let _e11089 = (1f / _e8595);
                                                                        let _e11093 = ((_e11089 * _e11089) * (1f - (_e10821 * _e10821)));
                                                                        if (_e11093 >= 1f) {
                                                                            phi_27828_ = 1f;
                                                                        } else {
                                                                            let _e11096 = sqrt((1f - _e11093));
                                                                            if (_e10821 != _e10821) {
                                                                                phi_27839_ = true;
                                                                            } else {
                                                                                phi_27839_ = (_e11096 <= _e10821);
                                                                            }
                                                                            let _e11100 = phi_27839_;
                                                                            let _e11105 = pow(((1f - _e8595) / (1f + _e8595)), 2f);
                                                                            phi_27828_ = (_e11105 + ((1f - _e11105) * pow((1f - select(_e10821, _e11096, _e11100)), 5f)));
                                                                        }
                                                                        let _e11112 = phi_27828_;
                                                                        phi_15683_ = vec3(_e11112);
                                                                    } else {
                                                                        let _e10825 = select(_e10821, 0f, (_e10821 < 0f));
                                                                        let _e10827 = select(_e10825, 1f, (_e10825 > 1f));
                                                                        let _e10829 = (1f - (_e10827 * _e10827));
                                                                        let _e10842 = ((_e8599.x * _e8599.x) - (_e8597.x * _e8597.x));
                                                                        let _e10843 = ((_e8599.y * _e8599.y) - (_e8597.y * _e8597.y));
                                                                        let _e10844 = ((_e8599.z * _e8599.z) - (_e8597.z * _e8597.z));
                                                                        let _e10846 = (_e8599.x * (_e8597.x + _e8597.x));
                                                                        let _e10848 = (_e8599.y * (_e8597.y + _e8597.y));
                                                                        let _e10850 = (_e8599.z * (_e8597.z + _e8597.z));
                                                                        let _e10857 = ((_e10842 * _e10842) + (_e10846 * _e10846));
                                                                        let _e10858 = ((_e10843 * _e10843) + (_e10848 * _e10848));
                                                                        let _e10859 = ((_e10844 * _e10844) + (_e10850 * _e10850));
                                                                        let _e10864 = ((_e10829 * _e10842) / _e10857);
                                                                        let _e10865 = ((_e10829 * _e10843) / _e10858);
                                                                        let _e10866 = ((_e10829 * _e10844) / _e10859);
                                                                        let _e10867 = (1f - _e10864);
                                                                        let _e10868 = (1f - _e10865);
                                                                        let _e10869 = (1f - _e10866);
                                                                        let _e10871 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e10864, _e10865, _e10866));
                                                                        let _e10872 = (_e10829 * _e10846);
                                                                        let _e10873 = (_e10829 * _e10848);
                                                                        let _e10874 = (_e10829 * _e10850);
                                                                        let _e10879 = (vec3<f32>(_e10872, _e10873, _e10874) / vec3<f32>(_e10857, _e10858, _e10859));
                                                                        let _e10892 = ((_e10871.x * _e10871.x) + (_e10879.x * _e10879.x));
                                                                        let _e10893 = ((_e10871.y * _e10871.y) + (_e10879.y * _e10879.y));
                                                                        let _e10894 = ((_e10871.z * _e10871.z) + (_e10879.z * _e10879.z));
                                                                        if (_e10892 != _e10892) {
                                                                            phi_27498_ = true;
                                                                        } else {
                                                                            phi_27498_ = (0f >= _e10892);
                                                                        }
                                                                        let _e10898 = phi_27498_;
                                                                        let _e10900 = sqrt(select(_e10892, 0f, _e10898));
                                                                        if (_e10893 != _e10893) {
                                                                            phi_27513_ = true;
                                                                        } else {
                                                                            phi_27513_ = (0f >= _e10893);
                                                                        }
                                                                        let _e10904 = phi_27513_;
                                                                        let _e10906 = sqrt(select(_e10893, 0f, _e10904));
                                                                        if (_e10894 != _e10894) {
                                                                            phi_27528_ = true;
                                                                        } else {
                                                                            phi_27528_ = (0f >= _e10894);
                                                                        }
                                                                        let _e10910 = phi_27528_;
                                                                        let _e10912 = sqrt(select(_e10894, 0f, _e10910));
                                                                        let _e10916 = ((_e10900 + _e10867) * 0.5f);
                                                                        let _e10917 = ((_e10906 + _e10868) * 0.5f);
                                                                        let _e10918 = ((_e10912 + _e10869) * 0.5f);
                                                                        if (_e10916 != _e10916) {
                                                                            phi_27555_ = true;
                                                                        } else {
                                                                            phi_27555_ = (0f >= _e10916);
                                                                        }
                                                                        let _e10922 = phi_27555_;
                                                                        let _e10924 = sqrt(select(_e10916, 0f, _e10922));
                                                                        if (_e10917 != _e10917) {
                                                                            phi_27570_ = true;
                                                                        } else {
                                                                            phi_27570_ = (0f >= _e10917);
                                                                        }
                                                                        let _e10928 = phi_27570_;
                                                                        let _e10930 = sqrt(select(_e10917, 0f, _e10928));
                                                                        if (_e10918 != _e10918) {
                                                                            phi_27585_ = true;
                                                                        } else {
                                                                            phi_27585_ = (0f >= _e10918);
                                                                        }
                                                                        let _e10934 = phi_27585_;
                                                                        let _e10936 = sqrt(select(_e10918, 0f, _e10934));
                                                                        let _e10937 = vec3<f32>(_e10924, _e10930, _e10936);
                                                                        let _e10941 = ((_e10900 - _e10867) * 0.5f);
                                                                        let _e10942 = ((_e10906 - _e10868) * 0.5f);
                                                                        let _e10943 = ((_e10912 - _e10869) * 0.5f);
                                                                        if (_e10941 != _e10941) {
                                                                            phi_27612_ = true;
                                                                        } else {
                                                                            phi_27612_ = (0f >= _e10941);
                                                                        }
                                                                        let _e10947 = phi_27612_;
                                                                        let _e10949 = sqrt(select(_e10941, 0f, _e10947));
                                                                        if (_e10942 != _e10942) {
                                                                            phi_27627_ = true;
                                                                        } else {
                                                                            phi_27627_ = (0f >= _e10942);
                                                                        }
                                                                        let _e10953 = phi_27627_;
                                                                        let _e10955 = sqrt(select(_e10942, 0f, _e10953));
                                                                        if (_e10943 != _e10943) {
                                                                            phi_27642_ = true;
                                                                        } else {
                                                                            phi_27642_ = (0f >= _e10943);
                                                                        }
                                                                        let _e10959 = phi_27642_;
                                                                        let _e10961 = sqrt(select(_e10943, 0f, _e10959));
                                                                        if ((_e10872 / _e10857) < 0f) {
                                                                            phi_15560_ = -(_e10949);
                                                                        } else {
                                                                            phi_15560_ = _e10949;
                                                                        }
                                                                        let _e10965 = phi_15560_;
                                                                        if ((_e10873 / _e10858) < 0f) {
                                                                            phi_15568_ = -(_e10955);
                                                                        } else {
                                                                            phi_15568_ = _e10955;
                                                                        }
                                                                        let _e10969 = phi_15568_;
                                                                        if ((_e10874 / _e10859) < 0f) {
                                                                            phi_15576_ = -(_e10961);
                                                                        } else {
                                                                            phi_15576_ = _e10961;
                                                                        }
                                                                        let _e10973 = phi_15576_;
                                                                        let _e10977 = (_e8597.x * _e10827);
                                                                        let _e10978 = (_e8597.y * _e10827);
                                                                        let _e10979 = (_e8597.z * _e10827);
                                                                        let _e10989 = vec3<f32>(((_e8599.x * _e10924) - (_e8597.x * _e10965)), ((_e8599.y * _e10930) - (_e8597.y * _e10969)), ((_e8599.z * _e10936) - (_e8597.z * _e10973)));
                                                                        let _e10996 = ((_e8599.x * _e10965) + (_e8597.x * _e10924));
                                                                        let _e10997 = ((_e8599.y * _e10969) + (_e8597.y * _e10930));
                                                                        let _e10998 = ((_e8599.z * _e10973) + (_e8597.z * _e10936));
                                                                        let _e11000 = vec3<f32>((_e8599.x * _e10827), (_e8599.y * _e10827), (_e8599.z * _e10827));
                                                                        let _e11001 = (_e11000 - _e10937);
                                                                        let _e11002 = (_e10977 - _e10965);
                                                                        let _e11003 = (_e10978 - _e10969);
                                                                        let _e11004 = (_e10979 - _e10973);
                                                                        let _e11017 = (_e11000 + _e10937);
                                                                        let _e11018 = (_e10977 + _e10965);
                                                                        let _e11019 = (_e10978 + _e10969);
                                                                        let _e11020 = (_e10979 + _e10973);
                                                                        let _e11030 = ((_e11017.x * _e11017.x) + (_e11018 * _e11018));
                                                                        let _e11031 = ((_e11017.y * _e11017.y) + (_e11019 * _e11019));
                                                                        let _e11032 = ((_e11017.z * _e11017.z) + (_e11020 * _e11020));
                                                                        let _e11042 = vec3(_e10827);
                                                                        let _e11043 = (_e11042 - _e10989);
                                                                        let _e11044 = -(vec3<f32>(_e10996, _e10997, _e10998));
                                                                        let _e11060 = (_e11042 + _e10989);
                                                                        let _e11070 = ((_e11060.x * _e11060.x) + (_e10996 * _e10996));
                                                                        let _e11071 = ((_e11060.y * _e11060.y) + (_e10997 * _e10997));
                                                                        let _e11072 = ((_e11060.z * _e11060.z) + (_e10998 * _e10998));
                                                                        phi_15683_ = vec3<f32>((((((_e11001.x * _e11001.x) + (_e11002 * _e11002)) / select(0.000000000000000000000000000001f, _e11030, (_e11030 > 0.000000000000000000000000000001f))) + (((_e11043.x * _e11043.x) + (_e11044.x * _e11044.x)) / select(0.000000000000000000000000000001f, _e11070, (_e11070 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e11001.y * _e11001.y) + (_e11003 * _e11003)) / select(0.000000000000000000000000000001f, _e11031, (_e11031 > 0.000000000000000000000000000001f))) + (((_e11043.y * _e11043.y) + (_e11044.y * _e11044.y)) / select(0.000000000000000000000000000001f, _e11071, (_e11071 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e11001.z * _e11001.z) + (_e11004 * _e11004)) / select(0.000000000000000000000000000001f, _e11032, (_e11032 > 0.000000000000000000000000000001f))) + (((_e11043.z * _e11043.z) + (_e11044.z * _e11044.z)) / select(0.000000000000000000000000000001f, _e11072, (_e11072 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                                    }
                                                                    let _e11115 = phi_15683_;
                                                                    phi_15684_ = _e11115;
                                                                }
                                                                let _e11135 = phi_15684_;
                                                                let _e11137 = (4f * _e10749);
                                                                phi_15696_ = type_59((_e11135 * (((1f / (_e10800 + _e10814)) * _e10785) / _e11137)), ((_e10785 / _e10800) / _e11137));
                                                            }
                                                            let _e11144 = phi_15696_;
                                                            phi_15697_ = _e11144;
                                                            phi_15698_ = _e10761;
                                                        }
                                                        let _e11146 = phi_15697_;
                                                        let _e11148 = phi_15698_;
                                                        if _e11148 {
                                                            phi_15703_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                                        } else {
                                                            phi_15703_ = _e11146;
                                                        }
                                                        let _e11150 = phi_15703_;
                                                        let _e11153 = (_e8605 == 1u);
                                                        if _e11153 {
                                                            phi_15731_ = _e11150;
                                                            phi_15732_ = type_59();
                                                        } else {
                                                            if (_e10749 != _e10749) {
                                                                phi_27904_ = true;
                                                            } else {
                                                                phi_27904_ = (0f >= _e10749);
                                                            }
                                                            let _e11157 = phi_27904_;
                                                            let _e11158 = select(_e10749, 0f, _e11157);
                                                            let _e11159 = (1f / _e8595);
                                                            let _e11163 = ((_e11159 * _e11159) * (1f - (_e11158 * _e11158)));
                                                            if (_e11163 >= 1f) {
                                                                phi_27923_ = 1f;
                                                            } else {
                                                                let _e11166 = sqrt((1f - _e11163));
                                                                if (_e11158 != _e11158) {
                                                                    phi_27934_ = true;
                                                                } else {
                                                                    phi_27934_ = (_e11166 <= _e11158);
                                                                }
                                                                let _e11170 = phi_27934_;
                                                                let _e11175 = pow(((1f - _e8595) / (1f + _e8595)), 2f);
                                                                phi_27923_ = (_e11175 + ((1f - _e11175) * pow((1f - select(_e11158, _e11166, _e11170)), 5f)));
                                                            }
                                                            let _e11182 = phi_27923_;
                                                            if (_e10724 <= 0f) {
                                                                phi_27973_ = vec3<f32>(0f, 0f, 0f);
                                                            } else {
                                                                phi_27973_ = (_e8593 * (_e10724 * 0.31830987f));
                                                            }
                                                            let _e11187 = phi_27973_;
                                                            let _e11188 = (1f - _e11182);
                                                            phi_15731_ = type_59();
                                                            phi_15732_ = type_59((_e11150.member + vec3<f32>((_e11187.x * _e11188), (_e11187.y * _e11188), (_e11187.z * _e11188))), ((_e11182 * _e11150.member_1) + (_e11188 * _e10730)));
                                                        }
                                                        let _e11202 = phi_15731_;
                                                        let _e11204 = phi_15732_;
                                                        phi_15733_ = _e11202;
                                                        phi_15734_ = _e11204;
                                                        phi_15735_ = _e11153;
                                                    }
                                                    let _e11212 = phi_15733_;
                                                    let _e11214 = phi_15734_;
                                                    let _e11216 = phi_15735_;
                                                    if _e11216 {
                                                        phi_15739_ = _e11212;
                                                    } else {
                                                        phi_15739_ = _e11214;
                                                    }
                                                    let _e11218 = phi_15739_;
                                                    if (_e10701 > 0f) {
                                                        if (_e10701 >= 300000000000000000000000000000000000000f) {
                                                            phi_28012_ = 1f;
                                                        } else {
                                                            let _e11223 = (_e10701 * _e10701);
                                                            let _e11225 = (_e11223 + (_e11218.member_1 * _e11218.member_1));
                                                            if (_e11225 == 0f) {
                                                                phi_28009_ = 0f;
                                                            } else {
                                                                phi_28009_ = (_e11223 / _e11225);
                                                            }
                                                            let _e11229 = phi_28009_;
                                                            phi_28012_ = _e11229;
                                                        }
                                                        let _e11231 = phi_28012_;
                                                        phi_15748_ = (_e11231 / _e10701);
                                                    } else {
                                                        phi_15748_ = 1f;
                                                    }
                                                    let _e11234 = phi_15748_;
                                                    if ((((_e11218.member.x * _e11218.member.x) + (_e11218.member.y * _e11218.member.y)) + (_e11218.member.z * _e11218.member.z)) > 0f) {
                                                        let _e11248 = global_1.member.member_1[0u];
                                                        let _e11251 = global_1.member.member_1[1u];
                                                        let _e11253 = (vec3<f32>(1f, 1f, 1f) / _e10703);
                                                        local = array<u32, 100>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                                                        phi_15782_ = _e11248;
                                                        phi_15785_ = select(0u, 1u, (_e11248 > 0u));
                                                        phi_15787_ = (_e11248 + _e11251);
                                                        phi_15791_ = type_49(_e10707, 4294967295u, vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                                        loop {
                                                            let _e11257 = phi_15782_;
                                                            let _e11259 = phi_15785_;
                                                            let _e11261 = phi_15787_;
                                                            let _e11263 = phi_15791_;
                                                            local_118 = _e11263;
                                                            if (_e11257 < _e11261) {
                                                                let _e11432 = global_2.member[_e11257];
                                                                if (_e11432.member_6.x == 0u) {
                                                                    let _e11538 = (_e11432.member_1.x - _e11432.member.x);
                                                                    let _e11539 = (_e11432.member_1.y - _e11432.member.y);
                                                                    let _e11540 = (_e11432.member_1.z - _e11432.member.z);
                                                                    let _e11545 = (_e11432.member_2.x - _e11432.member.x);
                                                                    let _e11546 = (_e11432.member_2.y - _e11432.member.y);
                                                                    let _e11547 = (_e11432.member_2.z - _e11432.member.z);
                                                                    let _e11550 = ((_e10703.y * _e11547) - (_e11546 * _e10703.z));
                                                                    let _e11553 = ((_e10703.z * _e11545) - (_e11547 * _e10703.x));
                                                                    let _e11556 = ((_e10703.x * _e11546) - (_e11545 * _e10703.y));
                                                                    let _e11561 = (((_e11538 * _e11550) + (_e11539 * _e11553)) + (_e11540 * _e11556));
                                                                    if (abs(_e11561) < 0.000000000001f) {
                                                                        phi_15958_ = vec4<f32>();
                                                                        phi_15959_ = true;
                                                                    } else {
                                                                        let _e11564 = (1f / _e11561);
                                                                        let _e11565 = (_e7755 - _e11432.member.x);
                                                                        let _e11566 = (_e7756 - _e11432.member.y);
                                                                        let _e11567 = (_e7757 - _e11432.member.z);
                                                                        let _e11573 = ((((_e11565 * _e11550) + (_e11566 * _e11553)) + (_e11567 * _e11556)) * _e11564);
                                                                        if (_e11573 < 0f) {
                                                                            phi_15948_ = vec4<f32>();
                                                                            phi_15949_ = true;
                                                                            phi_15950_ = false;
                                                                        } else {
                                                                            let _e11575 = (_e11573 > 1f);
                                                                            if _e11575 {
                                                                                phi_15946_ = vec4<f32>();
                                                                                phi_15947_ = false;
                                                                            } else {
                                                                                let _e11578 = ((_e11566 * _e11540) - (_e11539 * _e11567));
                                                                                let _e11581 = ((_e11567 * _e11538) - (_e11540 * _e11565));
                                                                                let _e11584 = ((_e11565 * _e11539) - (_e11538 * _e11566));
                                                                                let _e11590 = ((((_e10703.x * _e11578) + (_e10703.y * _e11581)) + (_e10703.z * _e11584)) * _e11564);
                                                                                if (_e11590 < 0f) {
                                                                                    phi_15936_ = vec4<f32>();
                                                                                    phi_15937_ = true;
                                                                                    phi_15938_ = false;
                                                                                } else {
                                                                                    let _e11593 = ((_e11573 + _e11590) > 1f);
                                                                                    if _e11593 {
                                                                                        phi_15934_ = vec4<f32>();
                                                                                        phi_15935_ = false;
                                                                                    } else {
                                                                                        let _e11599 = ((((_e11545 * _e11578) + (_e11546 * _e11581)) + (_e11547 * _e11584)) * _e11564);
                                                                                        if (_e11599 <= 0f) {
                                                                                            phi_15929_ = vec4<f32>();
                                                                                            phi_15930_ = true;
                                                                                        } else {
                                                                                            let _e11601 = (_e11599 >= _e11263.member);
                                                                                            if _e11601 {
                                                                                                phi_15928_ = vec4<f32>();
                                                                                            } else {
                                                                                                phi_15928_ = vec4<f32>(_e11599, _e11573, _e11590, 1f);
                                                                                            }
                                                                                            let _e11604 = phi_15928_;
                                                                                            phi_15929_ = _e11604;
                                                                                            phi_15930_ = _e11601;
                                                                                        }
                                                                                        let _e11606 = phi_15929_;
                                                                                        let _e11608 = phi_15930_;
                                                                                        phi_15934_ = _e11606;
                                                                                        phi_15935_ = _e11608;
                                                                                    }
                                                                                    let _e11610 = phi_15934_;
                                                                                    let _e11612 = phi_15935_;
                                                                                    phi_15936_ = _e11610;
                                                                                    phi_15937_ = _e11593;
                                                                                    phi_15938_ = _e11612;
                                                                                }
                                                                                let _e11614 = phi_15936_;
                                                                                let _e11616 = phi_15937_;
                                                                                let _e11618 = phi_15938_;
                                                                                phi_15946_ = _e11614;
                                                                                phi_15947_ = select(_e11618, true, _e11616);
                                                                            }
                                                                            let _e11621 = phi_15946_;
                                                                            let _e11623 = phi_15947_;
                                                                            phi_15948_ = _e11621;
                                                                            phi_15949_ = _e11575;
                                                                            phi_15950_ = _e11623;
                                                                        }
                                                                        let _e11625 = phi_15948_;
                                                                        let _e11627 = phi_15949_;
                                                                        let _e11629 = phi_15950_;
                                                                        phi_15958_ = _e11625;
                                                                        phi_15959_ = select(_e11629, true, _e11627);
                                                                    }
                                                                    let _e11632 = phi_15958_;
                                                                    let _e11634 = phi_15959_;
                                                                    phi_16105_ = _e11632;
                                                                    phi_16106_ = _e11634;
                                                                } else {
                                                                    if (_e11432.member_6.x == 1u) {
                                                                        let _e11460 = (_e7755 - _e11432.member.x);
                                                                        let _e11461 = (_e7756 - _e11432.member.y);
                                                                        let _e11462 = (_e7757 - _e11432.member.z);
                                                                        let _e11467 = (((_e11460 * _e10703.x) + (_e11461 * _e10703.y)) + (_e11462 * _e10703.z));
                                                                        if (_e11467 > 0f) {
                                                                            phi_15999_ = select(true, false, ((((_e11460 * _e11460) + (_e11461 * _e11461)) + (_e11462 * _e11462)) > (_e11432.member.w * _e11432.member.w)));
                                                                        } else {
                                                                            phi_15999_ = true;
                                                                        }
                                                                        let _e11480 = phi_15999_;
                                                                        if _e11480 {
                                                                            let _e11484 = (_e11460 - (_e10703.x * _e11467));
                                                                            let _e11485 = (_e11461 - (_e10703.y * _e11467));
                                                                            let _e11486 = (_e11462 - (_e10703.z * _e11467));
                                                                            let _e11495 = ((_e11432.member.w * _e11432.member.w) - (((_e11484 * _e11484) + (_e11485 * _e11485)) + (_e11486 * _e11486)));
                                                                            if (_e11495 < 0f) {
                                                                                phi_16058_ = vec4<f32>();
                                                                                phi_16059_ = true;
                                                                            } else {
                                                                                let _e11497 = sqrt(_e11495);
                                                                                let _e11498 = -(_e11467);
                                                                                let _e11499 = (_e11498 - _e11497);
                                                                                if (_e11499 <= 0f) {
                                                                                    phi_16036_ = true;
                                                                                } else {
                                                                                    phi_16036_ = (_e11499 >= _e11263.member);
                                                                                }
                                                                                let _e11503 = phi_16036_;
                                                                                if _e11503 {
                                                                                    phi_16042_ = (_e11498 + _e11497);
                                                                                } else {
                                                                                    phi_16042_ = _e11499;
                                                                                }
                                                                                let _e11506 = phi_16042_;
                                                                                if (_e11506 <= 0f) {
                                                                                    phi_16053_ = vec4<f32>();
                                                                                    phi_16054_ = true;
                                                                                } else {
                                                                                    let _e11508 = (_e11506 >= _e11263.member);
                                                                                    if _e11508 {
                                                                                        phi_16052_ = vec4<f32>();
                                                                                    } else {
                                                                                        phi_16052_ = vec4<f32>(_e11506, 0f, 0f, 1f);
                                                                                    }
                                                                                    let _e11511 = phi_16052_;
                                                                                    phi_16053_ = _e11511;
                                                                                    phi_16054_ = _e11508;
                                                                                }
                                                                                let _e11513 = phi_16053_;
                                                                                let _e11515 = phi_16054_;
                                                                                phi_16058_ = _e11513;
                                                                                phi_16059_ = _e11515;
                                                                            }
                                                                            let _e11517 = phi_16058_;
                                                                            let _e11519 = phi_16059_;
                                                                            phi_16063_ = _e11517;
                                                                            phi_16064_ = _e11519;
                                                                        } else {
                                                                            phi_16063_ = vec4<f32>();
                                                                            phi_16064_ = true;
                                                                        }
                                                                        let _e11521 = phi_16063_;
                                                                        let _e11523 = phi_16064_;
                                                                        phi_16095_ = _e11521;
                                                                        phi_16096_ = _e11523;
                                                                    } else {
                                                                        if (abs(_e10703.y) < 0.000000000001f) {
                                                                            phi_16093_ = vec4<f32>();
                                                                            phi_16094_ = true;
                                                                        } else {
                                                                            let _e11442 = ((_e11432.member.x - _e7756) / _e10703.y);
                                                                            if (_e11442 <= 0f) {
                                                                                phi_16088_ = vec4<f32>();
                                                                                phi_16089_ = true;
                                                                            } else {
                                                                                let _e11444 = (_e11442 >= _e11263.member);
                                                                                if _e11444 {
                                                                                    phi_16087_ = vec4<f32>();
                                                                                } else {
                                                                                    phi_16087_ = vec4<f32>(_e11442, 0f, 0f, 1f);
                                                                                }
                                                                                let _e11447 = phi_16087_;
                                                                                phi_16088_ = _e11447;
                                                                                phi_16089_ = _e11444;
                                                                            }
                                                                            let _e11449 = phi_16088_;
                                                                            let _e11451 = phi_16089_;
                                                                            phi_16093_ = _e11449;
                                                                            phi_16094_ = _e11451;
                                                                        }
                                                                        let _e11453 = phi_16093_;
                                                                        let _e11455 = phi_16094_;
                                                                        phi_16095_ = _e11453;
                                                                        phi_16096_ = _e11455;
                                                                    }
                                                                    let _e11525 = phi_16095_;
                                                                    let _e11527 = phi_16096_;
                                                                    phi_16105_ = _e11525;
                                                                    phi_16106_ = select(true, false, select(true, false, _e11527));
                                                                }
                                                                let _e11636 = phi_16105_;
                                                                let _e11638 = phi_16106_;
                                                                let _e11641 = select(vec4<f32>(0f, 0f, 0f, 0f), _e11636, vec4(select(true, false, _e11638)));
                                                                let _e11643 = (_e11641.w > 0f);
                                                                if _e11643 {
                                                                    let _e11649 = type_49(_e11641.x, _e11263.member_1, _e11263.member_2, _e11263.member_3);
                                                                    let _e11654 = type_49(_e11649.member, _e11257, _e11649.member_2, _e11649.member_3);
                                                                    phi_16127_ = _e11654;
                                                                    phi_16129_ = _e11654;
                                                                } else {
                                                                    phi_16127_ = type_49();
                                                                    phi_16129_ = _e11263;
                                                                }
                                                                let _e11656 = phi_16127_;
                                                                let _e11658 = phi_16129_;
                                                                let _e11659 = select(true, false, _e11643);
                                                                if _e11659 {
                                                                    phi_16135_ = (_e11257 + 1u);
                                                                } else {
                                                                    phi_16135_ = u32();
                                                                }
                                                                let _e11662 = phi_16135_;
                                                                phi_15783_ = _e11662;
                                                                phi_15786_ = _e11259;
                                                                phi_15788_ = _e11261;
                                                                phi_15792_ = _e11658;
                                                                phi_16208_ = _e11656;
                                                                phi_16209_ = _e11659;
                                                                phi_16210_ = false;
                                                            } else {
                                                                let _e11265 = (_e11259 == 0u);
                                                                if _e11265 {
                                                                    phi_16204_ = u32();
                                                                    phi_16205_ = u32();
                                                                    phi_16206_ = u32();
                                                                } else {
                                                                    let _e11266 = (_e11259 - 1u);
                                                                    let _e11267 = (_e11266 < 100u);
                                                                    if _e11267 {
                                                                    } else {
                                                                        phi_33331_ = true;
                                                                        phi_19885_ = bool();
                                                                        phi_19884_ = type_49();
                                                                        break;
                                                                    }
                                                                    let _e11269 = local[_e11266];
                                                                    let _e11271 = global_3.member[_e11269];
                                                                    let _e11282 = ((_e11271.member.x - _e7755) * _e11253.x);
                                                                    let _e11284 = ((_e11271.member.y - _e7756) * _e11253.y);
                                                                    let _e11286 = ((_e11271.member.z - _e7757) * _e11253.z);
                                                                    let _e11293 = ((_e11271.member_2.x - _e7755) * _e11253.x);
                                                                    let _e11294 = ((_e11271.member_2.y - _e7756) * _e11253.y);
                                                                    let _e11295 = ((_e11271.member_2.z - _e7757) * _e11253.z);
                                                                    if (_e11282 == _e11282) {
                                                                        let _e11297 = (_e11293 == _e11293);
                                                                        if _e11297 {
                                                                            let _e11298 = (_e11282 != _e11282);
                                                                            if _e11298 {
                                                                                phi_28109_ = true;
                                                                            } else {
                                                                                phi_28109_ = (_e11293 <= _e11282);
                                                                            }
                                                                            let _e11301 = phi_28109_;
                                                                            let _e11302 = select(_e11282, _e11293, _e11301);
                                                                            if _e11298 {
                                                                                phi_28139_ = true;
                                                                            } else {
                                                                                phi_28139_ = (_e11293 >= _e11282);
                                                                            }
                                                                            let _e11307 = phi_28139_;
                                                                            let _e11308 = select(_e11282, _e11293, _e11307);
                                                                            phi_28086_ = type_52(select(-300000000000000000000000000000000000000f, _e11302, (_e11302 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e11308, (_e11308 <= 300000000000000000000000000000000000000f)));
                                                                        } else {
                                                                            phi_28086_ = type_52();
                                                                        }
                                                                        let _e11313 = phi_28086_;
                                                                        phi_28089_ = _e11313;
                                                                        phi_28090_ = select(true, false, _e11297);
                                                                    } else {
                                                                        phi_28089_ = type_52();
                                                                        phi_28090_ = true;
                                                                    }
                                                                    let _e11316 = phi_28089_;
                                                                    let _e11318 = phi_28090_;
                                                                    if _e11318 {
                                                                        phi_28095_ = type_52(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                                    } else {
                                                                        phi_28095_ = _e11316;
                                                                    }
                                                                    let _e11320 = phi_28095_;
                                                                    if (_e11284 == _e11284) {
                                                                        let _e11324 = (_e11294 == _e11294);
                                                                        if _e11324 {
                                                                            let _e11325 = (_e11284 != _e11284);
                                                                            if _e11325 {
                                                                                phi_28196_ = true;
                                                                            } else {
                                                                                phi_28196_ = (_e11294 <= _e11284);
                                                                            }
                                                                            let _e11328 = phi_28196_;
                                                                            let _e11329 = select(_e11284, _e11294, _e11328);
                                                                            if (_e11320.member != _e11320.member) {
                                                                                phi_28211_ = true;
                                                                            } else {
                                                                                phi_28211_ = (_e11329 >= _e11320.member);
                                                                            }
                                                                            let _e11333 = phi_28211_;
                                                                            if _e11325 {
                                                                                phi_28226_ = true;
                                                                            } else {
                                                                                phi_28226_ = (_e11294 >= _e11284);
                                                                            }
                                                                            let _e11337 = phi_28226_;
                                                                            let _e11338 = select(_e11284, _e11294, _e11337);
                                                                            if (_e11320.member_1 != _e11320.member_1) {
                                                                                phi_28241_ = true;
                                                                            } else {
                                                                                phi_28241_ = (_e11338 <= _e11320.member_1);
                                                                            }
                                                                            let _e11342 = phi_28241_;
                                                                            phi_28173_ = type_52(select(_e11320.member, _e11329, _e11333), select(_e11320.member_1, _e11338, _e11342));
                                                                        } else {
                                                                            phi_28173_ = type_52();
                                                                        }
                                                                        let _e11346 = phi_28173_;
                                                                        phi_28176_ = _e11346;
                                                                        phi_28177_ = select(true, false, _e11324);
                                                                    } else {
                                                                        phi_28176_ = type_52();
                                                                        phi_28177_ = true;
                                                                    }
                                                                    let _e11349 = phi_28176_;
                                                                    let _e11351 = phi_28177_;
                                                                    if _e11351 {
                                                                        phi_28182_ = _e11320;
                                                                    } else {
                                                                        phi_28182_ = _e11349;
                                                                    }
                                                                    let _e11353 = phi_28182_;
                                                                    if (_e11286 == _e11286) {
                                                                        let _e11357 = (_e11295 == _e11295);
                                                                        if _e11357 {
                                                                            let _e11358 = (_e11286 != _e11286);
                                                                            if _e11358 {
                                                                                phi_28283_ = true;
                                                                            } else {
                                                                                phi_28283_ = (_e11295 <= _e11286);
                                                                            }
                                                                            let _e11361 = phi_28283_;
                                                                            let _e11362 = select(_e11286, _e11295, _e11361);
                                                                            if (_e11353.member != _e11353.member) {
                                                                                phi_28298_ = true;
                                                                            } else {
                                                                                phi_28298_ = (_e11362 >= _e11353.member);
                                                                            }
                                                                            let _e11366 = phi_28298_;
                                                                            if _e11358 {
                                                                                phi_28313_ = true;
                                                                            } else {
                                                                                phi_28313_ = (_e11295 >= _e11286);
                                                                            }
                                                                            let _e11370 = phi_28313_;
                                                                            let _e11371 = select(_e11286, _e11295, _e11370);
                                                                            if (_e11353.member_1 != _e11353.member_1) {
                                                                                phi_28328_ = true;
                                                                            } else {
                                                                                phi_28328_ = (_e11371 <= _e11353.member_1);
                                                                            }
                                                                            let _e11375 = phi_28328_;
                                                                            phi_28260_ = type_52(select(_e11353.member, _e11362, _e11366), select(_e11353.member_1, _e11371, _e11375));
                                                                        } else {
                                                                            phi_28260_ = type_52();
                                                                        }
                                                                        let _e11379 = phi_28260_;
                                                                        phi_28263_ = _e11379;
                                                                        phi_28264_ = select(true, false, _e11357);
                                                                    } else {
                                                                        phi_28263_ = type_52();
                                                                        phi_28264_ = true;
                                                                    }
                                                                    let _e11382 = phi_28263_;
                                                                    let _e11384 = phi_28264_;
                                                                    if _e11384 {
                                                                        phi_28269_ = _e11353;
                                                                    } else {
                                                                        phi_28269_ = _e11382;
                                                                    }
                                                                    let _e11386 = phi_28269_;
                                                                    if (_e11386.member_1 < 0f) {
                                                                        phi_28065_ = bool();
                                                                        phi_28066_ = true;
                                                                    } else {
                                                                        let _e11390 = (_e11386.member > _e11386.member_1);
                                                                        if _e11390 {
                                                                            phi_28062_ = bool();
                                                                        } else {
                                                                            phi_28062_ = (_e11386.member > _e11263.member);
                                                                        }
                                                                        let _e11393 = phi_28062_;
                                                                        phi_28065_ = _e11393;
                                                                        phi_28066_ = _e11390;
                                                                    }
                                                                    let _e11395 = phi_28065_;
                                                                    let _e11397 = phi_28066_;
                                                                    if !(select(_e11395, true, _e11397)) {
                                                                        if (_e11271.member_3 > 0u) {
                                                                            phi_16198_ = _e11271.member_1;
                                                                            phi_16199_ = _e11266;
                                                                            phi_16200_ = (_e11271.member_1 + _e11271.member_3);
                                                                        } else {
                                                                            let _e11402 = (_e11259 + 1u);
                                                                            let _e11403 = (_e11402 <= 100u);
                                                                            if _e11403 {
                                                                                if _e11267 {
                                                                                } else {
                                                                                    phi_33331_ = true;
                                                                                    phi_19885_ = bool();
                                                                                    phi_19884_ = type_49();
                                                                                    break;
                                                                                }
                                                                                local[_e11266] = _e11271.member_1;
                                                                                if (_e11259 < 100u) {
                                                                                } else {
                                                                                    phi_33331_ = true;
                                                                                    phi_19885_ = bool();
                                                                                    phi_19884_ = type_49();
                                                                                    break;
                                                                                }
                                                                                local[_e11259] = (_e11269 + 1u);
                                                                            }
                                                                            phi_16198_ = _e11257;
                                                                            phi_16199_ = select(_e11266, _e11402, _e11403);
                                                                            phi_16200_ = _e11261;
                                                                        }
                                                                        let _e11412 = phi_16198_;
                                                                        let _e11414 = phi_16199_;
                                                                        let _e11416 = phi_16200_;
                                                                        phi_16201_ = _e11412;
                                                                        phi_16202_ = _e11414;
                                                                        phi_16203_ = _e11416;
                                                                    } else {
                                                                        phi_16201_ = _e11257;
                                                                        phi_16202_ = _e11266;
                                                                        phi_16203_ = _e11261;
                                                                    }
                                                                    let _e11418 = phi_16201_;
                                                                    let _e11420 = phi_16202_;
                                                                    let _e11422 = phi_16203_;
                                                                    phi_16204_ = _e11418;
                                                                    phi_16205_ = _e11420;
                                                                    phi_16206_ = _e11422;
                                                                }
                                                                let _e11424 = phi_16204_;
                                                                let _e11426 = phi_16205_;
                                                                let _e11428 = phi_16206_;
                                                                phi_15783_ = _e11424;
                                                                phi_15786_ = _e11426;
                                                                phi_15788_ = _e11428;
                                                                phi_15792_ = _e11263;
                                                                phi_16208_ = type_49();
                                                                phi_16209_ = select(true, false, _e11265);
                                                                phi_16210_ = _e11265;
                                                            }
                                                            let _e11664 = phi_15783_;
                                                            let _e11666 = phi_15786_;
                                                            let _e11668 = phi_15788_;
                                                            let _e11670 = phi_15792_;
                                                            let _e11672 = phi_16208_;
                                                            let _e11674 = phi_16209_;
                                                            let _e11676 = phi_16210_;
                                                            phi_15782_ = _e11664;
                                                            phi_15785_ = _e11666;
                                                            phi_15787_ = _e11668;
                                                            phi_15791_ = _e11670;
                                                            phi_33331_ = _e8628;
                                                            phi_19885_ = _e11676;
                                                            phi_19884_ = _e11672;
                                                            if !(_e11674) {
                                                                break;
                                                            }
                                                        }
                                                        let _e11679 = phi_33331_;
                                                        let _e11681 = phi_19885_;
                                                        let _e11683 = phi_19884_;
                                                        phi_33729_ = _e10699;
                                                        phi_33715_ = _e11679;
                                                        if _e11679 {
                                                            break;
                                                        }
                                                        if _e11681 {
                                                            let _e14935 = local_118;
                                                            phi_16382_ = _e14935;
                                                        } else {
                                                            phi_16382_ = type_49();
                                                        }
                                                        let _e11685 = phi_16382_;
                                                        if select(true, false, _e11681) {
                                                            phi_16387_ = _e11683;
                                                        } else {
                                                            phi_16387_ = _e11685;
                                                        }
                                                        let _e11688 = phi_16387_;
                                                        if (_e11688.member_1 != 4294967295u) {
                                                            phi_16442_ = _e8634;
                                                            phi_16443_ = _e8636;
                                                            phi_16444_ = _e8640;
                                                            phi_16445_ = _e8642;
                                                        } else {
                                                            let _e11692 = (_e8638 == (_e8626 + 2u));
                                                            if _e11692 {
                                                                phi_16438_ = _e8634;
                                                                phi_16439_ = vec3<f32>(((_e7617.x * _e11218.member.x) * _e11234), ((_e7617.y * _e11218.member.y) * _e11234), ((_e7617.z * _e11218.member.z) * _e11234));
                                                            } else {
                                                                phi_16438_ = vec3<f32>((_e8634.x + (((_e7617.x * _e11218.member.x) * _e10705.x) * _e11234)), (_e8634.y + (((_e7617.y * _e11218.member.y) * _e10705.y) * _e11234)), (_e8634.z + (((_e7617.z * _e11218.member.z) * _e10705.z) * _e11234)));
                                                                phi_16439_ = _e8636;
                                                            }
                                                            let _e11723 = phi_16438_;
                                                            let _e11725 = phi_16439_;
                                                            let _e11726 = vec3(_e11692);
                                                            phi_16442_ = _e11723;
                                                            phi_16443_ = _e11725;
                                                            phi_16444_ = select(_e8640, _e10703, _e11726);
                                                            phi_16445_ = select(_e8642, _e7758, _e11726);
                                                        }
                                                        let _e11730 = phi_16442_;
                                                        let _e11732 = phi_16443_;
                                                        let _e11734 = phi_16444_;
                                                        let _e11736 = phi_16445_;
                                                        phi_33718_ = _e11679;
                                                        phi_16446_ = (_e8632 + 1u);
                                                        phi_16447_ = _e11730;
                                                        phi_16448_ = _e11732;
                                                        phi_16449_ = _e11734;
                                                        phi_16450_ = _e11736;
                                                    } else {
                                                        phi_33718_ = _e8628;
                                                        phi_16446_ = _e8632;
                                                        phi_16447_ = _e8634;
                                                        phi_16448_ = _e8636;
                                                        phi_16449_ = _e8640;
                                                        phi_16450_ = _e8642;
                                                    }
                                                    let _e11738 = phi_33718_;
                                                    let _e11740 = phi_16446_;
                                                    let _e11742 = phi_16447_;
                                                    let _e11744 = phi_16448_;
                                                    let _e11746 = phi_16449_;
                                                    let _e11748 = phi_16450_;
                                                    phi_33717_ = _e11738;
                                                    phi_16451_ = _e11740;
                                                    phi_16452_ = _e11742;
                                                    phi_16453_ = _e11744;
                                                    phi_16454_ = _e11746;
                                                    phi_16455_ = _e11748;
                                                } else {
                                                    phi_33717_ = _e8628;
                                                    phi_16451_ = _e8632;
                                                    phi_16452_ = _e8634;
                                                    phi_16453_ = _e8636;
                                                    phi_16454_ = _e8640;
                                                    phi_16455_ = _e8642;
                                                }
                                                let _e11750 = phi_33717_;
                                                let _e11752 = phi_16451_;
                                                let _e11754 = phi_16452_;
                                                let _e11756 = phi_16453_;
                                                let _e11758 = phi_16454_;
                                                let _e11760 = phi_16455_;
                                                phi_33960_ = _e10699;
                                                phi_33716_ = _e11750;
                                                phi_12289_ = _e11752;
                                                phi_12292_ = _e11754;
                                                phi_12294_ = _e11756;
                                                phi_12296_ = (_e8638 + 1u);
                                                phi_12298_ = _e11758;
                                                phi_12300_ = _e11760;
                                            } else {
                                                phi_33960_ = _e8630;
                                                phi_33716_ = _e8628;
                                                phi_12289_ = u32();
                                                phi_12292_ = vec3<f32>();
                                                phi_12294_ = vec3<f32>();
                                                phi_12296_ = u32();
                                                phi_12298_ = vec3<f32>();
                                                phi_12300_ = vec3<f32>();
                                            }
                                            let _e11763 = phi_33960_;
                                            let _e11765 = phi_33716_;
                                            let _e11767 = phi_12289_;
                                            let _e11769 = phi_12292_;
                                            let _e11771 = phi_12294_;
                                            let _e11773 = phi_12296_;
                                            let _e11775 = phi_12298_;
                                            let _e11777 = phi_12300_;
                                            phi_33411_ = _e11765;
                                            phi_31479_ = _e11763;
                                            phi_12288_ = _e11767;
                                            phi_12291_ = _e11769;
                                            phi_12293_ = _e11771;
                                            phi_12295_ = _e11773;
                                            phi_12297_ = _e11775;
                                            phi_12299_ = _e11777;
                                            phi_33729_ = _e11763;
                                            phi_33715_ = _e11765;
                                            if !(_e8644) {
                                                break;
                                            }
                                        }
                                        let _e11780 = phi_33729_;
                                        let _e11782 = phi_33715_;
                                        phi_35216_ = _e11782;
                                        phi_19906_ = type_49();
                                        phi_19905_ = u32();
                                        phi_19904_ = vec3<f32>();
                                        phi_19903_ = bool();
                                        phi_19902_ = bool();
                                        phi_19901_ = bool();
                                        phi_19900_ = bool();
                                        phi_19899_ = vec3<f32>();
                                        phi_19897_ = u32();
                                        phi_19895_ = type_14();
                                        phi_19893_ = vec3<f32>();
                                        phi_19892_ = f32();
                                        phi_19891_ = f32();
                                        phi_19890_ = f32();
                                        phi_19888_ = vec3<f32>();
                                        if _e11782 {
                                            break;
                                        }
                                        if (_e393 >= _e365) {
                                            phi_35321_ = _e11780;
                                            phi_17003_ = _e7617;
                                            phi_17004_ = vec3<f32>();
                                            phi_17005_ = vec3<f32>();
                                            phi_17006_ = u32();
                                            phi_17007_ = f32();
                                            phi_17008_ = vec3<f32>();
                                            phi_17009_ = true;
                                        } else {
                                            let _e11785 = ((_e11780 * 747796405u) + 2891336453u);
                                            let _e11793 = (((_e11785 >> bitcast<u32>((((_e11785 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e11785) * 277803737u);
                                            let _e11796 = ((_e11793 >> bitcast<u32>(22i)) ^ _e11793);
                                            let _e11799 = f32((_e11796 >> bitcast<u32>(8i)));
                                            let _e11802 = ((_e11796 * 747796405u) + 2891336453u);
                                            let _e11810 = (((_e11802 >> bitcast<u32>((((_e11802 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e11802) * 277803737u);
                                            let _e11813 = ((_e11810 >> bitcast<u32>(22i)) ^ _e11810);
                                            let _e11816 = f32((_e11813 >> bitcast<u32>(8i)));
                                            let _e11818 = ((_e11813 * 747796405u) + 2891336453u);
                                            let _e11826 = (((_e11818 >> bitcast<u32>((((_e11818 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e11818) * 277803737u);
                                            let _e11829 = ((_e11826 >> bitcast<u32>(22i)) ^ _e11826);
                                            let _e11834 = (_e8605 == 0u);
                                            if _e11834 {
                                                phi_28423_ = vec3<f32>();
                                                phi_28424_ = true;
                                            } else {
                                                if (_e8605 == 2u) {
                                                    let _e11843 = (((_e7704.x * _e7730.x) + (_e7704.y * _e7730.y)) + (_e7704.z * _e7730.z));
                                                    if (_e11843 != _e11843) {
                                                        phi_28464_ = true;
                                                    } else {
                                                        phi_28464_ = (0f >= _e11843);
                                                    }
                                                    let _e11847 = phi_28464_;
                                                    let _e11848 = select(_e11843, 0f, _e11847);
                                                    let _e11849 = (1f / _e8595);
                                                    let _e11853 = ((_e11849 * _e11849) * (1f - (_e11848 * _e11848)));
                                                    if (_e11853 >= 1f) {
                                                        phi_28483_ = 1f;
                                                    } else {
                                                        let _e11856 = sqrt((1f - _e11853));
                                                        if (_e11848 != _e11848) {
                                                            phi_28494_ = true;
                                                        } else {
                                                            phi_28494_ = (_e11856 <= _e11848);
                                                        }
                                                        let _e11860 = phi_28494_;
                                                        let _e11865 = pow(((1f - _e8595) / (1f + _e8595)), 2f);
                                                        phi_28483_ = (_e11865 + ((1f - _e11865) * pow((1f - select(_e11848, _e11856, _e11860)), 5f)));
                                                    }
                                                    let _e11872 = phi_28483_;
                                                    phi_28414_ = select(true, false, ((f32((_e11829 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e11872));
                                                } else {
                                                    phi_28414_ = true;
                                                }
                                                let _e11876 = phi_28414_;
                                                if _e11876 {
                                                    let _e11884 = (((_e7704.x * _e7776) + (_e7704.y * _e7777)) + (_e7704.z * _e7779));
                                                    let _e11889 = (((_e7704.x * _e7772) + (_e7704.y * _e7782)) + (_e7704.z * _e7783));
                                                    let _e11894 = (((_e7704.x * _e7730.x) + (_e7704.y * _e7730.y)) + (_e7704.z * _e7730.z));
                                                    if (_e11894 <= 0f) {
                                                        phi_28648_ = vec3<f32>();
                                                        phi_28649_ = true;
                                                    } else {
                                                        let _e11897 = (_e8603 * _e11884);
                                                        let _e11898 = (_e8603 * _e11889);
                                                        let _e11905 = (1f / sqrt((((_e11897 * _e11897) + (_e11898 * _e11898)) + (_e11894 * _e11894))));
                                                        let _e11906 = (_e11897 * _e11905);
                                                        let _e11907 = (_e11898 * _e11905);
                                                        let _e11908 = (_e11894 * _e11905);
                                                        let _e11911 = ((_e11906 * _e11906) + (_e11907 * _e11907));
                                                        if (_e11911 > 0f) {
                                                            let _e11914 = sqrt(_e11911);
                                                            phi_28550_ = vec3<f32>((-(_e11907) / _e11914), (_e11906 / _e11914), 0f);
                                                        } else {
                                                            phi_28550_ = vec3<f32>(1f, 0f, 0f);
                                                        }
                                                        let _e11919 = phi_28550_;
                                                        let _e11932 = sqrt((_e11799 * 0.000000059604645f));
                                                        let _e11933 = (_e11816 * 0.00000037450704f);
                                                        let _e11935 = (_e11932 * cos(_e11933));
                                                        let _e11937 = (0.5f * (1f + _e11908));
                                                        let _e11940 = (1f - (_e11935 * _e11935));
                                                        if (_e11940 != _e11940) {
                                                            phi_28706_ = true;
                                                        } else {
                                                            phi_28706_ = (0f >= _e11940);
                                                        }
                                                        let _e11944 = phi_28706_;
                                                        let _e11951 = (((1f - _e11937) * sqrt(select(_e11940, 0f, _e11944))) + ((_e11937 * _e11932) * sin(_e11933)));
                                                        let _e11962 = (_e11940 - (_e11951 * _e11951));
                                                        if (_e11962 != _e11962) {
                                                            phi_28721_ = true;
                                                        } else {
                                                            phi_28721_ = (0f >= _e11962);
                                                        }
                                                        let _e11966 = phi_28721_;
                                                        let _e11968 = sqrt(select(_e11962, 0f, _e11966));
                                                        let _e11974 = (((_e11919.z * _e11935) + (((_e11906 * _e11919.y) - (_e11919.x * _e11907)) * _e11951)) + (_e11908 * _e11968));
                                                        let _e11975 = (_e8603 * (((_e11919.x * _e11935) + (((_e11907 * _e11919.z) - (_e11919.y * _e11908)) * _e11951)) + (_e11906 * _e11968)));
                                                        let _e11976 = (_e8603 * (((_e11919.y * _e11935) + (((_e11908 * _e11919.x) - (_e11919.z * _e11906)) * _e11951)) + (_e11907 * _e11968)));
                                                        if (_e11974 != _e11974) {
                                                            phi_28736_ = true;
                                                        } else {
                                                            phi_28736_ = (0f >= _e11974);
                                                        }
                                                        let _e11980 = phi_28736_;
                                                        let _e11981 = select(_e11974, 0f, _e11980);
                                                        let _e11988 = (1f / sqrt((((_e11975 * _e11975) + (_e11976 * _e11976)) + (_e11981 * _e11981))));
                                                        let _e11989 = (_e11975 * _e11988);
                                                        let _e11990 = (_e11976 * _e11988);
                                                        let _e11991 = (_e11981 * _e11988);
                                                        let _e11997 = (2f * (((_e11884 * _e11989) + (_e11889 * _e11990)) + (_e11894 * _e11991)));
                                                        let _e12000 = (_e11991 * _e11997);
                                                        let _e12003 = (vec3<f32>((_e11989 * _e11997), (_e11990 * _e11997), _e12000) - vec3<f32>(_e11884, _e11889, _e11894));
                                                        let _e12004 = ((_e12000 - _e11894) <= 0f);
                                                        if _e12004 {
                                                            phi_28645_ = vec3<f32>();
                                                        } else {
                                                            phi_28645_ = vec3<f32>((((_e7776 * _e12003.x) + (_e7772 * _e12003.y)) + (_e7730.x * _e12003.z)), (((_e7777 * _e12003.x) + (_e7782 * _e12003.y)) + (_e7730.y * _e12003.z)), (((_e7779 * _e12003.x) + (_e7783 * _e12003.y)) + (_e7730.z * _e12003.z)));
                                                        }
                                                        let _e12025 = phi_28645_;
                                                        phi_28648_ = _e12025;
                                                        phi_28649_ = _e12004;
                                                    }
                                                    let _e12027 = phi_28648_;
                                                    let _e12029 = phi_28649_;
                                                    phi_28419_ = select(_e12027, vec3<f32>(0f, 0f, 0f), vec3(_e12029));
                                                } else {
                                                    phi_28419_ = vec3<f32>();
                                                }
                                                let _e12033 = phi_28419_;
                                                phi_28423_ = _e12033;
                                                phi_28424_ = select(true, false, _e11876);
                                            }
                                            let _e12036 = phi_28423_;
                                            let _e12038 = phi_28424_;
                                            if _e12038 {
                                                let _e12039 = (_e11799 * 0.00000037450704f);
                                                let _e12041 = ((_e11816 * 0.00000011920929f) - 1f);
                                                let _e12043 = (1f - (_e12041 * _e12041));
                                                if (_e12043 != _e12043) {
                                                    phi_28836_ = true;
                                                } else {
                                                    phi_28836_ = (0f >= _e12043);
                                                }
                                                let _e12047 = phi_28836_;
                                                let _e12049 = sqrt(select(_e12043, 0f, _e12047));
                                                let _e12054 = (_e7730.x + (_e12049 * cos(_e12039)));
                                                let _e12055 = (_e7730.y + (_e12049 * sin(_e12039)));
                                                let _e12056 = (_e7730.z + _e12041);
                                                let _e12062 = sqrt((((_e12054 * _e12054) + (_e12055 * _e12055)) + (_e12056 * _e12056)));
                                                if (_e12062 < 0.000001f) {
                                                    phi_28810_ = _e7730;
                                                } else {
                                                    let _e12064 = (1f / _e12062);
                                                    phi_28810_ = vec3<f32>((_e12054 * _e12064), (_e12055 * _e12064), (_e12056 * _e12064));
                                                }
                                                let _e12070 = phi_28810_;
                                                phi_28431_ = _e12070;
                                            } else {
                                                phi_28431_ = _e12036;
                                            }
                                            let _e12072 = phi_28431_;
                                            let _e12080 = (((_e12072.x * _e7730.x) + (_e12072.y * _e7730.y)) + (_e12072.z * _e7730.z));
                                            if (_e12080 != _e12080) {
                                                phi_28851_ = true;
                                            } else {
                                                phi_28851_ = (0f >= _e12080);
                                            }
                                            let _e12084 = phi_28851_;
                                            let _e12086 = (select(_e12080, 0f, _e12084) * 0.31830987f);
                                            if _e11834 {
                                                if (_e12080 <= 0f) {
                                                    phi_29648_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_29648_ = (_e8599 * (_e12080 * 0.31830987f));
                                                }
                                                let _e12564 = phi_29648_;
                                                phi_16845_ = type_59(_e12564, _e12086);
                                                phi_16846_ = type_59();
                                                phi_16847_ = true;
                                            } else {
                                                let _e12094 = (((_e7704.x * _e7776) + (_e7704.y * _e7777)) + (_e7704.z * _e7779));
                                                let _e12099 = (((_e7704.x * _e7772) + (_e7704.y * _e7782)) + (_e7704.z * _e7783));
                                                let _e12104 = (((_e7704.x * _e7730.x) + (_e7704.y * _e7730.y)) + (_e7704.z * _e7730.z));
                                                let _e12109 = (((_e12072.x * _e7776) + (_e12072.y * _e7777)) + (_e12072.z * _e7779));
                                                let _e12114 = (((_e12072.x * _e7772) + (_e12072.y * _e7782)) + (_e12072.z * _e7783));
                                                if (_e12104 <= 0f) {
                                                    phi_16809_ = type_59();
                                                    phi_16810_ = true;
                                                } else {
                                                    let _e12116 = (_e12080 <= 0f);
                                                    if _e12116 {
                                                        phi_16808_ = type_59();
                                                    } else {
                                                        let _e12117 = (_e12094 + _e12109);
                                                        let _e12118 = (_e12099 + _e12114);
                                                        let _e12119 = (_e12104 + _e12080);
                                                        let _e12126 = (1f / sqrt((((_e12117 * _e12117) + (_e12118 * _e12118)) + (_e12119 * _e12119))));
                                                        let _e12129 = (_e12119 * _e12126);
                                                        if (_e12129 <= 0f) {
                                                            phi_28961_ = 0f;
                                                        } else {
                                                            let _e12131 = (_e8603 * _e8603);
                                                            let _e12135 = ((((_e12131 - 1f) * _e12129) * _e12129) + 1f);
                                                            phi_28961_ = (_e12131 / ((3.1415927f * _e12135) * _e12135));
                                                        }
                                                        let _e12140 = phi_28961_;
                                                        let _e12141 = (_e12104 * _e12104);
                                                        if (_e12141 >= 1f) {
                                                            phi_28989_ = 0f;
                                                        } else {
                                                            phi_28989_ = ((sqrt((1f + (((_e8603 * _e8603) * ((_e12094 * _e12094) + (_e12099 * _e12099))) / _e12141))) - 1f) * 0.5f);
                                                        }
                                                        let _e12154 = phi_28989_;
                                                        let _e12155 = (1f + _e12154);
                                                        let _e12156 = (_e12080 * _e12080);
                                                        if (_e12156 >= 1f) {
                                                            phi_29017_ = 0f;
                                                        } else {
                                                            phi_29017_ = ((sqrt((1f + (((_e8603 * _e8603) * ((_e12109 * _e12109) + (_e12114 * _e12114))) / _e12156))) - 1f) * 0.5f);
                                                        }
                                                        let _e12169 = phi_29017_;
                                                        let _e12176 = (((_e12094 * (_e12117 * _e12126)) + (_e12099 * (_e12118 * _e12126))) + (_e12104 * _e12129));
                                                        if (_e8601 == 0u) {
                                                            let _e12471 = (1f - _e12176);
                                                            if (_e12471 != _e12471) {
                                                                phi_29520_ = true;
                                                            } else {
                                                                phi_29520_ = (0f >= _e12471);
                                                            }
                                                            let _e12475 = phi_29520_;
                                                            let _e12477 = pow(select(_e12471, 0f, _e12475), 5f);
                                                            let _e12478 = (1f - _e12477);
                                                            phi_16796_ = vec3<f32>(((_e8599.x * _e12478) + _e12477), ((_e8599.y * _e12478) + _e12477), ((_e8599.z * _e12478) + _e12477));
                                                        } else {
                                                            if (_e8601 == 1u) {
                                                                let _e12444 = (1f / _e8595);
                                                                let _e12448 = ((_e12444 * _e12444) * (1f - (_e12176 * _e12176)));
                                                                if (_e12448 >= 1f) {
                                                                    phi_29481_ = 1f;
                                                                } else {
                                                                    let _e12451 = sqrt((1f - _e12448));
                                                                    if (_e12176 != _e12176) {
                                                                        phi_29492_ = true;
                                                                    } else {
                                                                        phi_29492_ = (_e12451 <= _e12176);
                                                                    }
                                                                    let _e12455 = phi_29492_;
                                                                    let _e12460 = pow(((1f - _e8595) / (1f + _e8595)), 2f);
                                                                    phi_29481_ = (_e12460 + ((1f - _e12460) * pow((1f - select(_e12176, _e12451, _e12455)), 5f)));
                                                                }
                                                                let _e12467 = phi_29481_;
                                                                phi_16795_ = vec3(_e12467);
                                                            } else {
                                                                let _e12180 = select(_e12176, 0f, (_e12176 < 0f));
                                                                let _e12182 = select(_e12180, 1f, (_e12180 > 1f));
                                                                let _e12184 = (1f - (_e12182 * _e12182));
                                                                let _e12197 = ((_e8599.x * _e8599.x) - (_e8597.x * _e8597.x));
                                                                let _e12198 = ((_e8599.y * _e8599.y) - (_e8597.y * _e8597.y));
                                                                let _e12199 = ((_e8599.z * _e8599.z) - (_e8597.z * _e8597.z));
                                                                let _e12201 = (_e8599.x * (_e8597.x + _e8597.x));
                                                                let _e12203 = (_e8599.y * (_e8597.y + _e8597.y));
                                                                let _e12205 = (_e8599.z * (_e8597.z + _e8597.z));
                                                                let _e12212 = ((_e12197 * _e12197) + (_e12201 * _e12201));
                                                                let _e12213 = ((_e12198 * _e12198) + (_e12203 * _e12203));
                                                                let _e12214 = ((_e12199 * _e12199) + (_e12205 * _e12205));
                                                                let _e12219 = ((_e12184 * _e12197) / _e12212);
                                                                let _e12220 = ((_e12184 * _e12198) / _e12213);
                                                                let _e12221 = ((_e12184 * _e12199) / _e12214);
                                                                let _e12222 = (1f - _e12219);
                                                                let _e12223 = (1f - _e12220);
                                                                let _e12224 = (1f - _e12221);
                                                                let _e12226 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e12219, _e12220, _e12221));
                                                                let _e12227 = (_e12184 * _e12201);
                                                                let _e12228 = (_e12184 * _e12203);
                                                                let _e12229 = (_e12184 * _e12205);
                                                                let _e12234 = (vec3<f32>(_e12227, _e12228, _e12229) / vec3<f32>(_e12212, _e12213, _e12214));
                                                                let _e12247 = ((_e12226.x * _e12226.x) + (_e12234.x * _e12234.x));
                                                                let _e12248 = ((_e12226.y * _e12226.y) + (_e12234.y * _e12234.y));
                                                                let _e12249 = ((_e12226.z * _e12226.z) + (_e12234.z * _e12234.z));
                                                                if (_e12247 != _e12247) {
                                                                    phi_29151_ = true;
                                                                } else {
                                                                    phi_29151_ = (0f >= _e12247);
                                                                }
                                                                let _e12253 = phi_29151_;
                                                                let _e12255 = sqrt(select(_e12247, 0f, _e12253));
                                                                if (_e12248 != _e12248) {
                                                                    phi_29166_ = true;
                                                                } else {
                                                                    phi_29166_ = (0f >= _e12248);
                                                                }
                                                                let _e12259 = phi_29166_;
                                                                let _e12261 = sqrt(select(_e12248, 0f, _e12259));
                                                                if (_e12249 != _e12249) {
                                                                    phi_29181_ = true;
                                                                } else {
                                                                    phi_29181_ = (0f >= _e12249);
                                                                }
                                                                let _e12265 = phi_29181_;
                                                                let _e12267 = sqrt(select(_e12249, 0f, _e12265));
                                                                let _e12271 = ((_e12255 + _e12222) * 0.5f);
                                                                let _e12272 = ((_e12261 + _e12223) * 0.5f);
                                                                let _e12273 = ((_e12267 + _e12224) * 0.5f);
                                                                if (_e12271 != _e12271) {
                                                                    phi_29208_ = true;
                                                                } else {
                                                                    phi_29208_ = (0f >= _e12271);
                                                                }
                                                                let _e12277 = phi_29208_;
                                                                let _e12279 = sqrt(select(_e12271, 0f, _e12277));
                                                                if (_e12272 != _e12272) {
                                                                    phi_29223_ = true;
                                                                } else {
                                                                    phi_29223_ = (0f >= _e12272);
                                                                }
                                                                let _e12283 = phi_29223_;
                                                                let _e12285 = sqrt(select(_e12272, 0f, _e12283));
                                                                if (_e12273 != _e12273) {
                                                                    phi_29238_ = true;
                                                                } else {
                                                                    phi_29238_ = (0f >= _e12273);
                                                                }
                                                                let _e12289 = phi_29238_;
                                                                let _e12291 = sqrt(select(_e12273, 0f, _e12289));
                                                                let _e12292 = vec3<f32>(_e12279, _e12285, _e12291);
                                                                let _e12296 = ((_e12255 - _e12222) * 0.5f);
                                                                let _e12297 = ((_e12261 - _e12223) * 0.5f);
                                                                let _e12298 = ((_e12267 - _e12224) * 0.5f);
                                                                if (_e12296 != _e12296) {
                                                                    phi_29265_ = true;
                                                                } else {
                                                                    phi_29265_ = (0f >= _e12296);
                                                                }
                                                                let _e12302 = phi_29265_;
                                                                let _e12304 = sqrt(select(_e12296, 0f, _e12302));
                                                                if (_e12297 != _e12297) {
                                                                    phi_29280_ = true;
                                                                } else {
                                                                    phi_29280_ = (0f >= _e12297);
                                                                }
                                                                let _e12308 = phi_29280_;
                                                                let _e12310 = sqrt(select(_e12297, 0f, _e12308));
                                                                if (_e12298 != _e12298) {
                                                                    phi_29295_ = true;
                                                                } else {
                                                                    phi_29295_ = (0f >= _e12298);
                                                                }
                                                                let _e12314 = phi_29295_;
                                                                let _e12316 = sqrt(select(_e12298, 0f, _e12314));
                                                                if ((_e12227 / _e12212) < 0f) {
                                                                    phi_16672_ = -(_e12304);
                                                                } else {
                                                                    phi_16672_ = _e12304;
                                                                }
                                                                let _e12320 = phi_16672_;
                                                                if ((_e12228 / _e12213) < 0f) {
                                                                    phi_16680_ = -(_e12310);
                                                                } else {
                                                                    phi_16680_ = _e12310;
                                                                }
                                                                let _e12324 = phi_16680_;
                                                                if ((_e12229 / _e12214) < 0f) {
                                                                    phi_16688_ = -(_e12316);
                                                                } else {
                                                                    phi_16688_ = _e12316;
                                                                }
                                                                let _e12328 = phi_16688_;
                                                                let _e12332 = (_e8597.x * _e12182);
                                                                let _e12333 = (_e8597.y * _e12182);
                                                                let _e12334 = (_e8597.z * _e12182);
                                                                let _e12344 = vec3<f32>(((_e8599.x * _e12279) - (_e8597.x * _e12320)), ((_e8599.y * _e12285) - (_e8597.y * _e12324)), ((_e8599.z * _e12291) - (_e8597.z * _e12328)));
                                                                let _e12351 = ((_e8599.x * _e12320) + (_e8597.x * _e12279));
                                                                let _e12352 = ((_e8599.y * _e12324) + (_e8597.y * _e12285));
                                                                let _e12353 = ((_e8599.z * _e12328) + (_e8597.z * _e12291));
                                                                let _e12355 = vec3<f32>((_e8599.x * _e12182), (_e8599.y * _e12182), (_e8599.z * _e12182));
                                                                let _e12356 = (_e12355 - _e12292);
                                                                let _e12357 = (_e12332 - _e12320);
                                                                let _e12358 = (_e12333 - _e12324);
                                                                let _e12359 = (_e12334 - _e12328);
                                                                let _e12372 = (_e12355 + _e12292);
                                                                let _e12373 = (_e12332 + _e12320);
                                                                let _e12374 = (_e12333 + _e12324);
                                                                let _e12375 = (_e12334 + _e12328);
                                                                let _e12385 = ((_e12372.x * _e12372.x) + (_e12373 * _e12373));
                                                                let _e12386 = ((_e12372.y * _e12372.y) + (_e12374 * _e12374));
                                                                let _e12387 = ((_e12372.z * _e12372.z) + (_e12375 * _e12375));
                                                                let _e12397 = vec3(_e12182);
                                                                let _e12398 = (_e12397 - _e12344);
                                                                let _e12399 = -(vec3<f32>(_e12351, _e12352, _e12353));
                                                                let _e12415 = (_e12397 + _e12344);
                                                                let _e12425 = ((_e12415.x * _e12415.x) + (_e12351 * _e12351));
                                                                let _e12426 = ((_e12415.y * _e12415.y) + (_e12352 * _e12352));
                                                                let _e12427 = ((_e12415.z * _e12415.z) + (_e12353 * _e12353));
                                                                phi_16795_ = vec3<f32>((((((_e12356.x * _e12356.x) + (_e12357 * _e12357)) / select(0.000000000000000000000000000001f, _e12385, (_e12385 > 0.000000000000000000000000000001f))) + (((_e12398.x * _e12398.x) + (_e12399.x * _e12399.x)) / select(0.000000000000000000000000000001f, _e12425, (_e12425 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e12356.y * _e12356.y) + (_e12358 * _e12358)) / select(0.000000000000000000000000000001f, _e12386, (_e12386 > 0.000000000000000000000000000001f))) + (((_e12398.y * _e12398.y) + (_e12399.y * _e12399.y)) / select(0.000000000000000000000000000001f, _e12426, (_e12426 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e12356.z * _e12356.z) + (_e12359 * _e12359)) / select(0.000000000000000000000000000001f, _e12387, (_e12387 > 0.000000000000000000000000000001f))) + (((_e12398.z * _e12398.z) + (_e12399.z * _e12399.z)) / select(0.000000000000000000000000000001f, _e12427, (_e12427 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                            }
                                                            let _e12470 = phi_16795_;
                                                            phi_16796_ = _e12470;
                                                        }
                                                        let _e12490 = phi_16796_;
                                                        let _e12492 = (4f * _e12104);
                                                        phi_16808_ = type_59((_e12490 * (((1f / (_e12155 + _e12169)) * _e12140) / _e12492)), ((_e12140 / _e12155) / _e12492));
                                                    }
                                                    let _e12499 = phi_16808_;
                                                    phi_16809_ = _e12499;
                                                    phi_16810_ = _e12116;
                                                }
                                                let _e12501 = phi_16809_;
                                                let _e12503 = phi_16810_;
                                                if _e12503 {
                                                    phi_16815_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                                } else {
                                                    phi_16815_ = _e12501;
                                                }
                                                let _e12505 = phi_16815_;
                                                let _e12508 = (_e8605 == 1u);
                                                if _e12508 {
                                                    phi_16843_ = _e12505;
                                                    phi_16844_ = type_59();
                                                } else {
                                                    if (_e12104 != _e12104) {
                                                        phi_29557_ = true;
                                                    } else {
                                                        phi_29557_ = (0f >= _e12104);
                                                    }
                                                    let _e12512 = phi_29557_;
                                                    let _e12513 = select(_e12104, 0f, _e12512);
                                                    let _e12514 = (1f / _e8595);
                                                    let _e12518 = ((_e12514 * _e12514) * (1f - (_e12513 * _e12513)));
                                                    if (_e12518 >= 1f) {
                                                        phi_29576_ = 1f;
                                                    } else {
                                                        let _e12521 = sqrt((1f - _e12518));
                                                        if (_e12513 != _e12513) {
                                                            phi_29587_ = true;
                                                        } else {
                                                            phi_29587_ = (_e12521 <= _e12513);
                                                        }
                                                        let _e12525 = phi_29587_;
                                                        let _e12530 = pow(((1f - _e8595) / (1f + _e8595)), 2f);
                                                        phi_29576_ = (_e12530 + ((1f - _e12530) * pow((1f - select(_e12513, _e12521, _e12525)), 5f)));
                                                    }
                                                    let _e12537 = phi_29576_;
                                                    if (_e12080 <= 0f) {
                                                        phi_29626_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        phi_29626_ = (_e8593 * (_e12080 * 0.31830987f));
                                                    }
                                                    let _e12542 = phi_29626_;
                                                    let _e12543 = (1f - _e12537);
                                                    phi_16843_ = type_59();
                                                    phi_16844_ = type_59((_e12505.member + vec3<f32>((_e12542.x * _e12543), (_e12542.y * _e12543), (_e12542.z * _e12543))), ((_e12537 * _e12505.member_1) + (_e12543 * _e12086)));
                                                }
                                                let _e12557 = phi_16843_;
                                                let _e12559 = phi_16844_;
                                                phi_16845_ = _e12557;
                                                phi_16846_ = _e12559;
                                                phi_16847_ = _e12508;
                                            }
                                            let _e12567 = phi_16845_;
                                            let _e12569 = phi_16846_;
                                            let _e12571 = phi_16847_;
                                            if _e12571 {
                                                phi_16851_ = _e12567;
                                            } else {
                                                phi_16851_ = _e12569;
                                            }
                                            let _e12573 = phi_16851_;
                                            if ((((_e12072.x * _e12072.x) + (_e12072.y * _e12072.y)) + (_e12072.z * _e12072.z)) == 0f) {
                                                phi_35323_ = _e11829;
                                                phi_16989_ = vec3<f32>();
                                                phi_16990_ = u32();
                                                phi_16991_ = vec3<f32>();
                                                phi_16992_ = vec3<f32>();
                                                phi_16993_ = true;
                                                phi_16994_ = false;
                                            } else {
                                                let _e12582 = (_e12573.member_1 <= 0f);
                                                if _e12582 {
                                                    phi_35324_ = _e11829;
                                                    phi_16984_ = vec3<f32>();
                                                    phi_16985_ = u32();
                                                    phi_16986_ = vec3<f32>();
                                                    phi_16987_ = vec3<f32>();
                                                    phi_16988_ = false;
                                                } else {
                                                    let _e12584 = (_e12573.member.x / _e12573.member_1);
                                                    let _e12586 = (_e12573.member.y / _e12573.member_1);
                                                    let _e12588 = (_e12573.member.z / _e12573.member_1);
                                                    let _e12597 = global_1.member.member_4[1u];
                                                    if (sqrt((((_e12584 * _e12584) + (_e12586 * _e12586)) + (_e12588 * _e12588))) <= _e12597) {
                                                        phi_35326_ = _e11829;
                                                        phi_16938_ = _e7617;
                                                        phi_16939_ = false;
                                                    } else {
                                                        let _e12601 = global_1.member.member_7[1u];
                                                        if (_e12601 == 1u) {
                                                            let _e12606 = global_1.member.member_4[3u];
                                                            if (f32(_e393) >= _e12606) {
                                                                let _e12609 = select(_e12586, _e12584, (_e12584 > _e12586));
                                                                let _e12611 = select(_e12588, _e12609, (_e12609 > _e12588));
                                                                if (_e12611 != _e12611) {
                                                                    phi_29673_ = true;
                                                                } else {
                                                                    phi_29673_ = (1f <= _e12611);
                                                                }
                                                                let _e12615 = phi_29673_;
                                                                let _e12616 = select(_e12611, 1f, _e12615);
                                                                let _e12618 = ((_e11829 * 747796405u) + 2891336453u);
                                                                let _e12626 = (((_e12618 >> bitcast<u32>((((_e12618 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e12618) * 277803737u);
                                                                let _e12629 = ((_e12626 >> bitcast<u32>(22i)) ^ _e12626);
                                                                let _e12634 = ((f32((_e12629 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e12616);
                                                                if _e12634 {
                                                                    phi_16927_ = vec3<f32>();
                                                                } else {
                                                                    phi_16927_ = vec3<f32>((_e7617.x / _e12616), (_e7617.y / _e12616), (_e7617.z / _e12616));
                                                                }
                                                                let _e12643 = phi_16927_;
                                                                phi_35329_ = _e12629;
                                                                phi_16929_ = _e12643;
                                                                phi_16930_ = select(true, false, _e12634);
                                                            } else {
                                                                phi_35329_ = _e11829;
                                                                phi_16929_ = _e7617;
                                                                phi_16930_ = true;
                                                            }
                                                            let _e12646 = phi_35329_;
                                                            let _e12648 = phi_16929_;
                                                            let _e12650 = phi_16930_;
                                                            phi_35328_ = _e12646;
                                                            phi_16931_ = _e12648;
                                                            phi_16932_ = _e12650;
                                                        } else {
                                                            phi_35328_ = _e11829;
                                                            phi_16931_ = _e7617;
                                                            phi_16932_ = true;
                                                        }
                                                        let _e12652 = phi_35328_;
                                                        let _e12654 = phi_16931_;
                                                        let _e12656 = phi_16932_;
                                                        phi_35326_ = _e12652;
                                                        phi_16938_ = select(_e7617, _e12654, vec3(_e12656));
                                                        phi_16939_ = select(false, true, _e12656);
                                                    }
                                                    let _e12661 = phi_35326_;
                                                    let _e12663 = phi_16938_;
                                                    let _e12665 = phi_16939_;
                                                    if _e12665 {
                                                        if ((((_e12072.x * _e7717.x) + (_e12072.y * _e7717.y)) + (_e12072.z * _e7717.z)) < 0f) {
                                                            phi_16968_ = -(_e7717);
                                                        } else {
                                                            phi_16968_ = _e7717;
                                                        }
                                                        let _e12681 = phi_16968_;
                                                        phi_16980_ = vec3<f32>((_e12663.x * _e12584), (_e12663.y * _e12586), (_e12663.z * _e12588));
                                                        phi_16981_ = (_e393 + 1u);
                                                        phi_16982_ = vec3<f32>((_e7626 + (_e12681.x * _e7748)), (_e7628 + (_e12681.y * _e7748)), (_e7630 + (_e12681.z * _e7748)));
                                                    } else {
                                                        phi_16980_ = vec3<f32>();
                                                        phi_16981_ = u32();
                                                        phi_16982_ = vec3<f32>();
                                                    }
                                                    let _e12694 = phi_16980_;
                                                    let _e12696 = phi_16981_;
                                                    let _e12698 = phi_16982_;
                                                    phi_35324_ = _e12661;
                                                    phi_16984_ = _e12694;
                                                    phi_16985_ = _e12696;
                                                    phi_16986_ = _e12698;
                                                    phi_16987_ = _e12663;
                                                    phi_16988_ = select(true, false, _e12665);
                                                }
                                                let _e12701 = phi_35324_;
                                                let _e12703 = phi_16984_;
                                                let _e12705 = phi_16985_;
                                                let _e12707 = phi_16986_;
                                                let _e12709 = phi_16987_;
                                                let _e12711 = phi_16988_;
                                                phi_35323_ = _e12701;
                                                phi_16989_ = _e12703;
                                                phi_16990_ = _e12705;
                                                phi_16991_ = _e12707;
                                                phi_16992_ = _e12709;
                                                phi_16993_ = _e12582;
                                                phi_16994_ = _e12711;
                                            }
                                            let _e12713 = phi_35323_;
                                            let _e12715 = phi_16989_;
                                            let _e12717 = phi_16990_;
                                            let _e12719 = phi_16991_;
                                            let _e12721 = phi_16992_;
                                            let _e12723 = phi_16993_;
                                            let _e12725 = phi_16994_;
                                            phi_35321_ = _e12713;
                                            phi_17003_ = select(_e12721, _e7617, vec3(_e12723));
                                            phi_17004_ = _e12715;
                                            phi_17005_ = _e12072;
                                            phi_17006_ = _e12717;
                                            phi_17007_ = _e12573.member_1;
                                            phi_17008_ = _e12719;
                                            phi_17009_ = select(_e12725, true, _e12723);
                                        }
                                        let _e12730 = phi_35321_;
                                        let _e12732 = phi_17003_;
                                        let _e12734 = phi_17004_;
                                        let _e12736 = phi_17005_;
                                        let _e12738 = phi_17006_;
                                        let _e12740 = phi_17007_;
                                        let _e12742 = phi_17008_;
                                        let _e12744 = phi_17009_;
                                        phi_35319_ = _e12730;
                                        phi_35221_ = _e11782;
                                        let _e15058 = local_119;
                                        phi_17014_ = _e15058;
                                        let _e15061 = local_120;
                                        phi_17015_ = _e15061;
                                        phi_17016_ = _e12732;
                                        let _e15065 = local_121;
                                        phi_17017_ = _e15065;
                                        let _e15068 = local_122;
                                        phi_17020_ = _e15068;
                                        let _e15071 = local_123;
                                        phi_17021_ = _e15071;
                                        let _e15074 = local_124;
                                        phi_17026_ = _e15074;
                                        let _e15077 = local_125;
                                        phi_17027_ = _e15077;
                                        phi_17028_ = _e12734;
                                        let _e15081 = local_126;
                                        phi_17029_ = _e15081;
                                        phi_17030_ = _e12736;
                                        let _e15085 = local_127;
                                        phi_17031_ = _e15085;
                                        let _e15088 = local_128;
                                        phi_17032_ = _e15088;
                                        phi_17033_ = _e12738;
                                        phi_17034_ = _e12740;
                                        phi_17035_ = _e12742;
                                        phi_17036_ = _e12744;
                                        phi_17037_ = select(true, false, _e12744);
                                    } else {
                                        phi_35319_ = _e8607;
                                        phi_35221_ = _e7613;
                                        phi_17014_ = _e1157;
                                        phi_17015_ = _e7615;
                                        phi_17016_ = _e8609;
                                        phi_17017_ = vec3<f32>(0f, 0f, 0f);
                                        phi_17020_ = _e389;
                                        phi_17021_ = _e391;
                                        phi_17026_ = u32();
                                        phi_17027_ = vec3<f32>();
                                        phi_17028_ = vec3<f32>();
                                        phi_17029_ = vec3<f32>();
                                        phi_17030_ = vec3<f32>();
                                        phi_17031_ = vec3<f32>();
                                        phi_17032_ = vec3<f32>();
                                        phi_17033_ = u32();
                                        phi_17034_ = f32();
                                        phi_17035_ = vec3<f32>();
                                        phi_17036_ = _e8619;
                                        phi_17037_ = false;
                                    }
                                    let _e12747 = phi_35319_;
                                    let _e12749 = phi_35221_;
                                    let _e12751 = phi_17014_;
                                    let _e12753 = phi_17015_;
                                    let _e12755 = phi_17016_;
                                    let _e12757 = phi_17017_;
                                    let _e12759 = phi_17020_;
                                    let _e12761 = phi_17021_;
                                    let _e12763 = phi_17026_;
                                    let _e12765 = phi_17027_;
                                    let _e12767 = phi_17028_;
                                    let _e12769 = phi_17029_;
                                    let _e12771 = phi_17030_;
                                    let _e12773 = phi_17031_;
                                    let _e12775 = phi_17032_;
                                    let _e12777 = phi_17033_;
                                    let _e12779 = phi_17034_;
                                    let _e12781 = phi_17035_;
                                    let _e12783 = phi_17036_;
                                    let _e12785 = phi_17037_;
                                    let _e12786 = vec3(_e8617);
                                    let _e12796 = vec3(_e12783);
                                    phi_35317_ = _e12747;
                                    phi_35219_ = _e12749;
                                    phi_17059_ = select(_e12763, _e12751, _e12783);
                                    phi_17060_ = select(_e12765, _e12753, _e12796);
                                    phi_17061_ = select(_e12767, _e12755, _e12796);
                                    phi_17062_ = select(_e12769, _e12757, _e12796);
                                    phi_17063_ = select(_e12771, select(_e8611, _e385, _e12786), _e12796);
                                    phi_17064_ = select(_e387, select(_e387, true, _e8617), _e12783);
                                    phi_17065_ = select(_e12773, _e12759, _e12796);
                                    phi_17066_ = select(_e12775, _e12761, _e12796);
                                    phi_17067_ = select(_e12777, select(_e8613, _e393, _e8617), _e12783);
                                    phi_17068_ = select(true, select(_e395, true, _e8617), _e12783);
                                    phi_17069_ = select(_e12779, _e397, _e12783);
                                    phi_17070_ = select(false, select(true, _e399, _e8617), _e12783);
                                    phi_17071_ = select(_e12781, select(_e8615, _e401, _e12786), _e12796);
                                    phi_17072_ = select(_e12785, true, _e12783);
                                    phi_17073_ = select(select(_e8621, false, _e8617), false, _e12783);
                                    phi_17074_ = select(select(_e8623, false, _e8617), false, _e12783);
                                }
                                let _e12813 = phi_35317_;
                                let _e12815 = phi_35219_;
                                let _e12817 = phi_17059_;
                                let _e12819 = phi_17060_;
                                let _e12821 = phi_17061_;
                                let _e12823 = phi_17062_;
                                let _e12825 = phi_17063_;
                                let _e12827 = phi_17064_;
                                let _e12829 = phi_17065_;
                                let _e12831 = phi_17066_;
                                let _e12833 = phi_17067_;
                                let _e12835 = phi_17068_;
                                let _e12837 = phi_17069_;
                                let _e12839 = phi_17070_;
                                let _e12841 = phi_17071_;
                                let _e12843 = phi_17072_;
                                let _e12845 = phi_17073_;
                                let _e12847 = phi_17074_;
                                phi_35316_ = _e12813;
                                phi_35218_ = _e12815;
                                phi_17075_ = _e7615;
                                phi_17076_ = _e7617;
                                phi_17077_ = _e7626;
                                phi_17078_ = _e7628;
                                phi_17079_ = _e7630;
                                phi_17080_ = (_e401 + vec3<f32>(_e7620, _e7622, _e7624));
                                phi_17081_ = _e7635;
                                let _e15123 = local_129;
                                phi_17082_ = _e15123;
                                phi_17083_ = _e12817;
                                phi_17084_ = _e12819;
                                phi_17085_ = _e12821;
                                phi_17086_ = _e12823;
                                phi_17087_ = _e12825;
                                phi_17088_ = _e12827;
                                phi_17089_ = _e12829;
                                phi_17090_ = _e12831;
                                phi_17091_ = _e12833;
                                phi_17092_ = _e12835;
                                phi_17093_ = _e12837;
                                phi_17094_ = _e12839;
                                phi_17095_ = _e12841;
                                phi_17098_ = _e12843;
                                phi_17099_ = _e7700;
                                phi_17100_ = _e12845;
                                phi_17101_ = _e12847;
                            }
                            let _e12849 = phi_35316_;
                            let _e12851 = phi_35218_;
                            let _e12853 = phi_17075_;
                            let _e12855 = phi_17076_;
                            let _e12857 = phi_17077_;
                            let _e12859 = phi_17078_;
                            let _e12861 = phi_17079_;
                            let _e12863 = phi_17080_;
                            let _e12865 = phi_17081_;
                            let _e12867 = phi_17082_;
                            let _e12869 = phi_17083_;
                            let _e12871 = phi_17084_;
                            let _e12873 = phi_17085_;
                            let _e12875 = phi_17086_;
                            let _e12877 = phi_17087_;
                            let _e12879 = phi_17088_;
                            let _e12881 = phi_17089_;
                            let _e12883 = phi_17090_;
                            let _e12885 = phi_17091_;
                            let _e12887 = phi_17092_;
                            let _e12889 = phi_17093_;
                            let _e12891 = phi_17094_;
                            let _e12893 = phi_17095_;
                            let _e12895 = phi_17098_;
                            let _e12897 = phi_17099_;
                            let _e12899 = phi_17100_;
                            let _e12901 = phi_17101_;
                            phi_35315_ = _e12849;
                            phi_35217_ = _e12851;
                            phi_17102_ = _e12853;
                            phi_17103_ = _e12855;
                            phi_17104_ = _e12857;
                            phi_17105_ = _e12859;
                            phi_17106_ = _e12861;
                            phi_17107_ = _e12863;
                            phi_17108_ = _e12865;
                            phi_17109_ = _e12867;
                            phi_490_ = _e12869;
                            phi_493_ = _e12871;
                            phi_495_ = _e12873;
                            phi_497_ = _e12875;
                            phi_499_ = _e12877;
                            phi_501_ = _e12879;
                            phi_503_ = _e12881;
                            phi_505_ = _e12883;
                            phi_507_ = _e12885;
                            phi_509_ = _e12887;
                            phi_511_ = _e12889;
                            phi_513_ = _e12891;
                            phi_515_ = _e12893;
                            phi_17112_ = _e12895;
                            phi_17114_ = _e12897;
                            phi_17115_ = _e12899;
                            phi_17116_ = _e12901;
                        }
                        let _e12903 = phi_35315_;
                        let _e12905 = phi_35217_;
                        let _e12907 = phi_17102_;
                        let _e12909 = phi_17103_;
                        let _e12911 = phi_17104_;
                        let _e12913 = phi_17105_;
                        let _e12915 = phi_17106_;
                        let _e12917 = phi_17107_;
                        let _e12919 = phi_17108_;
                        let _e12921 = phi_17109_;
                        let _e12923 = phi_490_;
                        let _e12925 = phi_493_;
                        let _e12927 = phi_495_;
                        let _e12929 = phi_497_;
                        let _e12931 = phi_499_;
                        let _e12933 = phi_501_;
                        let _e12935 = phi_503_;
                        let _e12937 = phi_505_;
                        let _e12939 = phi_507_;
                        let _e12941 = phi_509_;
                        let _e12943 = phi_511_;
                        let _e12945 = phi_513_;
                        let _e12947 = phi_515_;
                        let _e12949 = phi_17112_;
                        let _e12951 = phi_17114_;
                        let _e12953 = phi_17115_;
                        let _e12955 = phi_17116_;
                        phi_30823_ = _e12903;
                        phi_30223_ = _e12905;
                        phi_489_ = _e12923;
                        phi_492_ = _e12925;
                        phi_494_ = _e12927;
                        phi_496_ = _e12929;
                        phi_498_ = _e12931;
                        phi_500_ = _e12933;
                        phi_502_ = _e12935;
                        phi_504_ = _e12937;
                        phi_506_ = _e12939;
                        phi_508_ = _e12941;
                        phi_510_ = _e12943;
                        phi_512_ = _e12945;
                        phi_514_ = _e12947;
                        phi_35216_ = _e12905;
                        phi_19906_ = _e1156;
                        phi_19905_ = _e1157;
                        let _e15173 = local_130;
                        phi_19904_ = _e15173;
                        phi_19903_ = _e12955;
                        phi_19902_ = _e12953;
                        phi_19901_ = _e12951;
                        phi_19900_ = select(_e1158, false, _e387);
                        phi_19899_ = _e12907;
                        phi_19897_ = _e12921;
                        phi_19895_ = _e12919;
                        phi_19893_ = _e12917;
                        phi_19892_ = _e12915;
                        phi_19891_ = _e12913;
                        phi_19890_ = _e12911;
                        phi_19888_ = _e12909;
                        if !(_e12949) {
                            break;
                        }
                    }
                    let _e12959 = phi_35216_;
                    let _e12961 = phi_19906_;
                    let _e12963 = phi_19905_;
                    let _e12965 = phi_19904_;
                    let _e12967 = phi_19903_;
                    let _e12969 = phi_19902_;
                    let _e12971 = phi_19901_;
                    let _e12973 = phi_19900_;
                    let _e12975 = phi_19899_;
                    let _e12977 = phi_19897_;
                    let _e12979 = phi_19895_;
                    let _e12981 = phi_19893_;
                    let _e12983 = phi_19892_;
                    let _e12985 = phi_19891_;
                    let _e12987 = phi_19890_;
                    let _e12989 = phi_19888_;
                    if _e12959 {
                        break;
                    }
                    let _e12991 = local_73;
                    let _e12992 = select(_e12973, false, _e12991);
                    let _e12994 = local_74;
                    let _e12997 = local_75;
                    let _e13000 = local_76;
                    if _e12992 {
                        if ((_e368 & 4u) == 0u) {
                            phi_18007_ = _e12965;
                        } else {
                            let _e13005 = local_77;
                            if _e13005 {
                                phi_17177_ = 1f;
                            } else {
                                let _e13007 = global_1.member.member_5;
                                let _e13012 = local_78;
                                let _e13014 = (_e13012.x - _e13007.x);
                                let _e13016 = local_79;
                                let _e13018 = (_e13016.y - _e13007.y);
                                let _e13020 = local_80;
                                let _e13022 = (_e13020.z - _e13007.z);
                                let _e13029 = (sqrt((((_e13014 * _e13014) + (_e13018 * _e13018)) + (_e13022 * _e13022))) * 0.5f);
                                if (_e13029 != _e13029) {
                                    phi_29709_ = true;
                                } else {
                                    phi_29709_ = (1f <= _e13029);
                                }
                                let _e13033 = phi_29709_;
                                let _e13039 = global_1.member.member_6[3u];
                                if ((2f * asin(select(_e13029, 1f, _e13033))) >= _e13039) {
                                    phi_17175_ = 0f;
                                } else {
                                    let _e13042 = sin((_e13039 * 0.5f));
                                    phi_17175_ = (1f / ((12.566371f * _e13042) * _e13042));
                                }
                                let _e13047 = phi_17175_;
                                let _e13049 = local_81;
                                if (_e13049 >= 300000000000000000000000000000000000000f) {
                                    phi_29730_ = 1f;
                                } else {
                                    let _e13052 = local_82;
                                    let _e13054 = local_83;
                                    let _e13055 = (_e13052 * _e13054);
                                    let _e13057 = (_e13055 + (_e13047 * _e13047));
                                    if (_e13057 == 0f) {
                                        phi_29727_ = 0f;
                                    } else {
                                        phi_29727_ = (_e13055 / _e13057);
                                    }
                                    let _e13061 = phi_29727_;
                                    phi_29730_ = _e13061;
                                }
                                let _e13063 = phi_29730_;
                                phi_17177_ = _e13063;
                            }
                            let _e13065 = phi_17177_;
                            let _e13067 = global_1.member.member_5;
                            let _e13072 = local_84;
                            let _e13074 = (_e13072.x - _e13067.x);
                            let _e13076 = local_85;
                            let _e13078 = (_e13076.y - _e13067.y);
                            let _e13080 = local_86;
                            let _e13082 = (_e13080.z - _e13067.z);
                            let _e13089 = (sqrt((((_e13074 * _e13074) + (_e13078 * _e13078)) + (_e13082 * _e13082))) * 0.5f);
                            if (_e13089 != _e13089) {
                                phi_29741_ = true;
                            } else {
                                phi_29741_ = (1f <= _e13089);
                            }
                            let _e13093 = phi_29741_;
                            let _e13096 = (2f * asin(select(_e13089, 1f, _e13093)));
                            let _e13100 = global_1.member.member_6[3u];
                            if (_e13096 >= _e13100) {
                                phi_17985_ = vec3<f32>(0f, 0f, 0f);
                            } else {
                                let _e13104 = (1f - pow((_e13096 / _e13100), 2f));
                                if (_e13104 != _e13104) {
                                    phi_29756_ = true;
                                } else {
                                    phi_29756_ = (0f >= _e13104);
                                }
                                let _e13108 = phi_29756_;
                                let _e13111 = global_1.member.member_6;
                                let _e13117 = global_1.member.member_8[2u];
                                if (_e13117 == 4294967295u) {
                                    phi_17958_ = vec3<f32>(1f, 1f, 1f);
                                } else {
                                    let _e13120 = global_6.member[_e13117];
                                    let _e13123 = global_6.member[(_e13117 + 1u)];
                                    let _e13126 = global_6.member[(_e13117 + 2u)];
                                    let _e13129 = global_6.member[(_e13117 + 3u)];
                                    let _e13132 = global_6.member[(_e13117 + 4u)];
                                    let _e13135 = global_6.member[(_e13117 + 5u)];
                                    let _e13138 = global_6.member[(_e13117 + 6u)];
                                    let _e13141 = global_6.member[(_e13117 + 7u)];
                                    let _e13144 = global_6.member[(_e13117 + 8u)];
                                    let _e13147 = global_6.member[(_e13117 + 9u)];
                                    let _e13150 = global_6.member[(_e13117 + 13u)];
                                    let _e13153 = global_6.member[(_e13117 + 14u)];
                                    let _e13156 = global_6.member[(_e13117 + 15u)];
                                    let _e13159 = global_6.member[(_e13117 + 16u)];
                                    let _e13162 = global_6.member[(_e13117 + 18u)];
                                    let _e13165 = global_6.member[(_e13117 + 19u)];
                                    let _e13168 = global_6.member[(_e13117 + 20u)];
                                    let _e13171 = global_6.member[(_e13117 + 29u)];
                                    let _e13172 = bitcast<u32>(_e13171);
                                    let _e13174 = local_87;
                                    let _e13178 = local_88;
                                    let _e13183 = local_89;
                                    let _e13194 = (((((_e13174.x * _e13174.x) + (_e13178.y * _e13178.y)) + (_e13183.z * _e13183.z)) - (2f * (((_e13174.x * _e13120) + (_e13178.y * _e13123)) + (_e13183.z * _e13126)))) + _e13138);
                                    if (_e13194 > 0f) {
                                        let _e13203 = ((((_e13174.x - _e13120) * _e13141) + ((_e13178.y - _e13123) * _e13144)) + ((_e13183.z - _e13126) * _e13147));
                                        let _e13205 = ((_e13203 * _e13203) - _e13194);
                                        if (_e13203 >= 0f) {
                                            phi_17658_ = vec3<f32>();
                                            phi_17659_ = true;
                                        } else {
                                            let _e13207 = (_e13205 <= 0f);
                                            if _e13207 {
                                                phi_17657_ = vec3<f32>();
                                            } else {
                                                let _e13210 = (-(_e13203) - sqrt(_e13205));
                                                phi_17657_ = vec3<f32>((_e13174.x + (_e13141 * _e13210)), (_e13178.y + (_e13144 * _e13210)), (_e13183.z + (_e13147 * _e13210)));
                                            }
                                            let _e13219 = phi_17657_;
                                            phi_17658_ = _e13219;
                                            phi_17659_ = _e13207;
                                        }
                                        let _e13221 = phi_17658_;
                                        let _e13223 = phi_17659_;
                                        phi_17664_ = _e13221;
                                        phi_17665_ = select(true, false, _e13223);
                                    } else {
                                        let _e15215 = local_131;
                                        phi_17664_ = _e15215;
                                        phi_17665_ = true;
                                    }
                                    let _e13226 = phi_17664_;
                                    let _e13228 = phi_17665_;
                                    if _e13228 {
                                        let _e13230 = (_e13226.x - _e13120);
                                        let _e13232 = (_e13226.y - _e13123);
                                        let _e13234 = (_e13226.z - _e13126);
                                        let _e13240 = sqrt((((_e13230 * _e13230) + (_e13232 * _e13232)) + (_e13234 * _e13234)));
                                        let _e13241 = (1f / _e13240);
                                        let _e13259 = ((((((_e13226.x * _e13226.x) + (_e13226.y * _e13226.y)) + (_e13226.z * _e13226.z)) - (2f * (((_e13226.x * _e13120) + (_e13226.y * _e13123)) + (_e13226.z * _e13126)))) + _e13135) / (_e13240 + _e13129));
                                        if (_e13259 != _e13259) {
                                            phi_29880_ = true;
                                        } else {
                                            phi_29880_ = (0f >= _e13259);
                                        }
                                        let _e13263 = phi_29880_;
                                        let _e13267 = sqrt((select(_e13259, 0f, _e13263) / (_e13132 - _e13129)));
                                        if (_e13267 != _e13267) {
                                            phi_29895_ = true;
                                        } else {
                                            phi_29895_ = (1f <= _e13267);
                                        }
                                        let _e13271 = phi_29895_;
                                        let _e13273 = (select(_e13267, 1f, _e13271) * 63f);
                                        let _e13278 = ((((_e13230 * _e13241) * _e13141) + ((_e13232 * _e13241) * _e13144)) + ((_e13234 * _e13241) * _e13147));
                                        let _e13280 = select(_e13278, -1f, (_e13278 < -1f));
                                        let _e13284 = ((select(_e13280, 1f, (_e13280 > 1f)) + 1f) * 127.5f);
                                        let _e13289 = select(select(u32(_e13273), 0u, (_e13273 < 0f)), 4294967295u, (_e13273 > 4294967000f));
                                        let _e13291 = select(_e13289, 62u, (62u < _e13289));
                                        let _e13296 = select(select(u32(_e13284), 0u, (_e13284 < 0f)), 4294967295u, (_e13284 > 4294967000f));
                                        let _e13298 = select(_e13296, 254u, (254u < _e13296));
                                        let _e13300 = (_e13273 - f32(_e13291));
                                        let _e13302 = (_e13284 - f32(_e13298));
                                        let _e13306 = (_e13172 + (3u * ((_e13291 * 256u) + _e13298)));
                                        let _e13311 = (_e13172 + (3u * (((_e13291 + 1u) * 256u) + _e13298)));
                                        let _e13313 = global_6.member[_e13306];
                                        let _e13316 = global_6.member[(_e13306 + 1u)];
                                        let _e13319 = global_6.member[(_e13306 + 2u)];
                                        let _e13320 = (1f - _e13302);
                                        let _e13326 = global_6.member[(_e13306 + 3u)];
                                        let _e13329 = global_6.member[(_e13306 + 4u)];
                                        let _e13332 = global_6.member[(_e13306 + 5u)];
                                        let _e13339 = (1f - _e13300);
                                        let _e13344 = global_6.member[_e13311];
                                        let _e13347 = global_6.member[(_e13311 + 1u)];
                                        let _e13350 = global_6.member[(_e13311 + 2u)];
                                        let _e13356 = global_6.member[(_e13311 + 3u)];
                                        let _e13359 = global_6.member[(_e13311 + 4u)];
                                        let _e13362 = global_6.member[(_e13311 + 5u)];
                                        phi_17956_ = vec3<f32>(((((_e13313 * _e13320) + (_e13326 * _e13302)) * _e13339) + (((_e13344 * _e13320) + (_e13356 * _e13302)) * _e13300)), ((((_e13316 * _e13320) + (_e13329 * _e13302)) * _e13339) + (((_e13347 * _e13320) + (_e13359 * _e13302)) * _e13300)), ((((_e13319 * _e13320) + (_e13332 * _e13302)) * _e13339) + (((_e13350 * _e13320) + (_e13362 * _e13302)) * _e13300)));
                                    } else {
                                        phi_17956_ = vec3<f32>(0f, 0f, 0f);
                                    }
                                    let _e13377 = phi_17956_;
                                    let _e13384 = ((_e13159 * 1.1f) * _e13377.y);
                                    phi_17958_ = vec3<f32>(exp(-((((_e13150 * _e13377.x) + _e13384) + (_e13162 * _e13377.z)))), exp(-((((_e13153 * _e13377.x) + _e13384) + (_e13165 * _e13377.z)))), exp(-((((_e13156 * _e13377.x) + _e13384) + (_e13168 * _e13377.z)))));
                                }
                                let _e13403 = phi_17958_;
                                let _e13415 = sin((_e13100 * 0.5f));
                                let _e13418 = (((1f - (0.6f * (1f - sqrt(select(_e13104, 0f, _e13108))))) * 1.25f) / ((12.566371f * _e13415) * _e13415));
                                phi_17985_ = vec3<f32>(((_e13111.x * _e13403.x) * _e13418), ((_e13111.y * _e13403.y) * _e13418), ((_e13111.z * _e13403.z) * _e13418));
                            }
                            let _e13424 = phi_17985_;
                            let _e13426 = local_90;
                            let _e13431 = local_91;
                            let _e13436 = local_92;
                            phi_18007_ = vec3<f32>((_e12965.x + ((_e13426.x * _e13424.x) * _e13065)), (_e12965.y + ((_e13431.y * _e13424.y) * _e13065)), (_e12965.z + ((_e13436.z * _e13424.z) * _e13065)));
                        }
                        let _e13451 = phi_18007_;
                        phi_18008_ = _e13451;
                    } else {
                        phi_18008_ = _e12965;
                    }
                    let _e13453 = phi_18008_;
                    let _e13454 = select(select(_e12971, false, _e12994), false, _e12992);
                    if _e13454 {
                        let _e13459 = local_93;
                        let _e13463 = local_94;
                        let _e13467 = local_95;
                        let _e13471 = global_4.member[_e12977];
                        if (_e13471.member == 4u) {
                            if (_e13471.member_1 == 1u) {
                                phi_18056_ = true;
                            } else {
                                phi_18056_ = ((((-(_e13459.x) * _e12961.member_3.x) + (-(_e13463.y) * _e12961.member_3.y)) + (-(_e13467.z) * _e12961.member_3.z)) > 0f);
                            }
                            let _e13486 = phi_18056_;
                            if _e13486 {
                                phi_18065_ = vec3<f32>(_e13471.member_4.x, _e13471.member_4.y, _e13471.member_4.z);
                            } else {
                                phi_18065_ = vec3<f32>();
                            }
                            let _e13493 = phi_18065_;
                            phi_18067_ = _e13493;
                            phi_18068_ = select(true, false, _e13486);
                        } else {
                            phi_18067_ = vec3<f32>();
                            phi_18068_ = true;
                        }
                        let _e13496 = phi_18067_;
                        let _e13498 = phi_18068_;
                        let _e13500 = select(_e13496, vec3<f32>(0f, 0f, 0f), vec3(_e13498));
                        let _e13502 = local_96;
                        if _e13502 {
                            phi_18241_ = 1f;
                        } else {
                            if ((_e368 & 1u) == 0u) {
                                phi_18240_ = 1f;
                            } else {
                                if (_e12979.member_6.z > 0u) {
                                    let _e13511 = global_1.member.member_2[1u];
                                    let _e13512 = (_e13511 + (_e12979.member_6.z - 1u));
                                    let _e13515 = global_6.member[(_e13512 + 1u)];
                                    let _e13517 = global_6.member[_e13512];
                                    let _e13522 = global_2.member[_e12961.member_1];
                                    if (_e13522.member_6.x == 1u) {
                                        let _e13607 = local_100;
                                        let _e13609 = (_e13522.member.x - _e13607.x);
                                        let _e13611 = local_101;
                                        let _e13613 = (_e13522.member.y - _e13611.y);
                                        let _e13615 = local_102;
                                        let _e13617 = (_e13522.member.z - _e13615.z);
                                        let _e13622 = (((_e13609 * _e13609) + (_e13613 * _e13613)) + (_e13617 * _e13617));
                                        let _e13623 = (_e13522.member.w * _e13522.member.w);
                                        if (_e13622 <= (_e13623 * 1.000001f)) {
                                            let _e13638 = (_e12981.x - _e13607.x);
                                            let _e13640 = (_e12981.y - _e13611.y);
                                            let _e13642 = (_e12981.z - _e13615.z);
                                            let _e13647 = (((_e13638 * _e13638) + (_e13640 * _e13640)) + (_e13642 * _e13642));
                                            phi_30002_ = (_e13647 / (((abs((((_e12961.member_2.x * _e13638) + (_e12961.member_2.y * _e13640)) + (_e12961.member_2.z * _e13642))) / sqrt(_e13647)) * 12.566371f) * _e13623));
                                        } else {
                                            let _e13626 = (_e13623 / _e13622);
                                            if (_e13626 != _e13626) {
                                                phi_30021_ = true;
                                            } else {
                                                phi_30021_ = (1f <= _e13626);
                                            }
                                            let _e13630 = phi_30021_;
                                            let _e13631 = select(_e13626, 1f, _e13630);
                                            phi_30002_ = (0.15915494f / (_e13631 / (1f + sqrt((1f - _e13631)))));
                                        }
                                        let _e13663 = phi_30002_;
                                        phi_18236_ = _e13663;
                                    } else {
                                        let _e13527 = local_97;
                                        let _e13529 = (_e12987 - _e13527.x);
                                        let _e13531 = local_98;
                                        let _e13533 = (_e12985 - _e13531.y);
                                        let _e13535 = local_99;
                                        let _e13537 = (_e12983 - _e13535.z);
                                        let _e13542 = (((_e13529 * _e13529) + (_e13533 * _e13533)) + (_e13537 * _e13537));
                                        let _e13553 = (abs((((_e12961.member_2.x * _e13529) + (_e12961.member_2.y * _e13533)) + (_e12961.member_2.z * _e13537))) / sqrt(_e13542));
                                        if (_e13553 <= 0f) {
                                            phi_18235_ = 0f;
                                        } else {
                                            if (_e13522.member_6.x == 0u) {
                                                let _e13568 = (_e13522.member_1.x - _e13522.member.x);
                                                let _e13569 = (_e13522.member_1.y - _e13522.member.y);
                                                let _e13570 = (_e13522.member_1.z - _e13522.member.z);
                                                let _e13575 = (_e13522.member_2.x - _e13522.member.x);
                                                let _e13576 = (_e13522.member_2.y - _e13522.member.y);
                                                let _e13577 = (_e13522.member_2.z - _e13522.member.z);
                                                let _e13580 = ((_e13569 * _e13577) - (_e13576 * _e13570));
                                                let _e13583 = ((_e13570 * _e13575) - (_e13577 * _e13568));
                                                let _e13586 = ((_e13568 * _e13576) - (_e13575 * _e13569));
                                                phi_18232_ = (sqrt((((_e13580 * _e13580) + (_e13583 * _e13583)) + (_e13586 * _e13586))) * 0.5f);
                                            } else {
                                                phi_18232_ = ((12.566371f * _e13522.member.w) * _e13522.member.w);
                                            }
                                            let _e13595 = phi_18232_;
                                            phi_18235_ = (_e13542 / (_e13553 * _e13595));
                                        }
                                        let _e13599 = phi_18235_;
                                        phi_18236_ = _e13599;
                                    }
                                    let _e13665 = phi_18236_;
                                    let _e13666 = ((_e13515 - _e13517) * _e13665);
                                    let _e13668 = local_103;
                                    if (_e13668 >= 300000000000000000000000000000000000000f) {
                                        phi_30042_ = 1f;
                                    } else {
                                        let _e13671 = local_104;
                                        let _e13673 = local_105;
                                        let _e13674 = (_e13671 * _e13673);
                                        let _e13676 = (_e13674 + (_e13666 * _e13666));
                                        if (_e13676 == 0f) {
                                            phi_30039_ = 0f;
                                        } else {
                                            phi_30039_ = (_e13674 / _e13676);
                                        }
                                        let _e13680 = phi_30039_;
                                        phi_30042_ = _e13680;
                                    }
                                    let _e13682 = phi_30042_;
                                    phi_18239_ = _e13682;
                                } else {
                                    phi_18239_ = 1f;
                                }
                                let _e13684 = phi_18239_;
                                phi_18240_ = _e13684;
                            }
                            let _e13686 = phi_18240_;
                            phi_18241_ = _e13686;
                        }
                        let _e13688 = phi_18241_;
                        phi_18263_ = vec3<f32>((_e12975.x + ((_e12989.x * _e13500.x) * _e13688)), (_e12975.y + ((_e12989.y * _e13500.y) * _e13688)), (_e12975.z + ((_e12989.z * _e13500.z) * _e13688)));
                    } else {
                        phi_18263_ = vec3<f32>();
                    }
                    let _e13709 = phi_18263_;
                    let _e13710 = select(select(select(_e12969, false, _e12997), false, _e12992), false, _e13454);
                    let _e13712 = select(select(select(select(_e12967, false, _e13000), false, _e12992), false, _e13454), false, _e13710);
                    let _e13714 = select(_e13710, true, _e13712);
                    let _e13720 = select(_e13453, select(_e13709, _e12975, vec3(_e13714)), vec3(select(select(select(_e13454, false, _e13710), false, _e13712), true, _e13714)));
                    let _e13723 = (_e13720.x * _e301.w);
                    let _e13725 = (_e13720.y * _e301.w);
                    let _e13727 = (_e13720.z * _e301.w);
                    if (_e13723 == _e13723) {
                        let _e13730 = (_e13725 == _e13725);
                        if _e13730 {
                            phi_18308_ = select(true, false, (_e13727 == _e13727));
                        } else {
                            phi_18308_ = false;
                        }
                        let _e13734 = phi_18308_;
                        phi_18309_ = select(true, false, _e13730);
                        phi_18310_ = _e13734;
                    } else {
                        phi_18309_ = true;
                        phi_18310_ = false;
                    }
                    let _e13737 = phi_18309_;
                    let _e13739 = phi_18310_;
                    let _e13742 = select((_e13720 * _e301.w), vec3<f32>(0f, 0f, 0f), vec3(select(_e13739, true, _e13737)));
                    if (_e286 < arrayLength((&global_8.member))) {
                    } else {
                        break;
                    }
                    let _e13751 = global_8.member[_e286][0u];
                    global_8.member[_e286][0u] = (_e13751 + _e13742.x);
                    let _e13756 = global_8.member[_e286][1u];
                    global_8.member[_e286][1u] = (_e13756 + _e13742.y);
                    let _e13761 = global_8.member[_e286][2u];
                    global_8.member[_e286][2u] = (_e13761 + _e13742.z);
                    let _e13766 = global_8.member[_e286][3u];
                    global_8.member[_e286][3u] = (_e13766 + f32(_e12963));
                }
            }
            break;
        }
    }
    return;
}

@compute @workgroup_size(64, 1, 1) 
fn main(@builtin(global_invocation_id) param: vec3<u32>) {
    global = param;
    function_();
}
