//马前失物诀　起课与前法略不同　月上起日　日上起时辰
use crate::public::{GongWei, JieChiKou, JieDaAn, JieKongWang, JieLiuLian, JieSuXi, JieXiaoJi};
use std::io;
use std::process::exit;

pub struct ShiWuJue {
    pub(crate) shi_wu: GongWei,
}
//方法 这里省略了十二时辰的细分方法　依然按之前落宫解卦
impl JieDaAn for ShiWuJue {
    fn jie_daan(&self) -> String {
        format!("失物诀-大安: {}", self.shi_wu.da_an)
    }
}
impl JieLiuLian for ShiWuJue {
    fn jie_liulian(&self) -> String {
        format!("失物诀-留连: {}", self.shi_wu.liu_lian)
    }
}
impl JieSuXi for ShiWuJue {
    fn jie_suxi(&self) -> String {
        format!("失物诀-速喜: {}", self.shi_wu.su_xi)
    }
}
impl JieChiKou for ShiWuJue {
    fn jie_chikou(&self) -> String {
        format!("失物诀-赤口: {}", self.shi_wu.chi_kou)
    }
}
impl JieXiaoJi for ShiWuJue {
    fn jie_xiaoji(&self) -> String {
        format!("失物诀-小吉: {}", self.shi_wu.xiao_ji)
    }
}
impl JieKongWang for ShiWuJue {
    fn jie_kongwang(&self) -> String {
        format!("失物诀-空亡: {}", self.shi_wu.kong_wang)
    }
}

pub fn display_shi_wu_jue(j: u32) {
    let shi_wu_j = ShiWuJue {
        shi_wu: GongWei {
            da_an: String::from(
                "\n
                 大安辰戌丑未时 
                 易人老者不差迟 
                 子午卯酉时住陈
                 两个顽童定其真
                 寅申巳亥定何身 
                 阳者中男阴女神
                 ",
            ),
            liu_lian: String::from(
                "\n
                流连辰戌丑未推 
                中男两个高短腿 
                子午老人卯酉十 
                寅申巳亥也好推 
                三个少年跨土堆 
                必有一个犯牢罪",
            ),
            su_xi: String::from(
                "\n
                 速喜辰戌丑未看 
                 贼人可知少年汉 
                 逢午子酉女贼犯 
                 其余中男不须换 ",
            ),
            chi_kou: String::from(
                "\n
                 白虎辰戌丑未神 
                 不推就知是女人 
                 寅申巳亥少年身 
                 子午卯酉群贼真 ",
            ),
            xiao_ji: String::from(
                "\n
                 小吉子午卯酉会 
                 必是老人惯偷鬼 
                 其余可为少年推",
            ),
            kong_wang: String::from(
                "\n
                 空亡辰戌丑昧追 
                 也是少年来犯罪 
                 申未酉午女人位 
                 剩下时辰中男对",
            ),
        },
    };

    //依据第三宫落宫解课
    match j {
        1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("info:{}", shi_wu_j.jie_daan()),
        2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("info:{}", shi_wu_j.jie_liulian()),
        3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("info:{}", shi_wu_j.jie_suxi()),
        4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("info:{}", shi_wu_j.jie_chikou()),
        5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("info:{}", shi_wu_j.jie_xiaoji()),
        0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("{}", shi_wu_j.jie_kongwang()),
        _ => println!("失物诀落宫数字异常无法解析..."),
    }
}

pub(crate) fn ask_swj(j: u32) {
    let mut _swj = j;
    display_shi_wu_jue(_swj);
    println!("输入m查看更多位置　时间信息");

    let mut ask_input = String::new(); //获取用户输入
    match io::stdin().read_line(&mut ask_input) {
        Ok(_n) => println!("{}", ask_input),
        Err(error) => {
            println!("error: {} 现在退出...", error);
        }
    }
    let m = String::from("m"); //对比字符串
    let lower_ask_input = ask_input.to_lowercase(); //转换输入为小写字母
                                                    //println!("l input ={}", lower_ask_input);

    if lower_ask_input.trim() == m {
        //失物诀位置　时辰部分内容
        println!(
            "       　------------------ ------------------ ------------------
        马前寻物法真灵 月上起日时顺行 定住宫中时时应 方可推知去向明 
        先把地支口诀传 再论男女何方转 子丑时辰林中里 寅卯时辰落坡间 
        辰巳定是亲人见 午未室中必出观 申酉溜走顺道前 戌亥时辰沟窝连 
    "
        );
        println!(
            "
        大安东北正东位 流连东南方向会 速喜必是正南回 白虎西南破财回
        小吉方向住西北 空亡正北不返归 不论何论永难回"
        );
        println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
    } else {
        println!("输入错误现在退出...");
        exit(0);
    }
}
