pub type Polynomial = Vec<f64>;

pub fn pol(input1: &String) -> Vec<f64> {
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

pub fn eval(a: &Polynomial, x: f64) -> f64{
    let mut out = 0.0;
    for som in a.iter(){
        out *= x;
        out += som;
    }
    out
}