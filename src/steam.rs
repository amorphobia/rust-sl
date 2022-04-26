#![allow(dead_code)]

pub(crate) const D51HEIGHT: i32 = 10;
pub(crate) const D51FUNNEL: i32 = 7;
pub(crate) const D51LENGTH: i32 = 83;
pub(crate) const D51PATTERNS: i32 = 6;

pub(crate) const D51STR1: &'static str = "      ====        ________                ___________ ";
pub(crate) const D51STR2: &'static str = "  _D _|  |_______/        \\__I_I_____===__|_________| ";
pub(crate) const D51STR3: &'static str = "   |(_)---  |   H\\________/ |   |        =|___ ___|   ";
pub(crate) const D51STR4: &'static str = "   /     |  |   H  |  |     |   |         ||_| |_||   ";
pub(crate) const D51STR5: &'static str = "  |      |  |   H  |__--------------------| [___] |   ";
pub(crate) const D51STR6: &'static str = "  | ________|___H__/__|_____/[][]~\\_______|       |   ";
pub(crate) const D51STR7: &'static str = "  |/ |   |-----------I_____I [][] []  D   |=======|__ ";

pub(crate) const D51WHL11: &'static str =
    "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
pub(crate) const D51WHL12: &'static str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
pub(crate) const D51WHL13: &'static str =
    "  \\_/      \\O=====O=====O=====O_/      \\_/            ";

pub(crate) const D51WHL21: &'static str =
    "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
pub(crate) const D51WHL22: &'static str = " |/-=|___|=O=====O=====O=====O   |_____/~\\___/        ";
pub(crate) const D51WHL23: &'static str =
    "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

pub(crate) const D51WHL31: &'static str = "__/ =| o |=-O=====O=====O=====O \\ ____Y___________|__ ";
pub(crate) const D51WHL32: &'static str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
pub(crate) const D51WHL33: &'static str =
    "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

pub(crate) const D51WHL41: &'static str = "__/ =| o |=-~O=====O=====O=====O\\ ____Y___________|__ ";
pub(crate) const D51WHL42: &'static str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
pub(crate) const D51WHL43: &'static str =
    "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

pub(crate) const D51WHL51: &'static str =
    "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
pub(crate) const D51WHL52: &'static str = " |/-=|___|=   O=====O=====O=====O|_____/~\\___/        ";
pub(crate) const D51WHL53: &'static str =
    "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

pub(crate) const D51WHL61: &'static str =
    "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
pub(crate) const D51WHL62: &'static str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
pub(crate) const D51WHL63: &'static str =
    "  \\_/      \\_O=====O=====O=====O/      \\_/            ";

pub(crate) const D51DEL: &'static str = "                                                      ";

pub(crate) const COAL01: &'static str = "                              ";
pub(crate) const COAL02: &'static str = "                              ";
pub(crate) const COAL03: &'static str = "    _________________         ";
pub(crate) const COAL04: &'static str = "   _|                \\_____A  ";
pub(crate) const COAL05: &'static str = " =|                        |  ";
pub(crate) const COAL06: &'static str = " -|                        |  ";
pub(crate) const COAL07: &'static str = "__|________________________|_ ";
pub(crate) const COAL08: &'static str = "|__________________________|_ ";
pub(crate) const COAL09: &'static str = "   |_D__D__D_|  |_D__D__D_|   ";
pub(crate) const COAL10: &'static str = "    \\_/   \\_/    \\_/   \\_/    ";

pub(crate) const COALDEL: &'static str = "                              ";

pub(crate) const LOGOHEIGHT: u16 = 6;
pub(crate) const LOGOFUNNEL: u16 = 4;
pub(crate) const LOGOLENGTH: u16 = 84;
pub(crate) const LOGOPATTERNS: u16 = 6;

pub(crate) const LOGO1: &'static str = "     ++      +------ ";
pub(crate) const LOGO2: &'static str = "     ||      |+-+ |  ";
pub(crate) const LOGO3: &'static str = "   /---------|| | |  ";
pub(crate) const LOGO4: &'static str = "  + ========  +-+ |  ";

pub(crate) const LWHL11: &'static str = " _|--O========O~\\-+  ";
pub(crate) const LWHL12: &'static str = "//// \\_/      \\_/    ";

pub(crate) const LWHL21: &'static str = " _|--/O========O\\-+  ";
pub(crate) const LWHL22: &'static str = "//// \\_/      \\_/    ";

pub(crate) const LWHL31: &'static str = " _|--/~O========O-+  ";
pub(crate) const LWHL32: &'static str = "//// \\_/      \\_/    ";

pub(crate) const LWHL41: &'static str = " _|--/~\\------/~\\-+  ";
pub(crate) const LWHL42: &'static str = "//// \\_O========O    ";

