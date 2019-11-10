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
fn get_input() {
    println!("输入十以内的任意三个数字");
    let mut inputone = String::new();
    io::stdin().read_line(&mut inputone).expect("输入错误");
    // println!("one = {}", inputone.trim_end());

    let mut inputtwo = String::new();
    io::stdin().read_line(&mut inputtwo).expect("错误的输入");

    let mut inputthree = String::new();
    io::stdin().read_line(&mut inputthree).expect("错误的输入");

    //解析字符串内容 确认落宫数字
    let _one: u32 = inputone.trim().parse::<u32>().unwrap();
    let _two: u32 = inputone.trim().parse::<u32>().unwrap();
    let _three: u32 = inputone.trim().parse::<u32>().unwrap();

    //根据确定落宫数字来定宫
    let yigong: u32 = _one % 6;
    //let ergong: u32 = one % 6;
    //let sangong: u32 = one % 6;
    println!("数字{}落{}宫", _one, yigong);
}

fn main() {
    //调用宫位
    gong_wei();
    //调用　获取用户输入
    get_input();
}
