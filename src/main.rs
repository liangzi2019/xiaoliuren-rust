pub mod info;
use crate::info::list_info;

//　六宫的结构体
struct LiuGong {
    yigong: String,
    ergong: String,
    sangong: String,
    sigong: String,
    wugong: String,
    liugong: String,
}

//　打印宫位
fn gong_wei() {
    let gong_wei = LiuGong {
        yigong: String::from("大安"),
        ergong: String::from("留连"),
        sangong: String::from("速喜"),
        sigong: String::from("赤口"),
        wugong: String::from("小吉"),
        liugong: String::from("空亡"),
    };

    println!("####################");
    println!("   宫位信息列表");
    println! {"   # {}: 一宫", gong_wei.yigong};
    println!("   # {}: 二宫", gong_wei.ergong);
    println!("   # {}: 三宫", gong_wei.sangong);
    println!("   # {}: 四宫", gong_wei.sigong);
    println!("   # {}: 五宫", gong_wei.wugong);
    println!("   # {}: 六宫", gong_wei.liugong);
}

fn main() {
    list_info();

    //打印宫位信息
    gong_wei();
}
