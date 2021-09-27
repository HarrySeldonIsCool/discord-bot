use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};
use std::f64::consts::{PI, E};
use std::string::String;
use scan_fmt::*;

#[command]
pub async fn sss(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let a = args.single::<f64>()?;
    let b = args.single::<f64>()?;
    let c = args.single::<f64>()?;
    let mut product = [0.0_f64;4];
    product[0] = ((b.powi(2)+c.powi(2)-a.powi(2))/(2.0*b*c)).acos()*180.0/PI;
    product[2] = ((b.powi(2)+a.powi(2)-c.powi(2))/(2.0*b*a)).acos()*180.0/PI;
    product[1] = ((a.powi(2)+c.powi(2)-b.powi(2))/(2.0*a*c)).acos()*180.0/PI;
    product[3] = a*b*(product[2]*PI/180.0).sin()/2.0;

    if ((a+b)<c)||((a+c)<b)||((b+c)<a){
        msg.channel_id.say(&ctx.http, "jestli takový trojúhelník narýsuješ, domluvím ti u Kučky jedničku :-)").await?;
        return Ok(())
    }

    for i in 0..4_usize{
        product[i] = (product[i]*1000000.0).round()/1000000.0;
    }

    msg.channel_id.say(&ctx.http, format!(" \nα = {0}°\nβ = {1}°\nγ = {2}°\nS = {3}", product[0], product[1], product[2], product[3])).await?;
    Ok(())
}

#[command]
pub async fn sus(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let a = args.single::<f64>()?;
    let alpha = PI/180.0*(args.single::<f64>())?;
    if alpha > PI{
        msg.channel_id.say(&ctx.http, "ještě jsem neviděl trojúhelník s úhlem přes 180°, ve škole mi ho potom ukážeš").await?;
        return Ok(())
    }
    let b = args.single::<f64>()?;
    let mut product = [0.0_f64;4];
    product[0] = (a.powi(2)+b.powi(2)-2.0*a*b*(alpha.cos())).sqrt();
    product[1] = ((b.powi(2)+product[0].powi(2)-a.powi(2))/(2.0*product[0]*b)).acos()*180.0/PI;
    product[2] = ((a.powi(2)+product[0].powi(2)-b.powi(2))/(2.0*a*product[0])).acos()*180.0/PI;
    product[3] = a*b*alpha.sin()/2.0;

    for i in 0..4_usize{
        product[i] = (product[i]*1000000.0).round()/1000000.0;
    }

    msg.channel_id.say(&ctx.http, format!(" \nα = {1}°\nb = {2}\nγ = {0}°\nS = {3}", product[2], product[1], product[0], product[3])).await?;
    Ok(())
}

#[command]
pub async fn usu(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let alpha = PI/180.0*(args.single::<f64>())?;
    if alpha > PI{
        msg.channel_id.say(&ctx.http, "ještě jsem neviděl trojúhelník s úhlem přes 180°, ve škole mi ho potom ukážeš").await?;
        return Ok(())
    }
    let a = args.single::<f64>()?;
    let beta = PI/180.0*(args.single::<f64>())?;
    if beta > PI{
        msg.channel_id.say(&ctx.http, "ještě jsem neviděl trojúhelník s úhlem přes 180°, ve škole mi ho potom ukážeš").await?;
        return Ok(())
    }
    let mut product = [0.0_f64;4];
    product[2] = PI-alpha-beta;
    product[1] = a/(product[2].sin())*(alpha.sin());
    product[0] = a/(product[2].sin())*(beta.sin());
    product[2] *= 180.0/PI;
    product[3] = a*product[1]*beta.sin()/2.0;

    for i in 0..4_usize{
        product[i] = (product[i]*1000000.0).round()/1000000.0;
    }

    msg.channel_id.say(&ctx.http, format!(" \na = {0}\nβ = {1}°\nc = {2}\nS = {3}", product[0], product[2], product[1], product[3])).await?;
    Ok(())
}

