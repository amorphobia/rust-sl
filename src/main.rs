//! # Rust SL
//! 
//! A rust port of [sl](https://github.com/mtoyoda/sl).
//! SL (Steam Locomotive) runs across your terminal when you type "sl" as you meant to type "ls". It's just a joke command, and not useful at all.
//! 
//! ## Installing
//! 
//! ### Using [Scoop](https://scoop.sh/)
//! 
//! ```PowerShell
//! scoop bucket add siku https://github.com/amorphobia/siku
//! scoop install rust-sl
//! ```
//! 
//! ### Using [cargo-binstall](https://crates.io/crates/cargo-binstall)
//! 
//! ```Shell
//! cargo binstall rust-sl
//! ```
//! 
//! ### Prebuilt Binaries
//! 
//! Find and download from [release page](https://github.com/amorphobia/rust-sl/releases).
//! 
//! ## Usage
//! 
//! ```PowerShell
//! sl [OPTIONS]
//! ```
//! 
//! ## Options
//! 
//! ```text
//! -a            An accident is occurring. People cry for help
//! -c            C51 appears instead of D51
//! -F            It flies like the galaxy express 999
//! -h, --help    Print help information
//! -l            Little version
//! ```
//! 
//! ## License
//! 
//! The original code was written in c by Toyoda Masashi. See [Original License](#original-license-c-version) below.
//! The modified (rust) code is opensourced under [AGPL-3.0](https://github.com/amorphobia/rust-sl/blob/master/LICENSE).
//! 
//! ## Original License (c version)
//! 
//! Copyright 1993,1998,2014 Toyoda Masashi (mtoyoda@acm.org)
//! 
//! Everyone is permitted to do anything on this program including copying,
//! modifying, and improving, unless you try to pretend that you wrote it.
//! i.e., the above copyright notice has to appear in all copies.
//! THE AUTHOR DISCLAIMS ANY RESPONSIBILITY WITH REGARD TO THIS SOFTWARE.

use clap::Parser;
use crossterm::{
    cursor::{Hide, MoveTo, RestorePosition, SavePosition, Show},
    execute,
    style::Print,
    terminal::{size, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};
use std::{io::stdout, thread, time, sync::{Arc, Mutex}};

mod steam;
use steam::*;

#[doc(hidden)]
fn move_print(x: i32, y: i32, pat: &str) -> Result<()> {
    use std::cmp::{max, min};

    let (cols, rows) = size()?;
    let cols = cols as i32;
    let rows = rows as i32;

    if x >= cols || x + (pat.len() as i32) < 0 || y >= rows || y < 0 {
        return Ok(());
    }

    let upper = min(pat.len(), (cols - x) as usize);
    let lower = max(-x, 0) as usize;

    let pat = &pat[lower..upper];

    execute!(stdout(), MoveTo(max(x, 0) as u16, y as u16), Print(pat))?;

    Ok(())
}

#[doc(hidden)]
#[derive(Parser, Clone, Copy)]
struct Options {
    /// An accident is occurring. People cry for help.
    #[clap(short)]
    accident: bool,

    /// It flies like the galaxy express 999.
    #[clap(short = 'F')]
    fly: bool,

    /// Little version.
    #[clap(short)]
    logo: bool,

    /// C51 appears instead of D51.
    #[clap(short)]
    c51: bool,
}

#[doc(hidden)]
fn main() -> Result<()> {
    let (cols, _) = size()?;
    let options = Options::parse();

    execute!(stdout(), EnterAlternateScreen, Hide, SavePosition)?;

    let running = Arc::new(Mutex::new(true));

    let state = Arc::clone(&running);
    ctrlc::set_handler(move || {
        *state.lock().expect("Running state error.") = false;
    })
    .expect("Setting Ctrl-C handler failed.");

    let mut x = cols as i32 - 1;
    while *running.lock().expect("Running state error.") {
        if options.logo {
            if add_sl(x, options)? { break; }
        } else if options.c51 {
            if add_c51(x, options)? { break; }
        } else if add_d51(x, options)? { break; }

        thread::sleep(time::Duration::from_millis(40));
        x -= 1;
    }

    execute!(stdout(), RestorePosition, Show, LeaveAlternateScreen)?;
    Ok(())
}

#[doc(hidden)]
fn add_sl(x: i32, options: Options) -> Result<bool> {
    const SL: [[&str; LOGOHEIGHT + 1]; LOGOPATTERNS] = [
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL11, LWHL12, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL21, LWHL22, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL31, LWHL32, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL41, LWHL42, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL51, LWHL52, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL61, LWHL62, DELLN],
    ];

    const COAL: [&str; LOGOHEIGHT + 1] = [LCOAL1, LCOAL2, LCOAL3, LCOAL4, LCOAL5, LCOAL6, DELLN];

    const CAR: [&str; LOGOHEIGHT + 1] = [LCAR1, LCAR2, LCAR3, LCAR4, LCAR5, LCAR6, DELLN];

    if x < -(LOGOLENGTH as i32) {
        return Ok(true);
    }

    let (cols, rows) = size()?;
    let cols = cols as i32;
    let rows = rows as i32;

    let (y, py1, py2, py3) = if options.fly {
        let y = (x / 6) + rows - (cols / 6) - LOGOHEIGHT as i32;
        (y, 2, 4, 6)
    } else {
        (rows / 2 - 3, 0, 0, 0)
    };

    for i in 0..=(LOGOHEIGHT as i32) {
        let pat = SL[((LOGOLENGTH as i32 + x) / 3 % LOGOPATTERNS as i32) as usize][i as usize];
        move_print(x, y + i, pat)?;
        move_print(x + 21, y + i + py1, COAL[i as usize])?;
        move_print(x + 42, y + i + py2, CAR[i as usize])?;
        move_print(x + 63, y + i + py3, CAR[i as usize])?;
    }

    if options.accident {
        add_man(x + 14, y + 1)?;
        add_man(x + 45, y + 1 + py2)?;
        add_man(x + 53, y + 1 + py2)?;
        add_man(x + 66, y + 1 + py3)?;
        add_man(x + 74, y + 1 + py3)?;
    }

    add_smoke(x + LOGOFUNNEL as i32, y - 1)?;
    Ok(false)
}

