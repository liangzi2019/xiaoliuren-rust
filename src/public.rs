use std::{thread, time};

//　六宫的结构体
struct LiuGong {
    yi_gong: String,
    er_gong: String,
    san_gong: String,
    si_gong: String,
    wu_gong: String,
    liu_gong: String,
}

//　打印宫位

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

pub fn some_other() {
    println!("::::::::::::::::::::::::::::");
    println!(" ");
    let ten_millis = time::Duration::from_secs(2);
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
    println!(" ");
}