#[command]
pub async fn ssu(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let a = args.single::<f64>()?;
    let b = args.single::<f64>()?;
    let alpha = PI/180.0*(args.single::<f64>())?;
    if alpha > PI{
        msg.channel_id.say(&ctx.http, "ještě jsem neviděl trojúhelník s úhlem přes 180°, ve škole mi ho potom ukážeš").await?;
        return Ok(())
    }
    let mut product = [0.0_f64;4];
    product[2] = (b/a*(alpha.sin())).asin();
    product[1] = PI-alpha-product[2];
    product[0] = a/(alpha.sin())*(product[1].sin());
    product[3] = product[0]*b*alpha.sin()/2.0;
    for i in 1..=2_usize{
        product[i] *= 180.0/PI;
    }

    for i in 0..4_usize{
        product[i] = (product[i]*1000000.0).round()/1000000.0;
    }

    msg.channel_id.say(&ctx.http, format!(" \nβ = {2}°\nγ = {1}°\nc = {0}\nS = {3}", product[0], product[1], product[2], product[3])).await?;
    Ok(())
}

#[command]
pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, format!("jsou zde 4 základní příkazy, sss, sus, usu a ssu (Ssu). všechny příjimají 3 argumenty. zapisují se touto formou\n
    *?sss a b c\n
    *?sus a β c\n
    *?usu α b γ\n
    *?ssu a b α\n
    vedle toho je tu ještě příkaz *?right, který přijme dva argumenty označující jména zadaných hodnot a dva na dané hodnoty, tedy například:
    *?right a c 3 5
    *?right alpha b 56.3 12
    hlavně pozor na desetinnou tečku pro všechny příkazy a anglický spelling příkazů.\n
    nyní jsem přidal ještě *?linear, který má dvě formy vstupu, nejlépe to půjde ukázat příkladem\n
    *?linear [-0.75, 0.5], [-1, 1]\n
    *?linear f: y = 5x - 0.5\n
    mimochodem, můj github je https://github.com/HarrySeldonIsCool/discord-bot")).await?;
    Ok(())
}

