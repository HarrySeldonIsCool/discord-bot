use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};
use std::f64::consts::PI;
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

#[command]
pub async fn base(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let from = args.single::<u32>()?;
    let to = args.single::<u32>()?;
    let what = args.single_quoted::<String>()?;
    let mut base = 0;
    let mut negative = false;

    for (_, x) in what.char_indices(){
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

    for _i in 1..((base as f32).log(to as f32).floor() as u32 + 2){
        let x = base % to;
        out.insert_str(0,&format!("{:X}",x));
        base -= x;
        base /= to;
    }

    if negative {out.insert(0,'-');}

    msg.channel_id.say(&ctx.http, format!("base {}: {}", to, out)).await?;

    Ok(())
}
