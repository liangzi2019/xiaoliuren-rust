//每个解法是一个结构体

// use crate::info::list_info;
// use std::time::SystemTime;

//六宫的结构体
pub struct GongWei {
    pub da_an: String,
    pub liu_lian: String,
    pub su_xi: String,
    pub chi_kou: String,
    pub xiao_ji: String,
    pub kong_wang: String,
}
//1:占求财
pub struct ZhanQiuCai {
    qiu_cai: GongWei,
}
//2:占行人
pub struct ZhanXingRen {
    xing_ren: GongWei,
}
//3:占寻人
pub struct ZhanXunRen {
    xun_ren: GongWei,
}
//4:占家先
pub struct ZhanJiaXian {
    jia_xian: GongWei,
}
//5:占失物
pub struct ZhanShiWu {
    shi_wu: GongWei,
}
//6:占病人
pub struct ZhanBingRen {
    bing_ren: GongWei,
}
//7:占走失
pub struct ZhanZouShi {
    zou_shi: GongWei,
}
//8:占访贵
pub struct ZhanFangGui {
    fang_gui: GongWei,
}
//9:占合伙
pub struct ZhanHeHuo {
    he_huo: GongWei,
}
//10:占婚姻
pub struct ZhanHunYin {
    hun_yin: GongWei,
}
//11:占埋葬
pub struct ZhanMaiZang {
    mai_zang: GongWei,
}
//12:占修方
pub struct ZhanXiuFang {
    xiu_fang: GongWei,
}
//13:占送方
pub struct ZhanSongFang {
    song_fang: GongWei,
}
//14:占预兆
pub struct ZhanYuZhao {
    yu_zhao: GongWei,
}
//15:占官非
pub struct ZhanGuanFei {
    guan_fei: GongWei,
}
//16:占风水
pub struct ZhanFengShui {
    feng_shui: GongWei,
}
//17:占虚实
pub struct ZhanXuShi {
    xu_shi: GongWei,
}

//18占其他
pub struct ZhanQiTa {
    qi_ta: GongWei,
}

//实现六个宫位的共性方法
pub trait JieDaAn {
    fn jie_daan(&self) -> String;
}
pub trait JieLiuLian {
    fn jie_liulian(&self) -> String;
}
pub trait JieSuXi {
    fn jie_suxi(&self) -> String;
}
pub trait JieChiKou {
    fn jie_chikou(&self) -> String;
}
pub trait JieXiaoJi {
    fn jie_xiaoji(&self) -> String;
}
pub trait JieKongWang {
    fn jie_kongwang(&self) -> String;
}

//依据落宫解卦
//1 占求财
impl JieDaAn for ZhanQiuCai {
    fn jie_daan(&self) -> String {
        format!("占求财-大安:{}", self.qiu_cai.da_an)
    }
}
impl JieLiuLian for ZhanQiuCai {
    fn jie_liulian(&self) -> String {
        format!("占求财-留连:{}", self.qiu_cai.liu_lian)
    }
}
impl JieSuXi for ZhanQiuCai {
    fn jie_suxi(&self) -> String {
        format!("占求财-速喜:{}", self.qiu_cai.su_xi)
    }
}
impl JieChiKou for ZhanQiuCai {
    fn jie_chikou(&self) -> String {
        format!("占求财-赤口:{}", self.qiu_cai.chi_kou)
    }
}
impl JieXiaoJi for ZhanQiuCai {
    fn jie_xiaoji(&self) -> String {
        format!("占求财-小吉:{}", self.qiu_cai.xiao_ji)
    }
}
impl JieKongWang for ZhanQiuCai {
    fn jie_kongwang(&self) -> String {
        format!("占求财-空亡:{}", self.qiu_cai.kong_wang)
    }
}