#[doc(hidden)]
fn add_d51(x: i32, options: Options) -> Result<bool> {
    const D51: [[&str; D51HEIGHT + 1]; D51PATTERNS] = [
        [
            D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7,
            D51WHL11, D51WHL12, D51WHL13, D51DEL,
        ],
        [
            D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7,
            D51WHL21, D51WHL22, D51WHL23, D51DEL,
        ],
        [
            D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7,
            D51WHL31, D51WHL32, D51WHL33, D51DEL,
        ],
        [
            D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7,
            D51WHL41, D51WHL42, D51WHL43, D51DEL,
        ],
        [
            D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7,
            D51WHL51, D51WHL52, D51WHL53, D51DEL,
        ],
        [
            D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7,
            D51WHL61, D51WHL62, D51WHL63, D51DEL,
        ],
    ];
    const COAL: [&str; D51HEIGHT + 1] = [
        COAL01, COAL02, COAL03, COAL04, COAL05,
        COAL06, COAL07, COAL08, COAL09, COAL10, COALDEL,
    ];

    if x < -(D51LENGTH as i32) {
        return Ok(true);
    }

    let (cols, rows) = size()?;
    let cols = cols as i32;
    let rows = rows as i32;

    let (y, dy) = if options.fly {
        let y = (x / 7) + rows - (cols / 7) - D51HEIGHT as i32;
        (y, 1)
    } else {
        (rows / 2 - 5, 0)
    };

    for i in 0..=(D51HEIGHT as i32) {
        let pat = D51[((D51LENGTH as i32 + x) % D51PATTERNS as i32) as usize][i as usize];
        move_print(x, y + i, pat)?;
        move_print(x + 53, y + i + dy, COAL[i as usize])?;
    }

    if options.accident {
        add_man(x + 43, y + 2)?;
        add_man(x + 47, y + 2)?;
    }

    add_smoke(x + D51FUNNEL as i32, y - 1)?;
    Ok(false)
}

