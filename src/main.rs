use std::io;

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
fn gong_wei() {
    let gong_wei = LiuGong {
        yi_gong: String::from("大安"),
        er_gong: String::from("留连"),
        san_gong: String::from("速喜"),
        si_gong: String::from("赤口"),
        wu_gong: String::from("小吉"),
        liu_gong: String::from("空亡"),
    };

    println! {"{}: 一宫", gong_wei.yi_gong};
    println!("{}: 二宫", gong_wei.er_gong);
    println!("{}: 三宫", gong_wei.san_gong);
    println!("{}: 四宫", gong_wei.si_gong);
    println!("{}: 五宫", gong_wei.wu_gong);
    println!("{}: 六宫", gong_wei.liu_gong);
}

//获取用户输入
fn get_input() -> (u32, u32, u32) {
    println!("输入十以内的任意三个数字");

    let mut inputone = String::new();
    io::stdin().read_line(&mut inputone).expect("输入错误");

    let mut inputtwo = String::new();
    io::stdin().read_line(&mut inputtwo).expect("错误的输入");

    let mut inputthree = String::new();
    io::stdin().read_line(&mut inputthree).expect("错误的输入");

    //解析字符串内容 确认落宫数字
    let _one: u32 = inputone.trim().parse::<u32>().unwrap();
    let _two: u32 = inputtwo.trim().parse::<u32>().unwrap();
    let _three: u32 = inputthree.trim().parse::<u32>().unwrap();
    return (_one, _two, _three);
}

//定宫
//数字为落宫位置　根据数字定宫位：
//１大安; ２留连;　３速喜; 4赤口; 5小吉; ０空亡;
//同一个宫位循环数字
//大安　１  7(6*1+1)  13(6*2+1) 19(6*3+1)  25(6*4+1)  31(6*5+1) 大安括号内都加一
//留连　２　8(6*1+2)  14(6*2+2)  20(6*3+2)  26(6*4+2) 32(6*5+2) 留连括号内都加２
//速喜　３　9(6*1+3)  15(6*2+3)  21(6*3+3)  27(6*4+3) 33(6*5+3) 速喜括号内都加三
//赤口  4  10(6*1+4)  16(6*2+4)  22(6*3+4) 28(6*4+4)  34(6*5+4) 赤口括号内都加４
//小吉  5  11(6*1+5)  17(6*2+5)  23(6*3+5) 29(6*4+5)  35(6*5+5) 小吉括号内都加５
//空亡  6  12(6*1+6)  18(6*2+6)  24(6*3+6) 30(6*4+6)  36(6*5+6) 空亡括号内都加6

fn ding_gong() {
    let (one, two, three) = get_input();
    //    println!("{},{},{}", one, two, three);

    //根据确定落宫数字来定宫
    //定一宫
    let yigong: u32 = one % 6;
    println!("数字{}落{}宫", one, yigong);
    //定二 三宫
    if two > 6 {
        let ergong: u32 = yigong + two - 7;
        println!("数字{}落{}宫", two, ergong);
        let sangong: u32 = ergong + three - 7;
        println!("数字{}落{}宫", three, sangong);
    } else if two <= 6 {
        let ergong: u32 = yigong + two - 1;
        println!("数字{}落{}宫", two, ergong);
        let sangong: u32 = ergong + three - 1;
        println!("数字{}落{}宫", three, sangong);
    }
}

fn main() {
    //调用宫位
    gong_wei();
    //调用　获取用户输入
    // get_input();
    //定宫
    ding_gong();
}
