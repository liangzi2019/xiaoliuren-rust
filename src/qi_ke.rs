//卜法卷可卜算内容列表

use crate::jie_gua::display_info;
use crate::shi_wu_jue::ask_swj;
use crate::xiao_diao_qiao::diao_qiao;
use crate::zhang_zhong_jue::ask_zzj;
use std::process::exit;
use std::{io, thread, time};

//提供选择　数字起课 or 时辰起课
pub fn select() -> u32 {
    println!(
        "\
         分类断起课选择 1\n\
         失物诀起课选择 2\n"
    );
    let _n = 1;
    let mut _t = Box::new(2);
    let mut input_n = String::new();
    io::stdin().read_line(&mut input_n).expect("输入错误");
    let mut _one: u32 = match input_n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("下一次请输入数字　现在程序退出...");
            exit(1);
        }
    };
    if _one == _n {
        //按数字起课
        list_info();
    } else if _one == *_t {
        //按时辰起课
        //get_input_time();
        qi_ke_time()
    }
    return *_t;
}

//按照数字起课　列出分类断可占卜信息
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
//按照数字起课　根据输入选择对应占卜信息
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
        println!("选择{}:占寻人", _n1);
        qi_ke(n);
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
        println!("问卜已超范围　退出...");
        exit(1);
    }
}

//按数字起课　获取用户输入
pub fn get_input_number() -> (u32, u32, u32) {
    println!("输入十以内的任意三个数字");
    //println!("如果安农历时间起课　输入农历的月　日　时　时：十二时辰");

    let mut input_one = String::new();
    io::stdin().read_line(&mut input_one).expect("输入错误");

    let mut input_two = String::new();
    io::stdin().read_line(&mut input_two).expect("错误的输入");

    let mut input_three = String::new();
    io::stdin().read_line(&mut input_three).expect("错误的输入");
    /*
        //解析字符串内容 确认落宫数字
        let _one: u32 = inputone.trim().parse::<u32>().unwrap();
        let _two: u32 = inputtwo.trim().parse::<u32>().unwrap();
        let _three: u32 = inputthree.trim().parse::<u32>().unwrap();
    */
    //过滤无效的输入
    let mut _one: u32 = match input_one.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("下一次请输入数字　现在程序退出...");
            exit(1);
        }
    };
    let mut _two: u32 = match input_two.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("下一次请输入数字　现在程序退出...");
            exit(1);
        }
    };
    let mut _three: u32 = match input_three.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("下一次请输入数字　现在程序退出...");
            exit(1);
        }
    };
    //判断输入数字是否在起课允许的数字范围之内
    if (_one > 9 || _one < 1) || (_two > 9 || _two < 1) || (_three > 9 || _three < 1) {
        println!("输入数字不在起课范围内 退出...");
        exit(1);
    }
    return (_one, _two, _three);
}

//按时辰起课　获取用户输入
pub fn get_input_time() -> (u32, u32, u32) {
    println!("按农历时间起课　输入农历的月　日　时　(时：十二时辰)");

    let mut input_one = String::new();
    io::stdin().read_line(&mut input_one).expect("输入错误");

    let mut input_two = String::new();
    io::stdin().read_line(&mut input_two).expect("错误的输入");

    let mut input_three = String::new();
    io::stdin().read_line(&mut input_three).expect("错误的输入");

    //过滤无效的输入
    let mut _one: u32 = match input_one.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("下一次请输入数字　现在程序退出...");
            exit(1);
        }
    };
    let mut _two: u32 = match input_two.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("下一次请输入数字　现在程序退出...");
            exit(1);
        }
    };
    let mut _three: u32 = match input_three.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("下一次请输入数字　现在程序退出...");
            exit(1);
        }
    };
    //判断输入数字是否在起课允许的数字范围之内
    if (_one > 30 || _one < 1) || (_two > 30 || _two < 1) || (_three > 30 || _three < 1) {
        println!("输入的　月　日　时　不在起课范围内 退出...");
        exit(1);
    }
    return (_one, _two, _three);
}