#[command]
pub async fn right(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let name1 = args.single_quoted::<String>()?;
    let name2 = args.single_quoted::<String>()?;
    let mut values = [0.0_f64;2];
    values[0] = args.single::<f64>()?;
    values[1] = args.single::<f64>()?;
    match (name1.as_str(), name2.as_str()){
        ("a", "b") => {msg.channel_id.say(&ctx.http, format!("c = {0}\nα = {1}°\nβ = {2}°",
            (values[0].powi(2)+values[1].powi(2)).sqrt(),
            (values[0]/values[1]).atan()*180.0/PI,
            (values[1]/values[0]).atan()*180.0/PI)).await?;()},
        ("b", "a") => {msg.channel_id.say(&ctx.http, format!("c = {0}\nα = {2}°\nβ = {1}°",
            (values[0].powi(2)+values[1].powi(2)).sqrt(),
            (values[0]/values[1]).atan()*180.0/PI,
            (values[1]/values[0]).atan()*180.0/PI)).await?;()},
        ("a", "c") => {msg.channel_id.say(&ctx.http, format!("b = {0}\nα = {1}°\nβ = {2}°",
            (-values[0].powi(2)+values[1].powi(2)).sqrt(),
            (values[0]/values[1]).asin()*180.0/PI,
            (values[0]/values[1]).acos()*180.0/PI)).await?;()},
        ("c", "a") => {msg.channel_id.say(&ctx.http, format!("b = {0}\nα = {1}°\nβ = {2}°",
            (values[0].powi(2)-values[1].powi(2)).sqrt(),
            (values[1]/values[0]).asin()*180.0/PI,
            (values[1]/values[0]).acos()*180.0/PI)).await?;()},
        ("b", "c") => {msg.channel_id.say(&ctx.http, format!("a = {0}\nα = {2}°\nβ = {1}°",
            (-values[0].powi(2)+values[1].powi(2)).sqrt(),
            (values[0]/values[1]).asin()*180.0/PI,
            (values[0]/values[1]).acos()*180.0/PI)).await?;()},
        ("c", "b") => {msg.channel_id.say(&ctx.http, format!("a = {0}\nα = {2}°\nβ = {1}°",
            (values[0].powi(2)-values[1].powi(2)).sqrt(),
            (values[1]/values[0]).asin()*180.0/PI,
            (values[1]/values[0]).acos()*180.0/PI)).await?;()},
        ("alpha", "a") => {msg.channel_id.say(&ctx.http, format!("b = {1}\nc = {0}\nβ = {2}°",
            values[1]/((values[0]/180.0*PI).sin()),
            values[1]/((values[0]/180.0*PI).tan()),
            90.0-values[0])).await?;()},
        ("a", "alpha") => {msg.channel_id.say(&ctx.http, format!("b = {1}\nc = {0}\nβ = {2}°",
            values[0]/((values[1]/180.0*PI).sin()),
            values[0]/((values[1]/180.0*PI).tan()),
            90.0-values[1])).await?;()},
        ("beta", "b") => {msg.channel_id.say(&ctx.http, format!("a = {1}\nc = {0}\nα = {2}°",
            values[1]/((values[0]/180.0*PI).sin()),
            values[1]/((values[0]/180.0*PI).tan()),
            90.0-values[0])).await?;()},
        ("b", "beta") => {msg.channel_id.say(&ctx.http, format!("a = {1}\nc = {0}\nα = {2}°",
            values[0]/((values[1]/180.0*PI).sin()),
            values[0]/((values[1]/180.0*PI).tan()),
            90.0-values[1])).await?;()},
        ("beta", "a") => {let alpha = 90.0-values[0];
            msg.channel_id.say(&ctx.http, format!("b = {1}\nc = {0}\nα = {2}°",
            values[1]/((alpha/180.0*PI).sin()),
            values[1]/((alpha/180.0*PI).tan()),
            alpha)).await?;()},
        ("a", "beta") => {let alpha = 90.0-values[1];
            msg.channel_id.say(&ctx.http, format!("b = {1}\nc = {0}\nα = {2}°",
            values[0]/((alpha/180.0*PI).sin()),
            values[0]/((alpha/180.0*PI).tan()),
            alpha)).await?;()},
        ("alpha", "b") => {let beta = 90.0-values[0];
            msg.channel_id.say(&ctx.http, format!("a = {1}\nc = {0}\nα = {2}°",
            values[1]/((beta/180.0*PI).sin()),
            values[1]/((beta/180.0*PI).tan()),
            beta)).await?;()},
        ("b", "alpha") => {let beta = 90.0-values[1];
            msg.channel_id.say(&ctx.http, format!("a = {1}\nc = {0}\nα = {2}°",
            values[0]/((beta/180.0*PI).sin()),
            values[0]/((beta/180.0*PI).tan()),
            beta)).await?;()},
        ("c", "beta") => {msg.channel_id.say(&ctx.http, format!("a = {1}\nb = {0}\nα = {2}°",
            values[0]*((values[1]/180.0*PI).sin()),
            values[0]*((values[1]/180.0*PI).cos()),
            90.0-values[1])).await?;()},
        ("beta", "c") => {msg.channel_id.say(&ctx.http, format!("a = {1}\nb = {0}\nα = {2}°",
            values[1]*((values[0]/180.0*PI).sin()),
            values[1]*((values[0]/180.0*PI).cos()),
            90.0-values[0])).await?;()},
        ("alpha", "c") => {msg.channel_id.say(&ctx.http, format!("a = {0}\nb = {1}\nα = {2}°",
            values[1]*((values[0]/180.0*PI).sin()),
            values[1]*((values[0]/180.0*PI).cos()),
            90.0-values[0])).await?;()},
        ("c", "alpha") => {msg.channel_id.say(&ctx.http, format!("a = {0}\nb = {1}\nα = {2}°",
            values[0]*((values[1]/180.0*PI).sin()),
            values[0]*((values[1]/180.0*PI).cos()),
            90.0-values[1])).await?;()},
        _ => {msg.channel_id.say(&ctx.http, format!("wtf, co to sakra je?")).await?;()},
    }
    Ok(())
}

