mod sl;

use sl::*;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

use ncurses::{
    curs_set, endwin, getch, initscr, leaveok, mvaddch, mvcur, nodelay, noecho, refresh, scrollok,
    stdscr, COLS, CURSOR_VISIBILITY, ERR, LINES, OK,
};

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
    for c in chars {
        if mvaddch(y, x, c as u32) == ERR {
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

    let sl_patterns = LOGOPATTERNS as i32;
    let index = ((LOGOLENGTH as i32 + x) % sl_patterns + sl_patterns) % sl_patterns;

    for i in 0..=LOGOHEIGHT {
        my_mvaddstr(y + i as i32, x, sl[index as usize][i]);
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

fn add_D51(x: i32) -> i32 {
    let d51 = [
        [
            D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL11, D51WHL12,
            D51WHL13, D51DEL,
        ],
        [
            D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL21, D51WHL22,
            D51WHL23, D51DEL,
        ],
        [
            D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL31, D51WHL32,
            D51WHL33, D51DEL,
        ],
        [
            D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL41, D51WHL42,
            D51WHL43, D51DEL,
        ],
        [
            D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL51, D51WHL52,
            D51WHL53, D51DEL,
        ],
        [
            D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL61, D51WHL62,
            D51WHL63, D51DEL,
        ],
    ];
    let coal = [
        COAL01, COAL02, COAL03, COAL04, COAL05, COAL06, COAL07, COAL08, COAL09, COAL10, COALDEL,
    ];

    if x < -(D51LENGTH as i32) {
        return ERR;
    }

    let mut y = LINES() / 2 - 5;
    let dy = if FLY.load(Ordering::Relaxed) {
        y = (x / 7) + LINES() - (COLS() / 7) - D51HEIGHT as i32;
        1
    } else {
        0
    };

    let d51_patterns = D51PATTERNS as i32;
    let index = ((D51LENGTH as i32 + x) % d51_patterns + d51_patterns) % d51_patterns;

    for i in 0..=D51HEIGHT {
        my_mvaddstr(y + i as i32, x, d51[index as usize][i]);
        my_mvaddstr(y + i as i32 + dy, x + 53, coal[i]);
    }
    if ACCIDENT.load(Ordering::Relaxed) {
        add_man(y + 2, x + 43);
        add_man(y + 2, x + 47);
    }
    add_smoke(y - 1, x + D51FUNNEL as i32);
    OK
}

fn add_C51(x: i32) -> i32 {
    let c51 = [
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH11, C51WH12,
            C51WH13, C51WH14, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH21, C51WH22,
            C51WH23, C51WH24, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH31, C51WH32,
            C51WH33, C51WH34, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH41, C51WH42,
            C51WH43, C51WH44, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH51, C51WH52,
            C51WH53, C51WH54, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH61, C51WH62,
            C51WH63, C51WH64, C51DEL,
        ],
    ];
    let coal = [
        COALDEL, COAL01, COAL02, COAL03, COAL04, COAL05, COAL06, COAL07, COAL08, COAL09, COAL10,
        COALDEL,
    ];

    if x < -(C51LENGTH as i32) {
        return ERR;
    }

    let mut y = LINES() / 2 - 5;
    let dy = if FLY.load(Ordering::Relaxed) {
        y = (x / 7) + LINES() - (COLS() / 7) - C51HEIGHT as i32;
        1
    } else {
        0
    };

    let c51_patterns = C51PATTERNS as i32;
    let index = ((C51LENGTH as i32 + x) % c51_patterns + c51_patterns) % c51_patterns;

    for i in 0..=C51HEIGHT {
        my_mvaddstr(y + i as i32, x, c51[index as usize][i]);
        my_mvaddstr(y + i as i32 + dy, x + 55, coal[i]);
    }
    if ACCIDENT.load(Ordering::Relaxed) {
        add_man(y + 3, x + 45);
        add_man(y + 3, x + 49);
    }
    add_smoke(y - 1, x + C51FUNNEL as i32);
    OK
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    for arg in &args[1..] {
        if arg.starts_with('-') {
            option(&arg[1..]);
        }
    }

    initscr();
    signal_hook::flag::register(signal_hook::consts::SIGINT, AtomicBool::new(true).into()).unwrap();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    nodelay(stdscr(), true);
    leaveok(stdscr(), true);
    scrollok(stdscr(), false);

    let mut x = COLS() - 1;
    loop {
        if LOGO.load(Ordering::Relaxed) && add_sl(x) == ERR {
            break;
        }

        if C51.load(Ordering::Relaxed) && add_C51(x) == ERR {
            break;
        }

        if add_D51(x) == ERR {
            break;
        }

        getch();
        refresh();
        thread::sleep(Duration::from_millis(40));
        x = x.saturating_sub(1);
    }
    mvcur(0, COLS() - 1, LINES() - 1, 0);
    endwin();
}