//按照数字起课　根据输入起课
pub fn qi_ke(n: u32) {
    let (one, two, three) = get_input_number();

    println!("依据以上数字起课为:");
    //根据确定落宫数字来定宫
    //定一宫
    let yigong: u32 = one % 6;
    //println!("数字{}落{}宫", one, yigong);
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
    //二宫数字大于六的情况
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
        //如果三宫数字大于６－１　小于６－１例如:(三宫数字 1　９　１)
        if three > 6 {
            let sangong: u32 = ergong + three - 7;
            //println!("数字{}落{}宫", three, sangong);
            match sangong {
                1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("大安"),
                2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
                3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
                4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
                5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
                0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
                _ => println!("三宫什么鬼?"),
            }
            //画一个界限线
            println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
            let ten_millis = time::Duration::from_secs(3);
            thread::sleep(ten_millis);
            //根据落宫起课
            display_info(n, sangong);
            //小掉桥部分
            diao_qiao(sangong);
            //掌中诀部分
            ask_zzj(sangong);
        } else if three <= 6 {
            let sangong: u32 = ergong + three - 1;
            //println!("数字{}落{}宫", three, sangong);
            match sangong {
                1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("大安"),
                2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
                3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
                4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
                5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
                0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
                _ => println!("三宫什么鬼?"),
            }
            //画一个界限线
            println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
            let ten_millis = time::Duration::from_secs(3);
            thread::sleep(ten_millis);
            //根据落宫起课
            display_info(n, sangong);
            //小掉桥部分
            diao_qiao(sangong);
            //掌中诀部分
            ask_zzj(sangong);
        }
    //二宫起课数字小于等于６的情况
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
        //println!("数字{}落{}宫", three, sangong);
        match sangong {
            1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("大安"),
            2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
            3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
            4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
            5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
            0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
            _ => println!("三宫什么鬼?"),
        }
        //画一个界限线
        println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
        let ten_millis = time::Duration::from_secs(3);
        thread::sleep(ten_millis);
        //根据落宫起课
        display_info(n, sangong);
        //小掉桥部分
        diao_qiao(sangong);
        //掌中诀部分
        ask_zzj(sangong);
    }
}

//按时辰起课　根据输入起课
pub fn qi_ke_time() {
    let (one, two, three) = get_input_time();

    println!("依据以上数字起课为:");
    //根据确定落宫数字来定宫
    //定一宫
    let yigong: u32 = one % 6;
    //println!("数字{}落{}宫", one, yigong);
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
    //二宫数字大于六的情况
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
        //如果三宫数字大于６－１　小于６－１例如:(三宫数字 1　９　１)
        if three > 6 {
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
            //画一个界限线
            println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
            let ten_millis = time::Duration::from_secs(3);
            thread::sleep(ten_millis);

            //根据落宫时辰起课 这里改为失物诀内容
            ask_swj(sangong);
            //小掉桥部分
            diao_qiao(sangong);
            //掌中诀部分
            ask_zzj(sangong);
        } else if three <= 6 {
            let sangong: u32 = ergong + three - 1;
            //println!("数字{}落{}宫", three, sangong);
            match sangong {
                1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("大安"),
                2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
                3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
                4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
                5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
                0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
                _ => println!("三宫什么鬼?"),
            }
            //画一个界限线
            println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
            let ten_millis = time::Duration::from_secs(3);
            thread::sleep(ten_millis);

            //根据落宫时辰起课 这里改为失物诀内容
            ask_swj(sangong);
            //小掉桥部分
            diao_qiao(sangong);
            //掌中诀部分
            ask_zzj(sangong);
        }
    //二宫起课数字小于等于６的情况
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
        //println!("数字{}落{}宫", three, sangong);
        match sangong {
            1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("大安"),
            2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
            3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
            4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
            5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
            0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
            _ => println!("三宫什么鬼?"),
        }
        //画一个界限线
        println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
        let ten_millis = time::Duration::from_secs(3);
        thread::sleep(ten_millis);

        //根据落宫时辰起课 这里改为失物诀内容
        ask_swj(sangong);
        //小掉桥部分
        diao_qiao(sangong);
        //掌中诀部分
        ask_zzj(sangong);
    }
}