#[doc(hidden)]
fn add_c51(x: i32, options: Options) -> Result<bool> {
    const C51: [[&str; C51HEIGHT + 1]; C51PATTERNS] = [
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7,
            C51WH11, C51WH12, C51WH13, C51WH14, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7,
            C51WH21, C51WH22, C51WH23, C51WH24, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7,
            C51WH31, C51WH32, C51WH33, C51WH34, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7,
            C51WH41, C51WH42, C51WH43, C51WH44, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7,
            C51WH51, C51WH52, C51WH53, C51WH54, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7,
            C51WH61, C51WH62, C51WH63, C51WH64, C51DEL,
        ],
    ];
    const COAL: [&str; C51HEIGHT + 1] = [
        COALDEL, COAL01, COAL02, COAL03, COAL04, COAL05,
        COAL06, COAL07, COAL08, COAL09, COAL10, COALDEL,
    ];

    if x < -(C51LENGTH as i32) {
        return Ok(true);
    }

    let (cols, rows) = size()?;
    let cols = cols as i32;
    let rows = rows as i32;

    let (y, dy) = if options.fly {
        let y = (x / 7) + rows - (cols / 7) - C51HEIGHT as i32;
        (y, 1)
    } else {
        (rows / 2 - 5, 0)
    };

    for i in 0..=(C51HEIGHT as i32) {
        let pat = C51[((C51LENGTH as i32 + x) % C51PATTERNS as i32) as usize][i as usize];
        move_print(x, y + i, pat)?;
        move_print(x + 55, y + i + dy, COAL[i as usize])?;
    }

    if options.accident {
        add_man(x + 45, y + 3)?;
        add_man(x + 49, y + 3)?;
    }

    add_smoke(x + C51FUNNEL as i32, y - 1)?;
    Ok(false)
}

#[doc(hidden)]
fn add_man(x: i32, y: i32) -> Result<()> {
    const MAN: [[&str; 2]; 2] = [["", "(O)"], ["Help!", "\\O/"]];

    for i in 0..2 {
        let pat = MAN[((LOGOLENGTH as i32 + x) / 12 % 2) as usize][i as usize];
        move_print(x, y + i, pat)?;
    }
    Ok(())
}

#[doc(hidden)]
fn add_smoke(x: i32, y: i32) -> Result<()> {
    use lazy_static::lazy_static;

    #[derive(Clone, Copy)]
    struct Smokes {
        x: i32,
        y: i32,
        ptrn: usize,
        kind: usize,
    }

    impl Smokes {
        const fn new() -> Smokes {
            Smokes { x: 0, y: 0, ptrn: 0, kind: 0 }
        }
    }

    const SMOKEPTNS: usize = 16;
    const SMOKE: [[&str; SMOKEPTNS]; 2] = [
        [
            "(   )", "(    )", "(    )", "(   )", "(  )",
            "(  )" , "( )"   , "( )"   , "()"   , "()"  ,
            "O"    , "O"     , "O"     , "O"    , "O"   ,
            " "    ,
        ],
        [
            "(@@@)", "(@@@@)", "(@@@@)", "(@@@)", "(@@)",
            "(@@)" , "(@)"   , "(@)"   , "@@"   , "@@"  ,
            "@"    , "@"     , "@"     , "@"    , "@"   ,
            " "    ,
        ],
    ];
    const ERASER: [&str; SMOKEPTNS] = [
        "     ", "      ", "      ", "     ", "    ",
        "    " , "   "   , "   "   , "  "   , "  "  ,
        " "    , " "     , " "     , " "    , " "   ,
        " "    ,
    ];
    const DY: [i32; SMOKEPTNS] = [2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    const DX: [i32; SMOKEPTNS] = [-2, -1, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3];

    lazy_static! {
        static ref S: Mutex<[Smokes; 1000]> = Mutex::new([Smokes::new(); 1000]);
        static ref SUM: Mutex<usize> = Mutex::new(0);
    }

    if x % 4 == 0 {
        let sum = *SUM.lock().expect("Accessing global static SUM failed.");
        let mut smokes = S.lock().expect("Accessing global static S failed.");
        for i in 0..sum {
            let smoke = &mut smokes[i];
            move_print(smoke.x, smoke.y, ERASER[smoke.ptrn])?;
            smoke.y -= DY[smoke.ptrn] as i32;
            smoke.x += DX[smoke.ptrn] as i32;
            smoke.ptrn += if smoke.ptrn < SMOKEPTNS - 1 { 1 } else { 0 };
            move_print(smoke.x, smoke.y, SMOKE[smoke.kind][smoke.ptrn])?;
        }

        move_print(x, y, SMOKE[sum % 2][0])?;

        let smoke = &mut smokes[sum];
        smoke.y = y as i32;
        smoke.x = x as i32;
        smoke.ptrn = 0;
        smoke.kind = sum % 2;
        *SUM.lock().expect("Accessing global static SUM failed.") += 1;
    }
    Ok(())
}