//2 占行人
impl JieDaAn for ZhanXingRen {
    fn jie_daan(&self) -> String {
        format!("占行人-大安:{}", self.xing_ren.da_an)
    }
}
impl JieLiuLian for ZhanXingRen {
    fn jie_liulian(&self) -> String {
        format!("占行人-留连:{}", self.xing_ren.liu_lian)
    }
}
impl JieSuXi for ZhanXingRen {
    fn jie_suxi(&self) -> String {
        format!("占行人-留连:{}", self.xing_ren.su_xi)
    }
}
impl JieChiKou for ZhanXingRen {
    fn jie_chikou(&self) -> String {
        format!("占行人-留连:{}", self.xing_ren.chi_kou)
    }
}
impl JieXiaoJi for ZhanXingRen {
    fn jie_xiaoji(&self) -> String {
        format!("占行人-留连:{}", self.xing_ren.xiao_ji)
    }
}
impl JieKongWang for ZhanXingRen {
    fn jie_kongwang(&self) -> String {
        format!("占行人-留连:{}", self.xing_ren.kong_wang)
    }
}
//3:占寻人
impl JieDaAn for ZhanXunRen {
    fn jie_daan(&self) -> String {
        format!("占寻人-大安:{}", self.xun_ren.da_an)
    }
}
impl JieLiuLian for ZhanXunRen {
    fn jie_liulian(&self) -> String {
        format!("占寻人-留连:{}", self.xun_ren.liu_lian)
    }
}
impl JieSuXi for ZhanXunRen {
    fn jie_suxi(&self) -> String {
        format!("占寻人-速喜:{}", self.xun_ren.su_xi)
    }
}
impl JieChiKou for ZhanXunRen {
    fn jie_chikou(&self) -> String {
        format!("占寻人-赤口:{}", self.xun_ren.chi_kou)
    }
}
impl JieXiaoJi for ZhanXunRen {
    fn jie_xiaoji(&self) -> String {
        format!("占寻人-小吉:{}", self.xun_ren.xiao_ji)
    }
}
impl JieKongWang for ZhanXunRen {
    fn jie_kongwang(&self) -> String {
        format!("占寻人-空亡:{}", self.xun_ren.kong_wang)
    }
}
//4:占家先
impl JieDaAn for ZhanJiaXian {
    fn jie_daan(&self) -> String {
        format!("占家先-大安:{}", self.jia_xian.da_an)
    }
}
impl JieLiuLian for ZhanJiaXian {
    fn jie_liulian(&self) -> String {
        format!("占家先-留连:{}", self.jia_xian.liu_lian)
    }
}
impl JieSuXi for ZhanJiaXian {
    fn jie_suxi(&self) -> String {
        format!("占家先-速喜:{}", self.jia_xian.su_xi)
    }
}
impl JieChiKou for ZhanJiaXian {
    fn jie_chikou(&self) -> String {
        format!("占家先:-赤口:{}", self.jia_xian.chi_kou)
    }
}
impl JieXiaoJi for ZhanJiaXian {
    fn jie_xiaoji(&self) -> String {
        format!("占家先-小吉:{}", self.jia_xian.xiao_ji)
    }
}
impl JieKongWang for ZhanJiaXian {
    fn jie_kongwang(&self) -> String {
        format!("占家先-空亡:{}", self.jia_xian.kong_wang)
    }
}

