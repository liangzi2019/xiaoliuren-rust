use std::{thread, time};

//　六宫的结构体
#[allow(dead_code)]
struct LiuGong {
    yi_gong: String,
    er_gong: String,
    san_gong: String,
    si_gong: String,
    wu_gong: String,
    liu_gong: String,
}

//　打印宫位
#[allow(dead_code)]
pub(crate) fn gong_wei() {
    let gong_wei = LiuGong {
        yi_gong: String::from("大安"),
        er_gong: String::from("留连"),
        san_gong: String::from("速喜"),
        si_gong: String::from("赤口"),
        wu_gong: String::from("小吉"),
        liu_gong: String::from("空亡"),
    };

    println!("####################");
    println!("   宫位信息列表");
    println! {"   # {}: 一宫", gong_wei.yi_gong};
    println!("   # {}: 二宫", gong_wei.er_gong);
    println!("   # {}: 三宫", gong_wei.san_gong);
    println!("   # {}: 四宫", gong_wei.si_gong);
    println!("   # {}: 五宫", gong_wei.wu_gong);
    println!("   # {}: 六宫", gong_wei.liu_gong);
}

//六宫的结构体
pub struct GongWei {
    pub da_an: String,
    pub liu_lian: String,
    pub su_xi: String,
    pub chi_kou: String,
    pub xiao_ji: String,
    pub kong_wang: String,
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

//作者添加内容
pub fn some_other() {
    println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
    println!(" ");
    let ten_millis = time::Duration::from_secs(10);
    thread::sleep(ten_millis);
    println!("      六壬通神暄未然");
    let ten_millis = time::Duration::from_secs(1);
    thread::sleep(ten_millis);
    println!("      造化须知系自身");
    let ten_millis = time::Duration::from_secs(1);
    thread::sleep(ten_millis);
    println!("      阴阳共判劝君明");
    let ten_millis = time::Duration::from_secs(1);
    thread::sleep(ten_millis);
    println!("      虚实明辨壬中求");
    println!("                              /by liang_zi");
    println!(" ");
}
