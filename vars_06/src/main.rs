fn main() {
    let price0 = 168.125001;
    let price1:f32 = 6.6;
    let price2:f64 = 168.125;

    println!("price0 is {}", price0);
    println!("price1 and price2 are {}, {}", price1, price2);

    const_demo();

    string_demo();

    demo_judgement_program();

    match_demo();

    loop_demo();
}

fn const_demo(){
    const DISCOUNT:f64=0.8;
    const DISCOUNT2:f64=0.6;

    println!("DISCOUNT: {}", DISCOUNT);
    println!("DISCOUNT2: {}", DISCOUNT2);
}

fn string_demo(){
    let s1 = String::new();
    println!("s1: {}, length is {}", s1, s1.len());

    let s2 = String::from("学习成绩的记录");
    println!("s2 is {}, and length is {}", s2, s2.len());
}

fn demo_judgement_program(){
    let total:f32 = 366.00;
    if total>200.00 && total<500.00{
        println!("打9折,{}",total*0.9)
    }else if total>500.00{
        println!("打8折,{}",total*0.9)
    } else{
        println!("无折扣优惠,{}",total)
    }
    //输出 打9折,329.4
}

fn match_demo(){

/*     
    match variable_expression {
        constant_expr1 => {
           // 语句;
        },
        constant_expr2 => {
           // 语句;
        },
        _ => {
           // 默认
           // 其它语句
        }
     }; 
*/
     
     let code = "10010";
     let choose = match code {
        "10010" => "联通",
        "10086" => "移动",
        _ => "Unknown"
     };
     println!("选择 {}", choose);
     //输出 选择 联通
     
     
     let code = "80010";
     let choose = match code {
        "10010" => "联通",
        "10086" => "移动",
        _ => "Unknown"
     };
     println!("选择 {}", choose);
     //输出 选择 Unknown
     


}

//循环语法

fn loop_demo(){

    println!("\n\nloop_demo ...\n");

    let studyList = vec![
        "《Go语言极简一本通》",
        "Go语言微服务架构核心22讲",
        "从0到Go语言微服务架构师",
    ];
    for name in studyList.iter() {
        match name {
            &"从0到Go语言微服务架构师" => println!("恭喜你进阶到第三阶段-{}!", name),
            _ => println!("学习: {}", name),
        }
    }

    println!("\nstudyList now length is : {}", studyList.len());

    //into_iter()

    let studyList2 = vec![
        "《Go语言极简一本通》",
        "Go语言微服务架构核心22讲",
        "从0到Go语言微服务架构师",
    ];
    for name in studyList2.into_iter() {
        match name {
            "从0到Go语言微服务架构师" => println!("恭喜你进阶到第三阶段-{}!", name),
            _ => println!("学习: {}", name),
        }
    }

    //value borrowed here after move

    //println!("\nstudyList2 now length is : {}", studyList2.len());


}