//5:占失物
impl JieDaAn for ZhanShiWu {
    fn jie_daan(&self) -> String {
        format!("占失物-大安:{}", self.shi_wu.da_an)
    }
}
impl JieLiuLian for ZhanShiWu {
    fn jie_liulian(&self) -> String {
        format!("占失物-留连:{}", self.shi_wu.liu_lian)
    }
}
impl JieSuXi for ZhanShiWu {
    fn jie_suxi(&self) -> String {
        format!("占失物-速喜:{}", self.shi_wu.su_xi)
    }
}
impl JieChiKou for ZhanShiWu {
    fn jie_chikou(&self) -> String {
        format!("占失物-赤口:{}", self.shi_wu.chi_kou)
    }
}
impl JieXiaoJi for ZhanShiWu {
    fn jie_xiaoji(&self) -> String {
        format!("占失物-小吉:{}", self.shi_wu.xiao_ji)
    }
}
impl JieKongWang for ZhanShiWu {
    fn jie_kongwang(&self) -> String {
        format!("占失物-空亡:{}", self.shi_wu.kong_wang)
    }
}
//6:占病人
impl JieDaAn for ZhanBingRen {
    fn jie_daan(&self) -> String {
        format!("占病人-大安:{}", self.bing_ren.da_an)
    }
}
impl JieLiuLian for ZhanBingRen {
    fn jie_liulian(&self) -> String {
        format!("占病人-留连:{}", self.bing_ren.liu_lian)
    }
}
impl JieSuXi for ZhanBingRen {
    fn jie_suxi(&self) -> String {
        format!("占病人-速喜:{}", self.bing_ren.su_xi)
    }
}
impl JieChiKou for ZhanBingRen {
    fn jie_chikou(&self) -> String {
        format!("占病人-赤口:{}", self.bing_ren.chi_kou)
    }
}
impl JieXiaoJi for ZhanBingRen {
    fn jie_xiaoji(&self) -> String {
        format!("占病人-小吉:{}", self.bing_ren.xiao_ji)
    }
}
impl JieKongWang for ZhanBingRen {
    fn jie_kongwang(&self) -> String {
        format!("占病人-空亡:{}", self.bing_ren.kong_wang)
    }
}
//7:占走失
impl JieDaAn for ZhanZouShi {
    fn jie_daan(&self) -> String {
        format!("占病人-大安:{}", self.zou_shi.da_an)
    }
}
impl JieLiuLian for ZhanZouShi {
    fn jie_liulian(&self) -> String {
        format!("占病人-留连:{}", self.zou_shi.liu_lian)
    }
}
impl JieSuXi for ZhanZouShi {
    fn jie_suxi(&self) -> String {
        format!("占病人-速喜:{}", self.zou_shi.su_xi)
    }
}
impl JieChiKou for ZhanZouShi {
    fn jie_chikou(&self) -> String {
        format!("占病人-赤口:{}", self.zou_shi.chi_kou)
    }
}
impl JieXiaoJi for ZhanZouShi {
    fn jie_xiaoji(&self) -> String {
        format!("占病人-小吉:{}", self.zou_shi.xiao_ji)
    }
}
impl JieKongWang for ZhanZouShi {
    fn jie_kongwang(&self) -> String {
        format!("占病人-空亡:{}", self.zou_shi.kong_wang)
    }
}
//8:占访贵
impl JieDaAn for ZhanFangGui {
    fn jie_daan(&self) -> String {
        format!("占病人-大安:{}", self.fang_gui.da_an)
    }
}
impl JieLiuLian for ZhanFangGui {
    fn jie_liulian(&self) -> String {
        format!("占病人-留连:{}", self.fang_gui.liu_lian)
    }
}
impl JieSuXi for ZhanFangGui {
    fn jie_suxi(&self) -> String {
        format!("占病人-速喜:{}", self.fang_gui.su_xi)
    }
}
impl JieChiKou for ZhanFangGui {
    fn jie_chikou(&self) -> String {
        format!("占病人-赤口:{}", self.fang_gui.chi_kou)
    }
}
impl JieXiaoJi for ZhanFangGui {
    fn jie_xiaoji(&self) -> String {
        format!("占病人-小吉:{}", self.fang_gui.xiao_ji)
    }
}
impl JieKongWang for ZhanFangGui {
    fn jie_kongwang(&self) -> String {
        format!("占病人-空亡:{}", self.fang_gui.kong_wang)
    }
}
//9:占合伙
impl JieDaAn for ZhanHeHuo {
    fn jie_daan(&self) -> String {
        format!("占病人-大安:{}", self.he_huo.da_an)
    }
}
impl JieLiuLian for ZhanHeHuo {
    fn jie_liulian(&self) -> String {
        format!("占病人-留连:{}", self.he_huo.liu_lian)
    }
}
impl JieSuXi for ZhanHeHuo {
    fn jie_suxi(&self) -> String {
        format!("占病人-速喜:{}", self.he_huo.su_xi)
    }
}
impl JieChiKou for ZhanHeHuo {
    fn jie_chikou(&self) -> String {
        format!("占病人-赤口:{}", self.he_huo.chi_kou)
    }
}
impl JieXiaoJi for ZhanHeHuo {
    fn jie_xiaoji(&self) -> String {
        format!("占病人-小吉:{}", self.he_huo.xiao_ji)
    }
}
impl JieKongWang for ZhanHeHuo {
    fn jie_kongwang(&self) -> String {
        format!("占病人-空亡:{}", self.he_huo.kong_wang)
    }
}
//10:占婚姻
impl JieDaAn for ZhanHunYin {
    fn jie_daan(&self) -> String {
        format!("占病人-大安:{}", self.hun_yin.da_an)
    }
}
impl JieLiuLian for ZhanHunYin {
    fn jie_liulian(&self) -> String {
        format!("占病人-留连:{}", self.hun_yin.liu_lian)
    }
}
impl JieSuXi for ZhanHunYin {
    fn jie_suxi(&self) -> String {
        format!("占病人-速喜:{}", self.hun_yin.su_xi)
    }
}
impl JieChiKou for ZhanHunYin {
    fn jie_chikou(&self) -> String {
        format!("占病人-赤口:{}", self.hun_yin.chi_kou)
    }
}
impl JieXiaoJi for ZhanHunYin {
    fn jie_xiaoji(&self) -> String {
        format!("占病人-小吉:{}", self.hun_yin.xiao_ji)
    }
}
impl JieKongWang for ZhanHunYin {
    fn jie_kongwang(&self) -> String {
        format!("占病人-空亡:{}", self.hun_yin.kong_wang)
    }
}
//11 占埋葬
impl JieDaAn for ZhanMaiZang {
    fn jie_daan(&self) -> String {
        format!("占病人-大安:{}", self.mai_zang.da_an)
    }
}
impl JieLiuLian for ZhanMaiZang {
    fn jie_liulian(&self) -> String {
        format!("占病人-留连:{}", self.mai_zang.liu_lian)
    }
}
impl JieSuXi for ZhanMaiZang {
    fn jie_suxi(&self) -> String {
        format!("占病人-速喜:{}", self.mai_zang.su_xi)
    }
}
impl JieChiKou for ZhanMaiZang {
    fn jie_chikou(&self) -> String {
        format!("占病人-赤口:{}", self.mai_zang.chi_kou)
    }
}
impl JieXiaoJi for ZhanMaiZang {
    fn jie_xiaoji(&self) -> String {
        format!("占病人-小吉:{}", self.mai_zang.xiao_ji)
    }
}
impl JieKongWang for ZhanMaiZang {
    fn jie_kongwang(&self) -> String {
        format!("占病人-空亡:{}", self.mai_zang.kong_wang)
    }
}
//12:占修方
impl JieDaAn for ZhanXiuFang {
    fn jie_daan(&self) -> String {
        format!("占病人-大安:{}", self.xiu_fang.da_an)
    }
}
impl JieLiuLian for ZhanXiuFang {
    fn jie_liulian(&self) -> String {
        format!("占病人-留连:{}", self.xiu_fang.liu_lian)
    }
}
impl JieSuXi for ZhanXiuFang {
    fn jie_suxi(&self) -> String {
        format!("占病人-速喜:{}", self.xiu_fang.su_xi)
    }
}
impl JieChiKou for ZhanXiuFang {
    fn jie_chikou(&self) -> String {
        format!("占病人-赤口:{}", self.xiu_fang.chi_kou)
    }
}
impl JieXiaoJi for ZhanXiuFang {
    fn jie_xiaoji(&self) -> String {
        format!("占病人-小吉:{}", self.xiu_fang.xiao_ji)
    }
}
impl JieKongWang for ZhanXiuFang {
    fn jie_kongwang(&self) -> String {
        format!("占病人-空亡:{}", self.xiu_fang.kong_wang)
    }
}
//13:占送方
impl JieDaAn for ZhanSongFang {
    fn jie_daan(&self) -> String {
        format!("占病人-大安:{}", self.song_fang.da_an)
    }
}
impl JieLiuLian for ZhanSongFang {
    fn jie_liulian(&self) -> String {
        format!("占病人-留连:{}", self.song_fang.liu_lian)
    }
}
impl JieSuXi for ZhanSongFang {
    fn jie_suxi(&self) -> String {
        format!("占病人-速喜:{}", self.song_fang.su_xi)
    }
}
impl JieChiKou for ZhanSongFang {
    fn jie_chikou(&self) -> String {
        format!("占病人-赤口:{}", self.song_fang.chi_kou)
    }
}
impl JieXiaoJi for ZhanSongFang {
    fn jie_xiaoji(&self) -> String {
        format!("占病人-小吉:{}", self.song_fang.xiao_ji)
    }
}
impl JieKongWang for ZhanSongFang {
    fn jie_kongwang(&self) -> String {
        format!("占病人-空亡:{}", self.song_fang.kong_wang)
    }
}
//14:占预兆
impl JieDaAn for ZhanYuZhao {
    fn jie_daan(&self) -> String {
        format!("占病人-大安:{}", self.yu_zhao.da_an)
    }
}
impl JieLiuLian for ZhanYuZhao {
    fn jie_liulian(&self) -> String {
        format!("占病人-留连:{}", self.yu_zhao.liu_lian)
    }
}
impl JieSuXi for ZhanYuZhao {
    fn jie_suxi(&self) -> String {
        format!("占病人-速喜:{}", self.yu_zhao.su_xi)
    }
}
impl JieChiKou for ZhanYuZhao {
    fn jie_chikou(&self) -> String {
        format!("占病人-赤口:{}", self.yu_zhao.chi_kou)
    }
}
impl JieXiaoJi for ZhanYuZhao {
    fn jie_xiaoji(&self) -> String {
        format!("占病人-小吉:{}", self.yu_zhao.xiao_ji)
    }
}
impl JieKongWang for ZhanYuZhao {
    fn jie_kongwang(&self) -> String {
        format!("占病人-空亡:{}", self.yu_zhao.kong_wang)
    }
}
//15:占管非
impl JieDaAn for ZhanGuanFei {
    fn jie_daan(&self) -> String {
        format!("占病人-大安:{}", self.guan_fei.da_an)
    }
}
impl JieLiuLian for ZhanGuanFei {
    fn jie_liulian(&self) -> String {
        format!("占病人-留连:{}", self.guan_fei.liu_lian)
    }
}
impl JieSuXi for ZhanGuanFei {
    fn jie_suxi(&self) -> String {
        format!("占病人-速喜:{}", self.guan_fei.su_xi)
    }
}
impl JieChiKou for ZhanGuanFei {
    fn jie_chikou(&self) -> String {
        format!("占病人-赤口:{}", self.guan_fei.chi_kou)
    }
}
impl JieXiaoJi for ZhanGuanFei {
    fn jie_xiaoji(&self) -> String {
        format!("占病人-小吉:{}", self.guan_fei.xiao_ji)
    }
}
impl JieKongWang for ZhanGuanFei {
    fn jie_kongwang(&self) -> String {
        format!("占病人-空亡:{}", self.guan_fei.kong_wang)
    }
}
//16:占风水
impl JieDaAn for ZhanFengShui {
    fn jie_daan(&self) -> String {
        format!("占病人-大安:{}", self.feng_shui.da_an)
    }
}
impl JieLiuLian for ZhanFengShui {
    fn jie_liulian(&self) -> String {
        format!("占病人-留连:{}", self.feng_shui.liu_lian)
    }
}
impl JieSuXi for ZhanFengShui {
    fn jie_suxi(&self) -> String {
        format!("占病人-速喜:{}", self.feng_shui.su_xi)
    }
}
impl JieChiKou for ZhanFengShui {
    fn jie_chikou(&self) -> String {
        format!("占病人-赤口:{}", self.feng_shui.chi_kou)
    }
}
impl JieXiaoJi for ZhanFengShui {
    fn jie_xiaoji(&self) -> String {
        format!("占病人-小吉:{}", self.feng_shui.xiao_ji)
    }
}
impl JieKongWang for ZhanFengShui {
    fn jie_kongwang(&self) -> String {
        format!("占病人-空亡:{}", self.feng_shui.kong_wang)
    }
}
//17:占虚实
impl JieDaAn for ZhanXuShi {
    fn jie_daan(&self) -> String {
        format!("占病人-大安:{}", self.xu_shi.da_an)
    }
}
impl JieLiuLian for ZhanXuShi {
    fn jie_liulian(&self) -> String {
        format!("占病人-留连:{}", self.xu_shi.liu_lian)
    }
}
impl JieSuXi for ZhanXuShi {
    fn jie_suxi(&self) -> String {
        format!("占病人-速喜:{}", self.xu_shi.su_xi)
    }
}
impl JieChiKou for ZhanXuShi {
    fn jie_chikou(&self) -> String {
        format!("占病人-赤口:{}", self.xu_shi.chi_kou)
    }
}
impl JieXiaoJi for ZhanXuShi {
    fn jie_xiaoji(&self) -> String {
        format!("占病人-小吉:{}", self.xu_shi.xiao_ji)
    }
}
impl JieKongWang for ZhanXuShi {
    fn jie_kongwang(&self) -> String {
        format!("占病人-空亡:{}", self.xu_shi.kong_wang)
    }
}
//占其他
impl JieDaAn for ZhanQiTa {
    fn jie_daan(&self) -> String {
        format!("占病人-大安:{}", self.qi_ta.da_an)
    }
}
impl JieLiuLian for ZhanQiTa {
    fn jie_liulian(&self) -> String {
        format!("占病人-留连:{}", self.qi_ta.liu_lian)
    }
}
impl JieSuXi for ZhanQiTa {
    fn jie_suxi(&self) -> String {
        format!("占病人-速喜:{}", self.qi_ta.su_xi)
    }
}
impl JieChiKou for ZhanQiTa {
    fn jie_chikou(&self) -> String {
        format!("占病人-赤口:{}", self.qi_ta.chi_kou)
    }
}
impl JieXiaoJi for ZhanQiTa {
    fn jie_xiaoji(&self) -> String {
        format!("占病人-小吉:{}", self.qi_ta.xiao_ji)
    }
}
impl JieKongWang for ZhanQiTa {
    fn jie_kongwang(&self) -> String {
        format!("占病人-空亡:{}", self.qi_ta.kong_wang)
    }
}