#[command]
pub async fn linear(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let input2 = String::from(args.message());
    let mut input = input2.clone();
    let mut index = 0usize; 
    for (i, x) in input2.char_indices(){
        if x == ' '{
            input.remove(i-index);
            index += 1;
        }
    }
    if let Ok((a, b)) = scan_fmt!(input.as_str(),"f:y={}x+{}", f64, f64) {
        msg.channel_id.say(&ctx.http, format!("Px[{0}, 0], Py[0, {1}]", -b/a, b)).await?;
    }
    else if let Ok((a, b)) = scan_fmt!(input.as_str(),"f:y={}x-{}", f64, f64) {
        msg.channel_id.say(&ctx.http, format!("Px[{0}, 0], Py[0, {1}]", b/a, -b)).await?;
    }
    else if let Ok((a, b, c, d)) = scan_fmt!(input.as_str(),"[{},{}],[{},{}]", f64, f64, f64, f64) {
        let result =
        if b-a*(b-d)/(a-c) >= 0.0{
            format!("f: y = {0}x + {1}", (b-d)/(a-c), b-a*(b-d)/(a-c))
        }
        else{
            format!("f: y = {0}x - {1}", (b-d)/(a-c), -b+a*(b-d)/(a-c))
        };
        msg.channel_id.say(&ctx.http, result).await?;
    }
    else if let Ok((a, b)) = scan_fmt!(input.as_str(),"f(x)={}x+{}", f64, f64) {
        msg.channel_id.say(&ctx.http, format!("Px[{0}, 0], Py[0, {1}]", -b/a, b)).await?;
    }
    else if let Ok((a, b)) = scan_fmt!(input.as_str(),"f(x)={}x-{}", f64, f64) {
        msg.channel_id.say(&ctx.http, format!("Px[{0}, 0], Py[0, {1}]", b/a, -b)).await?;
    }
    else {
        msg.channel_id.say(&ctx.http, format!("jestli je to jen zvláštní uspořádání mezer či co, přidej mi na github pull reguest, jinak si to přepiš do normální podoby")).await?;
    }

    Ok(())
}
fn pol(input1: &String) -> Vec<f64> {
    let mut input2 = input1.clone();
    input2.push_str("x0");
    let mut input = vec![];
    if let Some(i) = input2.get(..1){
        input.push(match i {
            "-" => -1.0,
            _ => 1.0
        })
    }
    let mut buffer = String::new();
    let mut last = (0, false);
    let numbers = String::from("0123456789.");
    for (i, x) in input2.char_indices(){
        if let Some(_) = numbers.find(x){
            buffer.push(x);
            last.1 = false;
        }
        else if x == 'x'{
            match buffer.parse::<f64>(){
                Ok(n) => {let len = input.len();
                        input[len-1] *= n;
                    },
                Err(_) => (),
            }
            buffer.clear();
            last.1 = true;
        }
        else if x == '+'{
            input.push(1.0);
            if last.1 {buffer.push('1');}
            match buffer.parse::<u32>(){
                Ok(n) => {let len = input.len();
                    if last.0-n > 1{
                        for _ in (n+1)..last.0{
                            input.insert(len-2,0.0);
                        }
                    }
                    last.0 = n;
                },
                Err(_) => (),
            }
            buffer.clear();
            last.1 = false;
        }
        else if x == '-'{
            input.push(-1.0);
            if last.1 {buffer.push('1');}
            match buffer.parse::<u32>(){
                Ok(n) => {let len = input.len();
                    if last.0-n > 1{
                        for _ in (n+1)..last.0{
                            input.insert(len-2,0.0);
                        }
                    }
                    last.0 = n;
                },
                Err(_) => (),
            }
            buffer.clear();
            last.1 = false;
        }
    }
    
    match buffer.parse::<u32>(){
        Ok(n) => {let len = input.len();
            if last.0-n > 1{
                for _ in (n+1)..last.0{
                    input.insert(len-1,0.0);
                }
            }
            last.0 = n;
        },
        Err(_) => (),
    }
    input
}

