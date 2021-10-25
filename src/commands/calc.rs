use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};
use std::f64::consts::{PI, E};
use std::string::String;
use scan_fmt::*;
use std::ops::*;
use super::calc_types::polynomial;
use super::calc_types::mnozina;

#[derive(Eq,PartialEq,Debug)]
enum Op{//funkce
    Plus,
    Nothing,
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
    SqrtPart,
    //RootPart,
    Index,
    Interval(bool, bool),
}

#[derive(Debug, Clone)]
enum Arg{
    Number(f64),
    Stri(String),
    Polynomial(polynomial::Polynomial),
    Array(Vec<Arg>),
    Mnozina(mnozina::Mnozina),
}

impl Arg{
    fn string(&self) -> String{
        match self{
            Stri(some) => some.to_string(),
            _ => panic!("expected string"),
        }
    }
    fn number(&self) -> f64{
        match self{
            Number(some) => *some,
            _ => panic!("expected number"),
        }
    }
    fn polynomial(&self) -> polynomial::Polynomial{
        match self{
            Polynomial(some) => some.to_vec(),
            _ => panic!("expected polynomial"),
        }
    }
    fn array(&self) -> Vec<Arg>{
        match self{
            Array(some) => some.to_vec(),
            _ => panic!("expected array"),
        }
    }
    fn mnozina(&self) -> mnozina::Mnozina{
        match self{
            Mnozina(some) => some.clone(),
            _ => panic!("expected mnozina"),
        }
    }
}

fn sqrt_partial(input: u32) -> (u32, u32){
    let mut dividers: Vec<u32> = vec![];
    let mut squarable = 1;
    let mut srt = input;
    'big: loop{
        if srt == 1{
            break 'big;
        }
        else{
            'me: for i in 2..=srt{
                if srt % i == 0{
                    srt /= i;
                    if let Some((x, y)) = dividers.iter().enumerate().find(|(a, b)| i == **b){
                        dividers.remove(x);
                        squarable *= i;
                    }
                    else{
                        dividers.push(i);
                    }
                    break 'me;
                }
            }
        }
    }
    (squarable, input/(squarable.pow(2)))
}

use self::Arg::*;

use self::Op::*;
impl Op{
fn op(&self, args: &Vec<Arg>, prt: &mut Vec<Vec<Arg>>) -> Vec<Arg>{
    match self{//def. funkci
        Plus => vec![Number(args[0].number()+args[1].number())],
        Nothing => args.to_vec(),
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
        Pol => vec![Polynomial(polynomial::pol(&args[0].string()))],
        SqrtPart => vec![Number(sqrt_partial(args[0].number() as u32).0 as f64),Number(sqrt_partial(args[0].number() as u32).1 as f64)],
        Index => {
            let len = args.len();
            let last = args[len-1].number() as usize;
            if args.len() == 2{
                match &args[0]{
                    Array(a) => vec![a[last].clone()],
                    _ => vec![args[0].clone()],
                }
            }
            else{
                vec![args[last].clone()]
            }
        },
        Interval(a, b) => vec![Mnozina(mnozina::Mnozina::interval((args[0].number(), *a), (args[1].number(), *b)))],
    }
}
}

#[derive(Debug)]
struct OpAndNumb(Op, Vec<Arg>);

impl OpAndNumb{
    fn default() -> OpAndNumb{
        OpAndNumb(Nothing, vec![])
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
    let mut is_int = false;

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
            ')' => {input2.insert_str(i, ")))))))");shift += 7;},
            '(' => {input2.insert_str(i, "(((((((");shift += 7;},
            '<' => {input2.insert_str(i+1, "(((((((");shift += 7;if fun.as_str() != "interval"{input2.insert_str(i, ")))))))");shift += 7;}else{input2.insert_str(i+1, "(");shift += 1;is_int = true;}},
            '>' => {if is_int{input2.insert_str(i+1, ")");input2.insert_str(i, ")))))))");shift += 8;is_int = false;}else{input2.insert_str(i, ")))))))");input2.insert_str(i+8, "(((((((");shift += 14;}}
            ']' => {input2.insert_str(i, "))))))))");shift += 8;},
            '[' => {input2.insert_str(i+1, "((((((((");shift += 8;},
            ',' => {input2.insert_str(i, "))))))");input2.insert_str(i+7,"((((((");shift += 12;},
            ':' => {input2.insert_str(i, ")");input2.insert_str(i+2,"(");shift += 2;},
            '=' => {}
            '\"' => {is_str = true;},
            _ => (),
        }
        fun.clear();
        }}
        //msg.channel_id.say(&ctx.http, format!("`{}`", input2)).await?;
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
        else{
            let len = buffer0.len();
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
                "part_sqrt" => buffer0[len-1].0 = SqrtPart,
                "interval" => {if let Interval(_, _) = buffer0[len-1].0{}else{buffer0[len-1].0 = Interval(false, false)}},
                _ => ()
            }
            if let Some(_) = numbers.find(x){
                buffer1.push(x);
                fun.push(x);
            }
            else if let Some(_) = chars.find(x){
                fun.push(x);
            }
            else {
                match x{
                    '(' => {buffer0.push(OpAndNumb::default());},
                    ')' => {
                        match buffer1.parse::<f64>(){
                            Ok(n) => {if was_str{buffer0[len-1].1.push(Stri(buffer1.clone()))}
                                      else{buffer0[len-1].1.push(Number(n))}
                                      was_str = false;},
                            Err(_) => if !buffer1.is_empty(){buffer0[len-1].1.push(Stri(buffer1.clone()))},
                        }
                        let mut last = buffer0.pop().unwrap().1;
                        if last.len() > 1 && buffer0[len-2].1.len() > 0{
                            buffer0[len-2].1 = vec![Array(buffer0[len-2].1.clone())];
                            buffer0[len-2].1.push(Array(last));
                        }
                        else{
                            buffer0[len-2].1.append(&mut last);
                        }
                        buffer0[len-2].op(&mut prt);
                        buffer1.clear();
                    }
                    '-' => buffer0[len-1].0 = Minus,
                    '+' => buffer0[len-1].0 = Plus,
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
                    '[' => buffer0[len-1].0 = Index,
                    '<' => {
                        if let Interval(_, b) = buffer0[len-1].0{
                            buffer0[len-1].0 = Interval(true, b);
                        }
                    },
                    '>' => {
                        if let Interval(a, _) = buffer0[len-2].0{
                            buffer0[len-2].0 = Interval(a, true);
                        }
                    }
                    _ => (),
                }
                fun.clear();
            }
        }
        //msg.channel_id.say(&ctx.http, format!("`{:?}`,{}", buffer0, fun)).await?;
    }

    for x in prt{
        msg.channel_id.say(&ctx.http, format!("`{:?}`", x)).await?;
    }

    msg.channel_id.say(&ctx.http, format!("`{:?}`", buffer0[0].1)).await?;

    Ok(())
}