pub(crate) const LWHL51: &'static str = " _|--/~\\------/~\\-+  ";
pub(crate) const LWHL52: &'static str = "//// \\O========O/    ";

pub(crate) const LWHL61: &'static str = " _|--/~\\------/~\\-+  ";
pub(crate) const LWHL62: &'static str = "//// O========O_/    ";

pub(crate) const LCOAL1: &'static str = "____                 ";
pub(crate) const LCOAL2: &'static str = "|   \\@@@@@@@@@@@     ";
pub(crate) const LCOAL3: &'static str = "|    \\@@@@@@@@@@@@@_ ";
pub(crate) const LCOAL4: &'static str = "|                  | ";
pub(crate) const LCOAL5: &'static str = "|__________________| ";
pub(crate) const LCOAL6: &'static str = "   (O)       (O)     ";

pub(crate) const LCAR1: &'static str = "____________________ ";
pub(crate) const LCAR2: &'static str = "|  ___ ___ ___ ___ | ";
pub(crate) const LCAR3: &'static str = "|  |_| |_| |_| |_| | ";
pub(crate) const LCAR4: &'static str = "|__________________| ";
pub(crate) const LCAR5: &'static str = "|__________________| ";
pub(crate) const LCAR6: &'static str = "   (O)        (O)    ";

pub(crate) const DELLN: &'static str = "                     ";

pub(crate) const C51HEIGHT: u16 = 11;
pub(crate) const C51FUNNEL: u16 = 7;
pub(crate) const C51LENGTH: u16 = 87;
pub(crate) const C51PATTERNS: u16 = 6;

pub(crate) const C51DEL: &'static str = "                                                       ";

pub(crate) const C51STR1: &'static str = "        ___                                            ";
pub(crate) const C51STR2: &'static str = "       _|_|_  _     __       __             ___________";
pub(crate) const C51STR3: &'static str = "    D__/   \\_(_)___|  |__H__|  |_____I_Ii_()|_________|";
pub(crate) const C51STR4: &'static str = "     | `---'   |:: `--'  H  `--'         |  |___ ___|  ";
pub(crate) const C51STR5: &'static str = "    +|~~~~~~~~++::~~~~~~~H~~+=====+~~~~~~|~~||_| |_||  ";
pub(crate) const C51STR6: &'static str = "    ||        | ::       H  +=====+      |  |::  ...|  ";
pub(crate) const C51STR7: &'static str = "|    | _______|_::-----------------[][]-----|       |  ";

pub(crate) const C51WH61: &'static str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
pub(crate) const C51WH62: &'static str = "------'|oOo|==[]=-     ||      ||      |  ||=======_|__";
pub(crate) const C51WH63: &'static str =
    "/~\\____|___|/~\\_|   O=======O=======O  |__|+-/~\\_|     ";
pub(crate) const C51WH64: &'static str =
    "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";

pub(crate) const C51WH51: &'static str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
pub(crate) const C51WH52: &'static str = "------'|oOo|===[]=-    ||      ||      |  ||=======_|__";
pub(crate) const C51WH53: &'static str =
    "/~\\____|___|/~\\_|    O=======O=======O |__|+-/~\\_|     ";
pub(crate) const C51WH54: &'static str =
    "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";

pub(crate) const C51WH41: &'static str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
pub(crate) const C51WH42: &'static str = "------'|oOo|===[]=- O=======O=======O  |  ||=======_|__";
pub(crate) const C51WH43: &'static str =
    "/~\\____|___|/~\\_|      ||      ||      |__|+-/~\\_|     ";
pub(crate) const C51WH44: &'static str =
    "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";

pub(crate) const C51WH31: &'static str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
pub(crate) const C51WH32: &'static str = "------'|oOo|==[]=- O=======O=======O   |  ||=======_|__";
pub(crate) const C51WH33: &'static str =
    "/~\\____|___|/~\\_|      ||      ||      |__|+-/~\\_|     ";
pub(crate) const C51WH34: &'static str =
    "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";

pub(crate) const C51WH21: &'static str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
pub(crate) const C51WH22: &'static str = "------'|oOo|=[]=- O=======O=======O    |  ||=======_|__";
pub(crate) const C51WH23: &'static str =
    "/~\\____|___|/~\\_|      ||      ||      |__|+-/~\\_|     ";
pub(crate) const C51WH24: &'static str =
    "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";

pub(crate) const C51WH11: &'static str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
pub(crate) const C51WH12: &'static str = "------'|oOo|=[]=-      ||      ||      |  ||=======_|__";
pub(crate) const C51WH13: &'static str =
    "/~\\____|___|/~\\_|  O=======O=======O   |__|+-/~\\_|     ";
pub(crate) const C51WH14: &'static str =
    "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";