#[command]
pub async fn base(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let from = args.single::<u32>()?;
    let to = args.single::<u32>()?;
    let what = args.single_quoted::<String>()?;
    let mut base = 0;
    let mut negative = false;

    for (i, x) in what.char_indices(){
        base += match x{
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'a' => 10,
            'b' => 11,
            'c' => 12,
            'd' => 13,
            'e' => 14,
            'f' => 15,
            '-' => {negative = true;0},
            _ => {msg.channel_id.say(&ctx.http, format!("wtf, to neumim!!!>:(")).await?;0}
        };
        base *= from;
    }

    base /= from;

    msg.channel_id.say(&ctx.http, format!("base 10: {}", base)).await?;

    let mut out = String::new();

    for i in 1..((base as f32).log(to as f32).floor() as u32 + 2){
        let x = base % to;
        out.insert_str(0,&format!("{:X}",x));
        base -= x;
        base /= to;
    }

    if negative {out.insert(0,'-');}

    msg.channel_id.say(&ctx.http, format!("base {}: {}", to, out)).await?;

    Ok(())
}

#[derive(Eq,PartialEq,Debug)]
enum Op{//funkce
    PlusOrNone,
    Minus,
    Dot,
    Div,
    Exp,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Cot,
    Asin,
    Acos,
    Atan,
    Acot,
    Col,
    Log,
    Log2,
    Log10,
    Ln,
    Sinh,
    Cosh,
    Tanh,
    Coth,
    Asinh,
    Acosh,
    Atanh,
    Acoth,
    Root,
    Mod,
    Eul,
    Pi,
    Sum,
    List,
    ListIt,
    Print,
    Bcd,
    Scm,
    Pol,
}

#[derive(Debug, Clone)]
enum Arg{
    Number(f64),
    Stri(String),
    Polynomial(Vec<f64>),
}

impl Arg{
    fn string(&self) -> String{
        match self{
            Stri(some) => some.to_string(),
            _ => panic!(),
        }
    }
    fn number(&self) -> f64{
        match self{
            Number(some) => *some,
            _ => panic!(),
        }
    }
    fn polynomial(&self) -> Vec<f64>{
        match self{
            Polynomial(some) => some.to_vec(),
            _ => panic!(),
        }
    }
}

use self::Arg::*;

use self::Op::*;
impl Op{
fn op(&self, args: &Vec<Arg>, prt: &mut Vec<Vec<Arg>>) -> Vec<Arg>{
    match self{//def. funkci
        PlusOrNone => if args.len() == 2{vec![Number(args[0].number()+args[1].number())]} else {args.to_vec()},
        Minus => vec![Number(args[0].number()-args[1].number())],
        Dot => vec![Number(args[0].number()*args[1].number())],
        Div => vec![Number(args[0].number()/args[1].number())],
        Exp => vec![Number(args[0].number().powf(args[1].number()))],
        Sqrt => vec![Number(args[0].number().sqrt())],
        Sin => vec![Number((args[0].number()/180.0*PI).sin())],
        Cos => vec![Number((args[0].number()/180.0*PI).cos())],
        Tan => vec![Number((args[0].number()/180.0*PI).tan())],
        Cot => vec![Number(1.0/(args[0].number()/180.0*PI).tan())],
        Asin => vec![Number(args[0].number().asin()*180.0/PI)],
        Acos => vec![Number(args[0].number().acos()*180.0/PI)],
        Atan => vec![Number(args[0].number().atan()*180.0/PI)],
        Acot => vec![Number((1.0/args[0].number()).atan()*180.0/PI)],
        Col => args.to_vec(),
        Log => vec![Number(args[0].number().log(args[1].number()))],
        Log2 => vec![Number(args[0].number().log2())],
        Log10 => vec![Number(args[0].number().log10())],
        Ln => vec![Number(args[0].number().ln())],
        Sinh => vec![Number((args[0].number()/180.0*PI).sinh())],
        Cosh => vec![Number((args[0].number()/180.0*PI).cosh())],
        Tanh => vec![Number((args[0].number()/180.0*PI).tanh())],
        Coth => vec![Number(1.0/(args[0].number()/180.0*PI).tanh())],
        Asinh => vec![Number(args[0].number().asinh()*180.0/PI)],
        Acosh => vec![Number(args[0].number().acosh()*180.0/PI)],
        Atanh => vec![Number(args[0].number().atanh()*180.0/PI)],
        Acoth => vec![Number((1.0/args[0].number()).atanh()*180.0/PI)],
        Root => vec![Number(args[0].number().powf(1.0/args[1].number()))],
        Mod => vec![Number((args[0].number() as u32 % args[1].number() as u32) as f64)],
        Eul => vec![Number(E)],
        Pi => vec![Number(PI)],
        Sum => {
            let mut sum = 0.0;
            for arg in args.iter(){
                sum += arg.number();
            }
            vec![Number(sum)]
        },
        List => {
            if args[1].number().floor() == args[1].number() && args[0].number().floor() == args[0].number() && args[0].number() < args[1].number(){ 
            let mut list = vec![];
            for number in (args[0].number() as i32)..=(args[1].number() as i32){
                list.push(Number(number as f64));
            }
            list
            }
            else{
                args.to_vec()
            }
        },
        ListIt => {
            let mut number = args[0].number();
            let len = args.len();
            let mut list = vec![];
            while number <= args[len-1].number(){
                list.push(Number(number));
                number += args[len-2].number();
            }
            list
        },
        Print => {
            prt.push(args.clone());
            args.to_vec()
        },
        Bcd => vec![Number(nad(args[0].number() as u32, args[1].number() as u32).1 as f64)],
        Scm => vec![Number(nad(args[0].number() as u32, args[1].number() as u32).0 as f64)],
        Pol => vec![Polynomial(pol(&args[0].string()))],
    }
}
}

