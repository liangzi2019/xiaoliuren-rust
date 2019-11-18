pub mod info;
use crate::info::list_info;
pub mod jie_gua;
// use crate::jie_gua::display_info;

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

/*
定宫
数字为落宫位置　根据数字定宫位：
１大安; ２留连;　３速喜; 4赤口; 5小吉; ０空亡;
同一个宫位循环数字
大安　１  7(6*1+1)  13(6*2+1) 19(6*3+1)  25(6*4+1)  31(6*5+1) 大安括号内都加一
留连　２　8(6*1+2)  14(6*2+2)  20(6*3+2)  26(6*4+2) 32(6*5+2) 留连括号内都加２
速喜　３　9(6*1+3)  15(6*2+3)  21(6*3+3)  27(6*4+3) 33(6*5+3) 速喜括号内都加三
赤口  4  10(6*1+4)  16(6*2+4)  22(6*3+4) 28(6*4+4)  34(6*5+4) 赤口括号内都加４
小吉  5  11(6*1+5)  17(6*2+5)  23(6*3+5) 29(6*4+5)  35(6*5+5) 小吉括号内都加５
空亡  6  12(6*1+6)  18(6*2+6)  24(6*3+6) 30(6*4+6)  36(6*5+6) 空亡括号内都加6
*/

fn main() {
    list_info();

    //调用宫位
    gong_wei();
}
