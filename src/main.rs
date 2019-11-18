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
//获取用户输入
fn get_input() -> (u32, u32, u32) {
    println!("输入十以内的任意三个数字");
    println!("如果安农历时间起课　输入农历的月　日　时　时：十二时辰");

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
*/

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
/*
fn ding_gong() {
    let (one, two, three) = get_input();
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
*/
/*
fn qike() {
    let (one, two, three) = get_input();
    /*
    let yigong: Wrapping<u32> = Wrapping::<u32>(one) % Wrapping(6);
    let ergong: Wrapping<u32> = yigong + Wrapping(two) - Wrapping(7);
    let sangong: Wrapping<u32> = (ergong) + Wrapping(three) - Wrapping(1);

    println!("数字{}落{}宫", one, yigong);
    println!("数字{}落{}宫", two, ergong);
    println!("数字{}落{}宫", three, sangong);
    println!("依据以上数字起课为:");
    */
    println!("依据以上数字起课为:");
    //根据确定落宫数字来定宫
    //定一宫
    let yigong: u32 = one % 6;
    // println!("数字{}落{}宫", one, yigong);
    match yigong {
        1 => println!("大安"),
        2 => println!("留连"),
        3 => println!("速喜"),
        4 => println!("赤口"),
        5 => println!("小吉"),
        0 => println!("空亡"),
        _ => println!("一宫什么鬼?"),
    }
    //定二 三宫
    if two > 6 {
        let ergong: u32 = yigong + two - 7;
        //println!("数字{}落{}宫", two, ergong);
        match ergong {
            1 | 7 | 13 | 19 | 25 | 31 | 37 => println!("大安"),
            2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
            3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
            4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
            5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
            0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
            _ => println!("二宫什么鬼?"),
        }
        let sangong: u32 = ergong + three - 7;
        // println!("数字{}落{}宫", three, sangong);
        match sangong {
            1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("大安"),
            2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
            3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
            4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
            5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
            0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
            _ => println!("三宫什么鬼?"),
        }
    } else if two <= 6 {
        let ergong: u32 = yigong + two - 1;
        // println!("数字{}落{}宫", two, ergong);
        match ergong {
            1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("大安"),
            2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
            3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
            4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
            5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
            0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
            _ => println!("二宫什么鬼?"),
        }
        let sangong: u32 = ergong + three - 1;
        // println!("数字{}落{}宫", three, sangong);
        match sangong {
            1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("大安"),
            2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
            3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
            4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
            5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
            0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
            _ => println!("三宫什么鬼?"),
        }
    }
}
*/
fn main() {
    list_info();

    //调用宫位
    gong_wei();
}