#[derive(Debug)]
struct OpAndNumb(Op, Vec<Arg>);

impl OpAndNumb{
    fn default() -> OpAndNumb{
        OpAndNumb(PlusOrNone, vec![])
    }
    fn op(&mut self, prt: &mut Vec<Vec<Arg>>){
        let ar = &self.1;
        self.1 = self.0.op(ar, prt);
    }
}

fn nad(a0: u32, b0: u32) -> (u32, u32){
    let mut a = a0;
    let mut b = b0;
    let c = a*b;
    let mut d;
    while b != 0 {
        d = a % b;
        a = b;
        b = d;
    }
    (c/a, a)
}


#[command]
pub async fn calc(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut input2 = String::from(args.message());
    input2.pop();
    input2.remove(0);
    input2.insert_str(0, "(");
    input2.push_str(")");
    let input = input2.clone();
    let mut shift = 0;
    let mut fun = String::new();
    let chars = String::from("qwertyuiopasdfghjklzxcvbnm_");
    let mut prt = vec![];
    let mut is_str = false;

    for (i0, x) in input.char_indices(){
        if is_str{
            if x == '\"'{
                is_str = false;
            }
        }
        else{let i = i0 + shift;
        if let Some(_) = chars.find(x) {
            fun.push(x);
            match fun.as_str(){
                "e" => {input2.insert_str(i+1, "(");input2.insert_str(i+2,")");shift += 2;},
                "pi" => {input2.insert_str(i+1, "(");input2.insert_str(i+2,")");shift += 2;},
                _ => (),
            }          
        }
        else{match x{
            '+' => {input2.insert_str(i, ")))))");input2.insert_str(i+6,"(((((");shift += 10;},
            '-' => {input2.insert_str(i, ")))))");input2.insert_str(i+6,"(((((");shift += 10;},
            '*' => {input2.insert_str(i, ")))");input2.insert_str(i+4,"(((");shift += 6;},
            '/' => {input2.insert_str(i, ")))");input2.insert_str(i+4,"(((");shift += 6;},
            '^' => {input2.insert_str(i, "))");input2.insert_str(i+3,"((");shift += 4;},
            '%' => {input2.insert_str(i, "))))");input2.insert_str(i+5,"((((");shift += 8;},
            ')' => {input2.insert_str(i, "))))))");shift += 6;},
            '(' => {input2.insert_str(i, "((((((");shift += 6;},
            ',' => {input2.insert_str(i, "))))))");input2.insert_str(i+7,"((((((");shift += 12;},
            ':' => {input2.insert_str(i, ")");input2.insert_str(i+2,"(");shift += 2;},
            '\"' => {is_str = true;},
            _ => (),
        }
        fun.clear();
        }}
    }
    //msg.channel_id.say(&ctx.http, format!("`{}`", input2)).await?;

    is_str = false;
    let mut buffer0 = vec![OpAndNumb::default()];
    let mut buffer1 = String::new();
    let numbers = String::from("0123456789.");
    let mut was_str = false;

    for (i, x) in input2.char_indices(){
        if is_str{
            if x == '\"'{
                is_str = false;
                was_str = true;
            }
            else{
                buffer1.push(x);
            }
        }
        else{let len = buffer0.len();
        match fun.as_str(){ //jmena funkci
            "sqrt" => buffer0[len-1].0 = Sqrt,
            "sin" => buffer0[len-1].0 = Sin,
            "cos" => buffer0[len-1].0 = Cos,
            "tan" => buffer0[len-1].0 = Tan,
            "cot" => buffer0[len-1].0 = Cot,
            "asin" => buffer0[len-1].0 = Asin,
            "acos" => buffer0[len-1].0 = Acos,
            "atan" => buffer0[len-1].0 = Atan,
            "acot" => buffer0[len-1].0 = Acot,
            "log" => buffer0[len-1].0 = Log,
            "log2" => {buffer0[len-1].0 = Log2;buffer1.clear();},
            "log10" => {buffer0[len-1].0 = Log10;buffer1.clear();},
            "ln" => buffer0[len-1].0 = Ln,
            "sinh" => buffer0[len-1].0 = Sinh,
            "cosh" => buffer0[len-1].0 = Cosh,
            "tanh" => buffer0[len-1].0 = Tanh,
            "coth" => buffer0[len-1].0 = Coth,
            "asinh" => buffer0[len-1].0 = Asinh,
            "acosh" => buffer0[len-1].0 = Acosh,
            "atanh" => buffer0[len-1].0 = Atanh,
            "acoth" => buffer0[len-1].0 = Acoth,
            "rt" => buffer0[len-1].0 = Root,
            "pi" => buffer0[len-1].0 = Pi,
            "e" => buffer0[len-1].0 = Eul,
            "sum" => buffer0[len-1].0 = Sum,
            "print" => buffer0[len-1].0 = Print,
            "bcd" => buffer0[len-1].0 = Bcd,
            "scm" => buffer0[len-1].0 = Scm,
            "pol" => buffer0[len-1].0 = Pol,
            _ => ()
        }
        if let Some(_) = numbers.find(x){
            buffer1.push(x);
            fun.push(x);
        }
        else if let Some(_) = chars.find(x){
            fun.push(x);
        }
        else {match x{
            '(' => {buffer0.push(OpAndNumb::default());},
            ')' => {
                match buffer1.parse::<f64>(){
                    Ok(n) => {if was_str{buffer0[len-1].1.push(Stri(buffer1.clone()))}
                              else{buffer0[len-1].1.push(Number(n))}
                              was_str = false;},
                    Err(_) => if !buffer1.is_empty(){buffer0[len-1].1.push(Stri(buffer1.clone()))},
                }
                let mut last = buffer0.pop().unwrap().1;
                buffer0[len-2].1.append(&mut last);
                buffer0[len-2].op(&mut prt);
                buffer1.clear();
            }
            '-' => buffer0[len-1].0 = Minus,
            '*' => buffer0[len-1].0 = Dot,
            '/' => buffer0[len-1].0 = Div,
            '^' => buffer0[len-1].0 = Exp,
            ',' => buffer0[len-1].0 = Col,
            '%' => buffer0[len-1].0 = Mod,
            ':' =>  if buffer0[len-1].0 == List{
                        buffer0[len-1].0 = ListIt;
                    }
                    else{
                        buffer0[len-1].0 = List;
                    },
            '\"' => is_str = true,
            _ => (),
        }
        fun.clear();}}
        //msg.channel_id.say(&ctx.http, format!("`{:?}`,{}", buffer0, fun)).await?;
    }

    for x in prt{
        msg.channel_id.say(&ctx.http, format!("`{:?}`", x)).await?;
    }

    msg.channel_id.say(&ctx.http, format!("`{:?}`", buffer0[0].1)).await?;

    Ok(())
}