use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};
use std::f64::consts::PI;
use std::string::String;

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
    let a = args.single::<f64>()?;
    let beta = PI/180.0*(args.single::<f64>())?;
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
    hlavně pozor na desetinnou tečku pro všechny příkazy a anglický spelling příkazů.")).await?;
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
        ("a", "b") => {msg.channel_id.say(&ctx.http, format!("c = {0}\nα = {1}\nβ = {2}",
            (values[0].powi(2)+values[1].powi(2)).sqrt(),
            (values[0]/values[1]).atan()*180.0/PI,
            (values[1]/values[0]).atan()*180.0/PI)).await?;()},
        ("b", "a") => {msg.channel_id.say(&ctx.http, format!("c = {0}\nα = {2}\nβ = {1}",
            (values[0].powi(2)+values[1].powi(2)).sqrt(),
            (values[0]/values[1]).atan()*180.0/PI,
            (values[1]/values[0]).atan()*180.0/PI)).await?;()},
        ("a", "c") => {msg.channel_id.say(&ctx.http, format!("b = {0}\nα = {1}\nβ = {2}",
            (-values[0].powi(2)+values[1].powi(2)).sqrt(),
            (values[0]/values[1]).asin()*180.0/PI,
            (values[0]/values[1]).acos()*180.0/PI)).await?;()},
        ("c", "a") => {msg.channel_id.say(&ctx.http, format!("b = {0}\nα = {1}\nβ = {2}",
            (values[0].powi(2)-values[1].powi(2)).sqrt(),
            (values[1]/values[0]).asin()*180.0/PI,
            (values[1]/values[0]).acos()*180.0/PI)).await?;()},
        ("b", "c") => {msg.channel_id.say(&ctx.http, format!("a = {0}\nα = {2}\nβ = {1}",
            (-values[0].powi(2)+values[1].powi(2)).sqrt(),
            (values[0]/values[1]).asin()*180.0/PI,
            (values[0]/values[1]).acos()*180.0/PI)).await?;()},
        ("c", "b") => {msg.channel_id.say(&ctx.http, format!("a = {0}\nα = {2}\nβ = {1}",
            (values[0].powi(2)-values[1].powi(2)).sqrt(),
            (values[1]/values[0]).asin()*180.0/PI,
            (values[1]/values[0]).acos()*180.0/PI)).await?;()},
        ("alpha", "a") => {msg.channel_id.say(&ctx.http, format!("b = {1}\nc = {0}\nβ = {2}",
            values[1]/((values[0]/180.0*PI).sin()),
            values[1]/((values[0]/180.0*PI).tan()),
            90.0-values[0])).await?;()},
        ("a", "alpha") => {msg.channel_id.say(&ctx.http, format!("b = {1}\nc = {0}\nβ = {2}",
            values[0]/((values[1]/180.0*PI).sin()),
            values[0]/((values[1]/180.0*PI).tan()),
            90.0-values[1])).await?;()},
        ("beta", "b") => {msg.channel_id.say(&ctx.http, format!("a = {1}\nc = {0}\nα = {2}",
            values[1]/((values[0]/180.0*PI).sin()),
            values[1]/((values[0]/180.0*PI).tan()),
            90.0-values[0])).await?;()},
        ("b", "beta") => {msg.channel_id.say(&ctx.http, format!("a = {1}\nc = {0}\nα = {2}",
            values[0]/((values[1]/180.0*PI).sin()),
            values[0]/((values[1]/180.0*PI).tan()),
            90.0-values[1])).await?;()},
        ("beta", "a") => {let alpha = 90.0-values[0];
            msg.channel_id.say(&ctx.http, format!("b = {1}\nc = {0}\nα = {2}",
            values[1]/((alpha/180.0*PI).sin()),
            values[1]/((alpha/180.0*PI).tan()),
            alpha)).await?;()},
        ("a", "beta") => {let alpha = 90.0-values[1];
            msg.channel_id.say(&ctx.http, format!("b = {1}\nc = {0}\nα = {2}",
            values[0]/((alpha/180.0*PI).sin()),
            values[0]/((alpha/180.0*PI).tan()),
            alpha)).await?;()},
        ("alpha", "b") => {let beta = 90.0-values[0];
            msg.channel_id.say(&ctx.http, format!("a = {1}\nc = {0}\nα = {2}",
            values[1]/((beta/180.0*PI).sin()),
            values[1]/((beta/180.0*PI).tan()),
            beta)).await?;()},
        ("b", "alpha") => {let beta = 90.0-values[1];
            msg.channel_id.say(&ctx.http, format!("a = {1}\nc = {0}\nα = {2}",
            values[0]/((beta/180.0*PI).sin()),
            values[0]/((beta/180.0*PI).tan()),
            beta)).await?;()},
        ("c", "beta") => {msg.channel_id.say(&ctx.http, format!("a = {1}\nb = {0}\nα = {2}",
            values[0]*((values[1]/180.0*PI).sin()),
            values[0]*((values[1]/180.0*PI).cos()),
            90.0-values[1])).await?;()},
        ("beta", "c") => {msg.channel_id.say(&ctx.http, format!("a = {1}\nb = {0}\nα = {2}",
            values[1]*((values[0]/180.0*PI).sin()),
            values[1]*((values[0]/180.0*PI).cos()),
            90.0-values[0])).await?;()},
        ("alpha", "c") => {msg.channel_id.say(&ctx.http, format!("a = {0}\nb = {1}\nα = {2}",
            values[1]*((values[0]/180.0*PI).sin()),
            values[1]*((values[0]/180.0*PI).cos()),
            90.0-values[0])).await?;()},
        ("c", "alpha") => {msg.channel_id.say(&ctx.http, format!("a = {0}\nb = {1}\nα = {2}",
            values[0]*((values[1]/180.0*PI).sin()),
            values[0]*((values[1]/180.0*PI).cos()),
            90.0-values[1])).await?;()},
        _ => {msg.channel_id.say(&ctx.http, format!("wtf, co to sakra je?")).await?;()},
    }
    Ok(())
}