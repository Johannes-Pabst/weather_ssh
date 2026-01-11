use std::{
    env, fs, i16, ops::{Div, Mul}
};

use image::{GenericImageView, ImageReader};

pub struct Frame<C>
where
    C: Clone + PartialEq + ToAnsi,
{
    pub width: usize,
    pub height: usize,
    pub texels: Vec<(char, C)>,
}
impl<C> Frame<C>
where
    C: Clone + PartialEq + ToAnsi,
{
    pub fn new(width: usize, height: usize, c: C) -> Frame<C> {
        let texels = vec![(' ', c); width * height];
        Frame {
            width,
            height,
            texels,
        }
    }
    pub fn set_texel(&mut self, x: usize, y: usize, c: (char, C)) -> Result<(), ()> {
        if x < self.width && y < self.height {
            self.texels[x + y * self.width] = c;
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn set_pixel<ST>(&mut self, x: usize, y: usize, r:u8,g:u8,b:u8,st: &ST) -> Result<(), ()> where C: FromRGB<ST> {
        self.set_texel(x, y, C::from_rgb(r,g,b,&st))
    }
    pub fn render(&self) {
        // execute!(std::io::stdout(), MoveTo(0,0)).unwrap();
        let mut last: Option<C> = None;
        for y in 0..self.height {
            // execute!(std::io::stdout(), cursor::MoveTo(0, y as u16)).unwrap();
            for x in 0..self.width {
                let t = &self.texels[x + y * self.width];
                if let Some(l) = &last {
                    if l == &t.1 {
                        print!("{}", t.0);
                        continue;
                    }
                }
                print!("{}{}", t.1.to_ansi(), t.0);
                last = Some(t.1.clone());
            }
        }
    }
    pub fn render_str(&self)->String {
        let mut out=String::new();
        out=format!("{out}\x1b[H");
        let mut last: Option<C> = None;
        for y in 0..self.height {
            out=format!("{out}\r\x1b[{}d",y+1);
            for x in 0..self.width {
                let t = &self.texels[x + y * self.width];
                if let Some(l) = &last {
                    if l == &t.1 {
                        out=format!("{out}{}", t.0);
                        continue;
                    }
                }
                out=format!("{out}{}{}", t.1.to_ansi(), t.0);
                last = Some(t.1.clone());
            }
        }
        out
    }
}
type TerminalData = (Vec<[u8; 3]>, Vec<(char, i32)>, i32);
pub fn read_term_data() -> TerminalData {
    let exe_path=env::current_exe().unwrap().parent().unwrap().to_str().unwrap().to_string();
    println!("{exe_path}");
    let data = fs::read_to_string(format!("{exe_path}/lines.txt"))
        .expect("Unable to read file");
    let mut chars: Vec<(char, i32)> = Vec::new();
    let mut c = data.chars();
    let mut string: String = String::new();
    let mut biggest = 90720;
    while let Some(h) = c.next() {
        if h == '\n' {
            let n = string.chars().next().unwrap();
            // println!(".{exe_path}.", string[n.len_utf8() + 1..string.len()].to_string());
            let h2 = string[n.len_utf8() + 1..string.len()]
                .parse::<i32>()
                .unwrap();
            biggest = if h2 > biggest { h2 } else { biggest };
            chars.push((n, h2));
            string = String::new();
        } else {
            string += h.to_string().as_str();
        }
    }
    let data2 = fs::read_to_string(format!("{exe_path}/colors.txt"))
        .expect("Unable to read file");
    let mut colors: Vec<[u8; 3]> = Vec::new();
    let mut c2 = data2.chars();
    let mut string2: String = String::new();
    while let Some(h) = c2.next() {
        if h == '\n' {
            let h2 = string2.parse::<i32>().unwrap();
            colors.push([
                (h2 >> 16) as u8,
                ((h2 >> 8) & 0xff) as u8,
                (h2 & 0xff) as u8,
            ]);
            string2 = String::new();
        } else {
            string2 += h.to_string().as_str();
        }
    }

    (colors, chars, biggest)
}
impl<C> Frame<C>
where
    C: Clone + PartialEq + ToAnsi,
{
    pub fn put_image<ST>(
        &mut self,
        x: usize,
        y: usize,
        s: Size<usize>,
        imgurl: String,
        st: ST,
    ) -> Result<(), ()>
    where
        C: FromRGB<ST>,
    {
        let image = ImageReader::open(imgurl).unwrap().decode().unwrap();
        let (width, height) = s.to_dimensions((
            (2.5 * image.dimensions().0 as f32) as usize,
            image.dimensions().1 as usize,
        ));
        if x + width as usize <= self.width && y + height as usize <= self.height {
            let mut text: Vec<Vec<([u32; 4], u32)>> =
                vec![vec![([0; 4], 0); height as usize]; width as usize];
            for pix in image.pixels() {
                let x = ((pix.0 as f32) / (image.width() as f32 / width as f32) as f32) as usize;
                let y = ((pix.1 as f32) / (image.height() as f32 / height as f32) as f32) as usize;
                for i in 0..4 {
                    text[x][y].0[i] += pix.2 .0[i] as u32;
                }
                text[x][y].1 += 1;
            }
            for y in 0..height {
                for x in 0..width {
                    self.set_texel(
                        x,
                        y,
                        C::from_rgb(
                            (text[x][y].0[0] / text[x][y].1) as u8,
                            (text[x][y].0[1] / text[x][y].1) as u8,
                            (text[x][y].0[2] / text[x][y].1) as u8,
                            &st,
                        ),
                    )
                    .unwrap();
                }
            }
            Ok(())
        } else {
            Err(())
        }
    }
}
pub trait ToAnsi {
    fn to_ansi(&self) -> String;
}
pub trait FromRGB<ST> {
    fn from_rgb(r: u8, g: u8, b: u8, st: &ST) -> (char, Self);
}
impl ToAnsi for u8 {
    /*
    0bhccchccc
    */
    fn to_ansi(&self) -> String {
        format!(
            "\x1b[{};{}m",
            30 + ((self >> 4) & 0b111) + ((self >> 7) & 0b1) * 60,
            40 + (self & 0b111) + ((self >> 3) & 0b1) * 60
        )
    }
}
impl ToAnsi for u16 {
    /*
    0bhccchccc
    */
    fn to_ansi(&self) -> String {
        format!(
            "\x1b[38;5;{}m\x1b[48;5;{}m",
            self>>8,self&0b11111111
        )
    }
}
impl ToAnsi for (){
    fn to_ansi(&self) -> String {
        String::new()
    }
}
impl FromRGB<TerminalData> for () {
    fn from_rgb(r: u8, g: u8, b: u8, st: &TerminalData) -> (char, Self) {
        let chars = st.1.clone();
        let mut best: char = ' ';
        let mut bs: i32 = i32::MAX;
        let mut max=0;
        for &c in &chars {
            max=max.max(c.1);
        }
        let brightness: i32=(((r as f32)+(g as f32)+(b as f32))/(255.0*3.0)*max as f32) as i32;
        for &c in &chars {
            let score=(brightness-c.1).abs();
            if score < bs {
                (best,bs) = (c.0, score);
            }
        }
        (best,())
    }
}
impl FromRGB<TerminalData> for u8 {
    fn from_rgb(r: u8, g: u8, b: u8, st: &TerminalData) -> (char, Self) {
        let colors = st.0.clone();
        let chars = st.1.clone();
        let mut best: (char, Self) = (' ', 0);
        let mut score: i32 = i32::MAX;
        for &c in &chars {
            let t2 = [r, g, b];
            let mut i1 = 0;
            for &co in &colors {
                let mut i2 = 0;
                for &co2 in &colors {
                    let biggest = 90720;
                    let fgm = c.1 as f32 / biggest as f32;
                    // let fgm = c.1 as f32 / sa as f32 / 3.0 / 256.0;  //92160
                    let bgm = 1.0 - fgm;
                    let mut s = 0;
                    for i in 0..3 {
                        let col = (co[i] as f32 * fgm + co2[i] as f32 * bgm) as u8;
                        let dif = (col as i16 - t2[i] as i16).abs();
                        s += dif;
                    }
                    if (s as i32) < score {
                        score = s as i32;
                        best = (c.0, ((i1 << 4) + i2) as u8);
                    }
                    i2 += 1;
                }
                i1 += 1;
            }
        }
        best
    }
}
impl FromRGB<TerminalData> for u16 {
    fn from_rgb(r: u8, g: u8, b: u8, st: &TerminalData) -> (char, Self) {
        let chars = st.1.clone();
        let c1 = [
            ((r as f32) / 255.0 * 5.0).round() as u8,
            ((g as f32) / 255.0 * 5.0).round() as u8,
            ((b as f32) / 255.0 * 5.0).round() as u8,
        ];
        let mut bch = [0, 0, 0];
        let mut score = i16::MAX;
        for dr in [-1, 1] {
            for dg in [-1, 1] {
                for db in [-1, 1] {
                    let c2 = [
                        ((c1[0] as i16) + dr),
                        ((c1[1] as i16) + dg),
                        ((c1[2] as i16) + db),
                    ];
                    if c2[0] >= 0 && c2[0] < 6 && c2[1] >= 0 && c2[1] < 6 && c2[2] >= 0 && c2[2] < 6
                    {
                        let mut s = 0;
                        let dif = ((r as i16) - ((c2[0] as f32)/5.0*255.0) as i16).abs()
                            + ((g as i16) - ((c2[1] as f32)/5.0*255.0) as i16).abs()
                            + ((b as i16) - ((c2[2] as f32)/5.0*255.0) as i16).abs();
                        s += dif;
                        if s < score {
                            bch = [c2[0], c2[1], c2[2]];
                            score = s;
                        }
                    }
                }
            }
        }
        let mut bc: char = ' ';
        let mut score2=f32::MAX;
        for &c in &chars {
            let fg=(c.1 as f32)/ 90720.0;
            let bg=1.0-fg;
            let dr=((c1[0] as f32) * bg + (bch[0] as f32) * fg - (r as f32)/255.0*5.0).abs();
            let dg=((c1[1] as f32) * bg + (bch[1] as f32) * fg - (g as f32)/255.0*5.0).abs();
            let db=((c1[2] as f32) * bg + (bch[2] as f32) * fg - (b as f32)/255.0*5.0).abs();
            if (dr+dg+db) < score2{
                bc=c.0;
                score2=dr+dg+db;
            }
        }
        (bc,(((bch[0]*36+bch[1]*6+bch[2]+16) as u16)+(((c1[0]*36+c1[1]*6+c1[2]+16) as u16)<<8)))//0bffffffffbbbbbbbb
    }
}
pub enum Size<N>
where
    N: Clone,
{
    Width(N),
    Height(N),
    Both(N, N),
}
impl<N> Size<N>
where
    N: Clone,
    N: Div<Output = N>,
    N: Mul<Output = N>,
{
    fn to_dimensions(&self, d: (N, N)) -> (N, N) {
        match self {
            Size::Width(w) => (w.clone(), w.clone() * d.1 / d.0),
            Size::Height(h) => (h.clone() * d.0 / d.1, h.clone()),
            Size::Both(w, h) => (w.clone(), h.clone()),
        }
    }
}
