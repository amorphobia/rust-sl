mod steam;

use std::{io::stdout, thread, time};

use crossterm::{
    cursor::{Hide, MoveTo, RestorePosition, Show},
    execute,
    style::Print,
    terminal::{size, Clear, ClearType},
    Result,
};
use steam::*;

fn move_print(x: i32, y: i32, pat: &str) -> Result<()> {
    use std::cmp::max;

    let (cols, rows) = size()?;

    if x >= cols as i32 || y >= rows as i32 || y < 0 || x + (pat.len() as i32) < 0 {
        return Ok(());
    }

    let upper = if x + pat.len() as i32 > cols as i32 {
        (cols as i32 - x) as usize
    } else {
        pat.len()
    };
    let lower = if x < 0 { -x as usize } else { 0 };

    let pat = &pat[lower..upper];

    execute!(stdout(), MoveTo(max(x, 0) as u16, y as u16), Print(pat))?;

    Ok(())
}

fn main() -> Result<()> {
    let (cols, _) = size()?;

    execute!(stdout(), Clear(ClearType::All), Hide)?;

    // for x in (0..(D51LENGTH + cols as i32)).rev() {
    for x in (-D51LENGTH..(cols as i32)).rev() {
        add_d51(x)?;
        thread::sleep(time::Duration::from_millis(40));
    }

    execute!(stdout(), RestorePosition, Show)?;

    // for x in (0..(columns + D51LENGTH)).rev() {
    //     if x >= D51LENGTH {
    //         execute!(stdout(), MoveTo(x - D51LENGTH, 0), Print("-"))?;
    //     }
    //     thread::sleep(time::Duration::from_millis(40));
    // }

    // execute!(
    //     stdout(),
    //     MoveTo(0, 1),
    //     Print(format!("{} {}", columns, rows)),
    //     RestorePosition,
    //     Show
    // )?;

    Ok(())
}

fn add_d51(x: i32) -> Result<()> {
    // use std::cmp::min;

    const D51: [[&str; (D51HEIGHT + 1) as usize]; D51PATTERNS as usize] = [
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
    const COAL: [&str; (D51HEIGHT + 1) as usize] = [
        COAL01, COAL02, COAL03, COAL04, COAL05, COAL06, COAL07, COAL08, COAL09, COAL10, COALDEL,
    ];

    let (_cols, rows) = size()?;
    let rows = rows as i32;

    // x ranges from 0 to cols + D51LENGTH - 1
    let y = rows / 2 - 5;
    // let mut y = rows / 2 - 5;
    let dy = 0;
    // let mut dy = 0;

    // if fly {
    //     y = (x / 7) + rows - (cols / 7) - D51HEIGHT;
    //     dy = 1;
    // }

    for i in 0..=D51HEIGHT {
        // let d51_pat = D51[((D51LENGTH + x) % D51PATTERNS) as usize][i as usize];
        // let (d51_x, d51_pat) = if x >= D51LENGTH {
        //     let pos = min((cols + D51LENGTH - x) as usize, d51_pat.len());
        //     (x - D51LENGTH, &d51_pat[0..pos])
        // } else {
        //     let pos = min((D51LENGTH - x) as usize, d51_pat.len());
        //     (0, &d51_pat[pos..])
        // };

        // let coal_pat = COAL[i as usize];
        // let (coal_x, coal_pat) = if x + 53 > cols + D51LENGTH {
        //     (d51_x + 53, "")
        // } else if x + 53 >= D51LENGTH {
        //     let pos = min((cols + D51LENGTH - x - 53) as usize, coal_pat.len());
        //     (x + 53 - D51LENGTH, &coal_pat[0..pos])
        // } else {
        //     let pos = min((D51LENGTH - x - 53) as usize, coal_pat.len());
        //     (0, &coal_pat[pos..])
        // };

        // execute!(
        //     stdout(),
        //     MoveTo(d51_x, y + i),
        //     Print(d51_pat),
        //     MoveTo(coal_x, y + i + dy),
        //     Print(coal_pat)
        // )?;

        move_print(
            x,
            y + i,
            D51[((D51LENGTH + x) % D51PATTERNS) as usize][i as usize],
        )?;
        move_print(x + 53, y + i + dy, COAL[i as usize])?;
    }

    // if accident {
    // add_man(x + 45, y + 3)?;
    // add_man(x + 49, y + 3)?;
    // }

    // add_smoke(x - D51LENGTH + C51FUNNEL, y - 1)?;

    Ok(())
}

#[allow(dead_code)]
fn add_man(_x: u16, _y: u16) -> Result<()> {
    todo!()
}

#[allow(dead_code)]
fn add_smoke(x: u16, y: u16) -> Result<()> {
    #[derive(Default, Clone, Copy)]
    struct Smokes {
        x: i32,
        y: i32,
        ptrn: usize,
        kind: usize,
    }

    impl Smokes {
        const fn new() -> Smokes {
            Smokes {
                x: 0,
                y: 0,
                ptrn: 0,
                kind: 0,
            }
        }
    }

    const SMOKEPTNS: usize = 16;
    const SMOKE: [[&str; SMOKEPTNS]; 2] = [
        [
            "(   )", "(    )", "(    )", "(   )", "(  )", "(  )", "( )", "( )", "()", "()", "O",
            "O", "O", "O", "O", " ",
        ],
        [
            "(@@@)", "(@@@@)", "(@@@@)", "(@@@)", "(@@)", "(@@)", "(@)", "(@)", "@@", "@@", "@",
            "@", "@", "@", "@", " ",
        ],
    ];
    const ERASER: [&str; SMOKEPTNS] = [
        "     ", "      ", "      ", "     ", "    ", "    ", "   ", "   ", "  ", "  ", " ", " ",
        " ", " ", " ", " ",
    ];
    const DY: [i32; SMOKEPTNS] = [2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    const DX: [i32; SMOKEPTNS] = [-2, -1, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3];

    static mut S: [Smokes; 1000] = [Smokes::new(); 1000];
    static mut SUM: usize = 0;

    if x % 4 == 0 {
        let sum = unsafe { SUM };
        for i in 0..sum {
            let smoke = unsafe { S[i] };
            execute!(
                stdout(),
                MoveTo(smoke.x as u16, smoke.y as u16),
                Print(ERASER[smoke.ptrn])
            )?;
            unsafe {
                S[i].y -= DY[S[i].ptrn] as i32;
                S[i].x += DX[S[i].ptrn] as i32;
                S[i].ptrn += if S[i].ptrn < SMOKEPTNS - 1 { 1 } else { 0 };
            }
            let smoke = unsafe { S[i] };
            execute!(
                stdout(),
                MoveTo(smoke.x as u16, smoke.y as u16),
                Print(SMOKE[smoke.kind][smoke.ptrn])
            )?;
        }

        execute!(stdout(), MoveTo(x, y), Print(SMOKE[sum % 2][0]))?;

        unsafe {
            S[sum].y = y as i32;
            S[sum].x = x as i32;
            S[sum].ptrn = 0;
            S[sum].kind = sum % 2;
            SUM += 1;
        }
    }

    Ok(())
}
