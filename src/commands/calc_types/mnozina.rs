use std::ops::*;
use std::cmp::{Ordering, min, max};

#[derive(PartialEq, Clone, Debug)]
pub struct Mnozina(Vec<MnozinaC>);

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum MnozinaC{
    Single(f64),
    Interval(IntervalS),
}

use MnozinaC::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct IntervalS{
    domain: Domain,
    up: NumbABr,
    down: NumbABr,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct NumbABr(f64, bool);

impl PartialOrd for NumbABr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Index<usize> for Mnozina{
    type Output = MnozinaC;
    fn index(&self, index: usize) -> &Self::Output{
        &self.0[index]
    }
}

impl IndexMut<usize> for Mnozina{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output{
        &mut self.0[index]
    }
}

#[allow(dead_code)]
impl NumbABr{
    fn max(self, b: NumbABr) -> NumbABr{
        if b.0.max(self.0) == b.0{
            b
        }
        else{
            self
        }
    }

    fn min(self, b: NumbABr) -> NumbABr{
        if b.0.min(self.0) == b.0{
            b
        }
        else{
            self
        }
    }
}

#[derive(PartialEq, Copy, Clone, Eq, Debug)]
pub enum Domain{
    R,
    Q,
    Z,
}

impl Domain{
    fn dom(a: f64) -> Self{
        if a.ceil() == a{
            Z
        }
        else{
            R
        }
    }
}

use Domain::*;

impl PartialOrd for Domain{
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Domain{
    fn cmp(&self, rhs: &Self) -> Ordering {
        match (self, rhs){
            (R, R) => Ordering::Equal,
            (Q, Q) => Ordering::Equal,
            (Z, Z) => Ordering::Equal,
            (R, Q) => Ordering::Less,
            (R, Z) => Ordering::Less,
            (Q, Z) => Ordering::Less,
            (Z, Q) => Ordering::Greater,
            (Z, R) => Ordering::Greater,
            (Q, R) => Ordering::Greater,
        }
    }
}

impl BitOr for Mnozina{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output{
        let mut out = vec![];

        let mut iterable = self.0.clone();
        iterable.append(&mut rhs.0.clone());

        for x in iterable.iter(){
            match x{
                Single(a) => {
                    if let Some(Single(b)) = out.iter_mut().find(|som| if Single(*a) == **som {true} else {false}){}
                    else{
                        out.push(Single(*a));
                    }
                },
                Interval(a) => {
                    if let Some(Interval(b)) = out.iter_mut().find(|&&mut som| match som{
                                Interval(b) => (b.down.max(a.down) < b.up.min(a.up) || (
                                    b.down.max(a.down).0 == b.up.min(a.up).0 && (
                                    b.down.max(a.down).1 || b.up.min(a.up).1))) && 
                                    a.domain == b.domain,
                                _ => false,
                            }){
                        *b = IntervalS{
                            domain: a.domain,
                            up: b.up.max(a.up),
                            down: b.down.min(a.down),
                        }
                    }
                    else{
                        out.push(Interval(*a));
                    }
                },
            }
        }

        Mnozina(out)
    }
}

impl BitAnd for Mnozina{
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output{
        let mut out = vec![];

        let iterable = self.0.clone();
        //iterable.append(&mut rhs.0.clone());

        for x in iterable.iter(){
            match x{
                Single(a) => {
                    if let Some(_) = rhs.0.iter().find(|som| Single(*a) == **som){
                        out.push(Single(*a));
                    }
                },
                Interval(a) => {
                    if let Some(Interval(b)) = rhs.0.iter().find(|som| match **som{
                                Single(_) => false,
                                Interval(b) => b.down.max(a.down) < b.up.min(a.up),
                            }){
                        out.push(Interval(IntervalS{
                            domain: min(a.domain, b.domain),
                            up: b.up.min(a.up),
                            down: b.down.max(a.down),
                        }));
                    }
                    else if let Some(Interval(b)) = rhs.0.iter().find(|som| match **som{
                                Single(_) => false,
                                Interval(b) => b.down.max(a.down).0 == b.up.min(a.up).0 && b.down.max(a.down).1 && b.up.min(a.up).1,
                            }){
                        out.push(Single(b.down.max(a.down).0));
                    }
                },
            }
        }

        Mnozina(out)
    }
}

impl Sub for Mnozina{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output{
        let mut out = vec![];

        let iterable = self.0.clone();
        //iterable.append(&mut rhs.0.clone());

        for x in iterable.iter(){
            match x{
                Single(a) => {
                    if let Some(_) = rhs.0.iter().find(|som| Single(*a) == **som){}
                    else{
                        out.push(Single(*a));
                    }
                },
                Interval(a) => {
                    if let Some(Interval(b)) = rhs.0.iter().find(|som| match **som{
                                Single(_) => false,
                                Interval(b) => b.down.max(a.down) < b.up.min(a.up),
                            }){
                        if a.domain != b.domain{
                            panic!("When substracting, you need the same domains in all the parts");
                        }
                        if a.up > b.up{
                            out.push(Interval(IntervalS{
                                domain: a.domain,
                                up: a.up,
                                down: b.up,
                            }));
                        }
                        if a.down < b.down{
                            out.push(Interval(IntervalS{
                                domain: a.domain,
                                up: b.down,
                                down: a.down,
                            }));
                        }
                        if a.down.0 == b.down.0 && !b.down.1 && a.down.1{
                            out.push(Single(a.down.0));
                        }
                        if a.up.0 == b.up.0 && !b.up.1 && a.up.1{
                            out.push(Single(a.up.0));
                        }
                    }
                    else if let Some(Interval(b)) = rhs.0.iter().find(|som| match **som{
                                Single(_) => false,
                                Interval(b) => b.down.max(a.down).0 == b.up.min(a.up).0 && b.down.max(a.down).1 && b.up.min(a.up).1,
                            }){
                        if a.domain != b.domain{
                            panic!("When substracting, you need the same domains in all the parts");
                        }
                        if a.up.0 == b.down.0{
                            out.push(Interval(IntervalS{
                                domain: a.domain,
                                up: b.down,
                                down: a.down,
                            }));
                        }
                        if a.down.0 == b.up.0{
                            out.push(Interval(IntervalS{
                                domain: a.domain,
                                up: a.up,
                                down: b.up,
                            }));
                        }
                    }
                    else if let Some(Single(b)) = rhs.0.iter().find(|som| match **som{
                            Single(b) => b > a.down.0 && b < a.up.0,
                            Interval(_) => false,
                        }){
                        if Domain::dom(*b) >= a.domain{
                            panic!("When substracting, you need the same domains in all the parts");
                        }
                        out.push(Interval(IntervalS{
                            domain: a.domain,
                            up: NumbABr(*b, false),
                            down: a.down,
                        }));
                        out.push(Interval(IntervalS{
                            domain: a.domain,
                            up: a.up,
                            down: NumbABr(*b, false),
                        }));
                    }
                    else if let Some(Single(b)) = rhs.0.iter().find(|som| match **som{
                            Single(b) => (b == a.down.0 && a.down.1) || (b == a.up.0 && a.up.1) ,
                            Interval(_) => false,
                        }){
                        if Domain::dom(*b) >= a.domain{
                            panic!("When substracting, you need the same domains in all the parts");
                        }
                        if *b == a.down.0{
                            out.push(Interval(IntervalS{
                                domain: a.domain,
                                up: a.up,
                                down: NumbABr(*b, false),
                            }));
                        }
                        if *b == a.up.0{
                            out.push(Interval(IntervalS{
                                domain: a.domain,
                                up: NumbABr(*b, false),
                                down: a.down,
                            }));
                        }
                    }
                    else{
                        out.push(Interval(*a));
                    }
                },
            }
        }

        Mnozina(out)
    }
}

impl Not for Mnozina{
    type Output = Self;
    fn not(self) -> Self::Output{
        Mnozina(vec![Interval(IntervalS{
            domain: {if let Some(Interval(a)) = self.0.iter().find(|som| match **som{Interval(_) => true, Single(_) => false}){
                a.domain
            }
            else if let Some(Single(a)) = self.0.iter().find(|som| match **som{Interval(_) => false, Single(_) => true}){
                Domain::dom(*a)
            }
            else{
                R
            }},
            up: NumbABr(std::f64::INFINITY, false),
            down: NumbABr(std::f64::NEG_INFINITY, false),
        })]) - self
    }
}

#[allow(dead_code)]
impl Mnozina{
    fn r() -> Self{
        Mnozina(vec![Interval(IntervalS{
            domain: R,
            up: NumbABr(std::f64::INFINITY, false),
            down: NumbABr(std::f64::NEG_INFINITY, false),
        })])
    }
    fn q() -> Self{
        Mnozina(vec![Interval(IntervalS{
            domain: Q,
            up: NumbABr(std::f64::INFINITY, false),
            down: NumbABr(std::f64::NEG_INFINITY, false),
        })])
    }
    fn z() -> Self{
        Mnozina(vec![Interval(IntervalS{
            domain: Z,
            up: NumbABr(std::f64::INFINITY, false),
            down: NumbABr(std::f64::NEG_INFINITY, false),
        })])
    }
    fn n() -> Self{
        Mnozina(vec![Interval(IntervalS{
            domain: Z,
            up: NumbABr(std::f64::INFINITY, false),
            down: NumbABr(1.0, true),
        })])
    }
    fn empty() -> Self{
        Mnozina(vec![])
    }
    fn simplify(&self) -> Self{
        (*self).clone() | Self::empty()
    }
    pub fn interval(from: (f64, bool), to: (f64, bool)) -> Mnozina{
        Mnozina(vec![MnozinaC::Interval(IntervalS{
            domain: R,
            up: NumbABr(to.0, to.1),
            down: NumbABr(from.0, from.1)
        })])
    }
}