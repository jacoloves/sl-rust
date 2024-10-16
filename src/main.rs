mod sl;

use sl::*;
use std::sync::atomic::{AtomicBool, Ordering};

use ncurses::{COLS, ERR, LINES, OK};

static ACCIDENT: AtomicBool = AtomicBool::new(false);
static LOGO: AtomicBool = AtomicBool::new(false);
static FLY: AtomicBool = AtomicBool::new(false);
static C51: AtomicBool = AtomicBool::new(false);

struct Smoke {
    y: i32,
    x: i32,
    ptrn: usize,
    kind: usize,
}

fn my_mvaddstr(y: i32, x: i32, s: &str) -> i32 {
    let mut x = x;
    let mut chars = s.chars();
    while x < 0 {
        if chars.next().is_none() {
            return ERR;
        }
        x += 1;
    }
    OK
}

fn option(args: &str) {
    for c in args.chars() {
        match c {
            'a' => ACCIDENT.store(true, Ordering::Relaxed),
            'F' => FLY.store(true, Ordering::Relaxed),
            'l' => LOGO.store(true, Ordering::Relaxed),
            'c' => C51.store(true, Ordering::Relaxed),
            _ => (),
        }
    }
}

fn add_sl(x: i32) -> i32 {
    let sl = [
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL11, LWHL12, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL21, LWHL22, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL31, LWHL32, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL41, LWHL42, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL51, LWHL52, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL61, LWHL62, DELLN],
    ];
    let coal = [LCOAL1, LCOAL2, LCOAL3, LCOAL4, LCOAL5, LCOAL6, DELLN];
    let car = [LCAR1, LCAR2, LCAR3, LCAR4, LCAR5, LCAR6, DELLN];

    if x < -(LOGOLENGTH as i32) {
        return ERR;
    }
    let mut y = LINES() / 2 - 3;
    let (py1, py2, py3) = if FLY.load(Ordering::Relaxed) {
        y = (x / 6) + LINES() - (COLS() / 6) - LOGOHEIGHT as i32;
        (2, 4, 6)
    } else {
        (0, 0, 0)
    };

    for i in 0..=LOGOHEIGHT {
        my_mvaddstr(
            y + i as i32,
            x,
            sl[((LOGOLENGTH as i32 + x) / 3 % LOGOPATTERNS as i32) as usize][i],
        );
        my_mvaddstr(y + i as i32 + py1, x + 21, coal[i]);
        my_mvaddstr(y + i as i32 + py2, x + 42, car[i]);
        my_mvaddstr(y + i as i32 + py3, x + 63, car[i]);
    }
    if ACCIDENT.load(Ordering::Relaxed) {
        add_man(y + 1, x + 14);
        add_man(y + 1 + py2, x + 45);
        add_man(y + 1 + py2, x + 53);
        add_man(y + 1 + py3, x + 66);
        add_man(y + 1 + py3, x + 74);
    }
    add_smoke(y - 1, x + LOGOFUNNEL as i32);
    OK
}

fn add_man(y: i32, x: i32) {
    let man = [["", "(0)"], ["Help!", "\\0/"]];
    for i in 0..2 {
        my_mvaddstr(
            y + i,
            x,
            man[((LOGOLENGTH as i32 + x) / 12 % 2) as usize][i as usize],
        );
    }
}

fn add_smoke(y: i32, x: i32) {
    const SMOKEPTNS: usize = 16;
    static mut S: Vec<Smoke> = Vec::new();
    static SMOKE: [&[&str]; 2] = [
        &[
            "(   )", "(    )", "(    )", "(   )", "(  )", "(  )", "( )", "( )", "()", "()", "O",
            "O", "O", "O", "O", " ",
        ],
        &[
            "(@@@)", "(@@@@)", "(@@@@)", "(@@@)", "(@@)", "(@@)", "(@)", "(@)", "@@", "@@", "@",
            "@", "@", "@", "@", " ",
        ],
    ];
    static ERASER: [&str; SMOKEPTNS] = [
        "     ", "      ", "      ", "     ", "    ", "    ", "   ", "   ", "  ", "  ", " ", " ",
        " ", " ", " ", " ",
    ];
    static DY: [i32; SMOKEPTNS] = [2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    static DX: [i32; SMOKEPTNS] = [-2, -1, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3];

    unsafe {
        if x % 4 == 0 {
            for s in &mut S {
                my_mvaddstr(s.y, s.x, ERASER[s.ptrn]);
                s.y -= DY[s.ptrn];
                s.x += DX[s.ptrn];
                s.ptrn = if s.ptrn < SMOKEPTNS - 1 {
                    s.ptrn + 1
                } else {
                    s.ptrn
                };
                my_mvaddstr(s.y, s.x, SMOKE[s.kind][s.ptrn]);
            }
            my_mvaddstr(y, x, SMOKE[S.len() % 2][0]);
            S.push(Smoke {
                y,
                x,
                ptrn: 0,
                kind: S.len() % 2,
            });
        }
    }
}

fn main() {
    println!("Hello, world!");
}