//打印解卦信息　_n为选择占卜内容　即: value_in_info(n: u32)
pub fn display_info(_n1: u32, number: u32) {
    if _n1 == 1 {
        info_zhan_qiu_cai(number); //1:占求财
    }
    if _n1 == 2 {
        info_zhan_xing_ren(number); //2:占行人
    }
}
//1:占求财
pub fn info_zhan_qiu_cai(number: u32) {
    let zhanqiucai = ZhanQiuCai {
        qiu_cai: GongWei {
            da_an: String::from("大安来不得"),
            liu_lian: String::from("流连本无收"),
            su_xi: String::from("速喜加倍利"),
            chi_kou: String::from("白虎支才多"),
            xiao_ji: String::from("小吉本有利"),
            kong_wang: String::from("空亡本无收"),
        },
    };
    //依据第三宫落宫解课
    match number {
        1 => println!("info:{}", zhanqiucai.jie_daan()),
        2 => println!("info:{}", zhanqiucai.jie_liulian()),
        3 => println!("info:{}", zhanqiucai.jie_suxi()),
        4 => println!("info:{}", zhanqiucai.jie_chikou()),
        5 => println!("info:{}", zhanqiucai.jie_xiaoji()),
        6 => println!("info:{}", zhanqiucai.jie_kongwang()),
        _ => println!("落宫数字异常无法解卦..."),
    }
}
//2:占行人 number 为落到第三宫的数字
pub fn info_zhan_xing_ren(number: u32) {
    let zhanxingren = ZhanXingRen {
        xing_ren: GongWei {
            da_an: String::from("大安身未动"),
            liu_lian: String::from("流连人未见"),
            su_xi: String::from("速喜立时见"),
            chi_kou: String::from("白虎在他乡 "),
            xiao_ji: String::from("小吉终须到"),
            kong_wang: String::from("空亡即世亡"),
        },
    };
    //依据第三宫落宫解课
    match number {
        1 => println!("info:{}", zhanxingren.jie_daan()),
        2 => println!("info:{}", zhanxingren.jie_liulian()),
        3 => println!("info:{}", zhanxingren.jie_suxi()),
        4 => println!("info:{}", zhanxingren.jie_chikou()),
        5 => println!("info:{}", zhanxingren.jie_xiaoji()),
        6 => println!("info:{}", zhanxingren.jie_kongwang()),
        _ => println!("落宫数字异常无法解卦..."),
    }
}
