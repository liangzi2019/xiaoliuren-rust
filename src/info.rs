//卜法卷可卜算内容列表
use crate::jie_gua::display_info;
use std::io;
use std::process::exit;

//列出可占卜信息
pub fn list_info() -> u32 {
    println!(
        "道家小六壬卜法卷
        测算内容：
        1:占求财 2:占行人 3:占寻人 
        4:占家先 5:占失物 6:占病人 
        7:占走失 8:占访贵 9:占合伙 
        10:占婚姻 11:占埋葬 12:占修方
        13:占送方 14:占预兆 15:占官非 
        16:占风水 17:占虚实 18:占其他"
    );
    println!("根据数字选择要测算的内容");
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("输入错误...");
    // let _n: u32 = input.trim().parse::<u32>().unwrap();
    //过滤掉用户的错误输入
    let _n: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("输入错误　程序退出...");
            exit(1);
        }
    };

    value_in_info(_n);
    return _n;
}
//根据输入遍历占卜信息
pub fn value_in_info(n: u32) {
    let mut _n1 = n;
    //println!("n in value = {}", _n1);
    if _n1 == 1 {
        println!("选择{}: 占求才", _n1);
        qi_ke(n);
    }
    if _n1 == 2 {
        println!("选择{}: 占行人", _n1);
        qi_ke(n);
    }
    if _n1 == 3 {
        println!("选择{}:占寻人", _n1)
    }
    if _n1 == 4 {
        println!("选择{}: 占家先", _n1);
        qi_ke(n);
    }
    if _n1 == 5 {
        println!("选择{}: 占失物", _n1);
        qi_ke(n);
    }
    if _n1 == 6 {
        println!("选择{}: 占病人", _n1);
        qi_ke(n);
    }
    if _n1 == 7 {
        println!("选择{}: 占走失", _n1);
        qi_ke(n);
    }
    if _n1 == 8 {
        println!("选择{}: 占访贵", _n1);
        qi_ke(n);
    }
    if _n1 == 9 {
        println!("选择{}: 占合伙", _n1);
        qi_ke(n);
    }
    if _n1 == 10 {
        println!("选择{}: 占婚姻", _n1);
        qi_ke(n);
    }
    if _n1 == 11 {
        println!("选择{}: 占埋葬", _n1);
        qi_ke(n);
    }
    if _n1 == 12 {
        println!("选择{}: 占修方", _n1);
        qi_ke(n);
    }
    if _n1 == 13 {
        println!("选择{}: 占送方", _n1);
        qi_ke(n);
    }
    if _n1 == 14 {
        println!("选择{}: 占预兆", _n1);
        qi_ke(n);
    }
    if _n1 == 15 {
        println!("选择{}: 占官非", _n1);
        qi_ke(n);
    }
    if _n1 == 16 {
        println!("选择{}: 占风水", _n1);
        qi_ke(n);
    }
    if _n1 == 17 {
        println!("选择{}: 占虚实", _n1);
        qi_ke(n);
    }
    if _n1 == 18 {
        println!("选择{}: 占其他", _n1);
        qi_ke(n);
    }

    if _n1 < 1 || _n1 > 18 {
        println!("问卜已超范围 退出...");
        exit(1);
    }
}
//获取用户输入
pub fn get_input() -> (u32, u32, u32) {
    println!("输入十以内的任意三个数字");
    println!("如果安农历时间起课　输入农历的月　日　时　时：十二时辰");

    let mut inputone = String::new();
    io::stdin().read_line(&mut inputone).expect("输入错误");

    let mut inputtwo = String::new();
    io::stdin().read_line(&mut inputtwo).expect("错误的输入");

    let mut inputthree = String::new();
    io::stdin().read_line(&mut inputthree).expect("错误的输入");
    /*
        //解析字符串内容 确认落宫数字
        let _one: u32 = inputone.trim().parse::<u32>().unwrap();
        let _two: u32 = inputtwo.trim().parse::<u32>().unwrap();
        let _three: u32 = inputthree.trim().parse::<u32>().unwrap();
    */
    //过滤无效的输入
    let _one: u32 = match inputone.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("下一次请输入数字　现在程序退出...");
            exit(1);
        }
    };
    let _two: u32 = match inputtwo.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("下一次请输入数字　现在程序退出...");
            exit(1);
        }
    };
    let _three: u32 = match inputthree.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("下一次请输入数字　现在程序退出...");
            exit(1);
        }
    };

    return (_one, _two, _three);
}

//根据输入起课
pub fn qi_ke(n: u32) {
    let (one, two, three) = get_input();

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
        //根据落宫起课
        display_info(n, sangong);
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
        //根据落宫起课
        display_info(n, sangong);
    }
}
