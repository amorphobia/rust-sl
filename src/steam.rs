#![doc(hidden)]

pub(crate) const D51HEIGHT: usize = 10;
pub(crate) const D51FUNNEL: usize = 7;
pub(crate) const D51LENGTH: usize = 83;
pub(crate) const D51PATTERNS: usize = 6;

pub(crate) const D51STR1: &str = "      ====        ________                ___________ ";
pub(crate) const D51STR2: &str = "  _D _|  |_______/        \\__I_I_____===__|_________| ";
pub(crate) const D51STR3: &str = "   |(_)---  |   H\\________/ |   |        =|___ ___|   ";
pub(crate) const D51STR4: &str = "   /     |  |   H  |  |     |   |         ||_| |_||   ";
pub(crate) const D51STR5: &str = "  |      |  |   H  |__--------------------| [___] |   ";
pub(crate) const D51STR6: &str = "  | ________|___H__/__|_____/[][]~\\_______|       |   ";
pub(crate) const D51STR7: &str = "  |/ |   |-----------I_____I [][] []  D   |=======|__ ";

pub(crate) const D51WHL11: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
pub(crate) const D51WHL12: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
pub(crate) const D51WHL13: &str = "  \\_/      \\O=====O=====O=====O_/      \\_/            ";

pub(crate) const D51WHL21: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
pub(crate) const D51WHL22: &str = " |/-=|___|=O=====O=====O=====O   |_____/~\\___/        ";
pub(crate) const D51WHL23: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

pub(crate) const D51WHL31: &str = "__/ =| o |=-O=====O=====O=====O \\ ____Y___________|__ ";
pub(crate) const D51WHL32: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
pub(crate) const D51WHL33: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

pub(crate) const D51WHL41: &str = "__/ =| o |=-~O=====O=====O=====O\\ ____Y___________|__ ";
pub(crate) const D51WHL42: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
pub(crate) const D51WHL43: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

pub(crate) const D51WHL51: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
pub(crate) const D51WHL52: &str = " |/-=|___|=   O=====O=====O=====O|_____/~\\___/        ";
pub(crate) const D51WHL53: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

pub(crate) const D51WHL61: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
pub(crate) const D51WHL62: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
pub(crate) const D51WHL63: &str = "  \\_/      \\_O=====O=====O=====O/      \\_/            ";

pub(crate) const D51DEL: &str = "                                                      ";

pub(crate) const COAL01: &str = "                              ";
pub(crate) const COAL02: &str = "                              ";
pub(crate) const COAL03: &str = "    _________________         ";
pub(crate) const COAL04: &str = "   _|                \\_____A  ";
pub(crate) const COAL05: &str = " =|                        |  ";
pub(crate) const COAL06: &str = " -|                        |  ";
pub(crate) const COAL07: &str = "__|________________________|_ ";
pub(crate) const COAL08: &str = "|__________________________|_ ";
pub(crate) const COAL09: &str = "   |_D__D__D_|  |_D__D__D_|   ";
pub(crate) const COAL10: &str = "    \\_/   \\_/    \\_/   \\_/    ";

pub(crate) const COALDEL: &str = "                              ";

pub(crate) const LOGOHEIGHT: usize = 6;
pub(crate) const LOGOFUNNEL: usize = 4;
pub(crate) const LOGOLENGTH: usize = 84;
pub(crate) const LOGOPATTERNS: usize = 6;

pub(crate) const LOGO1: &str = "     ++      +------ ";
pub(crate) const LOGO2: &str = "     ||      |+-+ |  ";
pub(crate) const LOGO3: &str = "   /---------|| | |  ";
pub(crate) const LOGO4: &str = "  + ========  +-+ |  ";

pub(crate) const LWHL11: &str = " _|--O========O~\\-+  ";
pub(crate) const LWHL12: &str = "//// \\_/      \\_/    ";

pub(crate) const LWHL21: &str = " _|--/O========O\\-+  ";
pub(crate) const LWHL22: &str = "//// \\_/      \\_/    ";

pub(crate) const LWHL31: &str = " _|--/~O========O-+  ";
pub(crate) const LWHL32: &str = "//// \\_/      \\_/    ";

pub(crate) const LWHL41: &str = " _|--/~\\------/~\\-+  ";
pub(crate) const LWHL42: &str = "//// \\_O========O    ";

pub(crate) const LWHL51: &str = " _|--/~\\------/~\\-+  ";
pub(crate) const LWHL52: &str = "//// \\O========O/    ";

pub(crate) const LWHL61: &str = " _|--/~\\------/~\\-+  ";
pub(crate) const LWHL62: &str = "//// O========O_/    ";

pub(crate) const LCOAL1: &str = "____                 ";
pub(crate) const LCOAL2: &str = "|   \\@@@@@@@@@@@     ";
pub(crate) const LCOAL3: &str = "|    \\@@@@@@@@@@@@@_ ";
pub(crate) const LCOAL4: &str = "|                  | ";
pub(crate) const LCOAL5: &str = "|__________________| ";
pub(crate) const LCOAL6: &str = "   (O)       (O)     ";

pub(crate) const LCAR1: &str = "____________________ ";
pub(crate) const LCAR2: &str = "|  ___ ___ ___ ___ | ";
pub(crate) const LCAR3: &str = "|  |_| |_| |_| |_| | ";
pub(crate) const LCAR4: &str = "|__________________| ";
pub(crate) const LCAR5: &str = "|__________________| ";
pub(crate) const LCAR6: &str = "   (O)        (O)    ";

pub(crate) const DELLN: &str = "                     ";

pub(crate) const C51HEIGHT: usize = 11;
pub(crate) const C51FUNNEL: usize = 7;
pub(crate) const C51LENGTH: usize = 87;
pub(crate) const C51PATTERNS: usize = 6;

pub(crate) const C51DEL: &str = "                                                       ";

pub(crate) const C51STR1: &str = "        ___                                            ";
pub(crate) const C51STR2: &str = "       _|_|_  _     __       __             ___________";
pub(crate) const C51STR3: &str = "    D__/   \\_(_)___|  |__H__|  |_____I_Ii_()|_________|";
pub(crate) const C51STR4: &str = "     | `---'   |:: `--'  H  `--'         |  |___ ___|  ";
pub(crate) const C51STR5: &str = "    +|~~~~~~~~++::~~~~~~~H~~+=====+~~~~~~|~~||_| |_||  ";
pub(crate) const C51STR6: &str = "    ||        | ::       H  +=====+      |  |::  ...|  ";
pub(crate) const C51STR7: &str = "|    | _______|_::-----------------[][]-----|       |  ";

pub(crate) const C51WH61: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
pub(crate) const C51WH62: &str = "------'|oOo|==[]=-     ||      ||      |  ||=======_|__";
pub(crate) const C51WH63: &str = "/~\\____|___|/~\\_|   O=======O=======O  |__|+-/~\\_|     ";
pub(crate) const C51WH64: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";

pub(crate) const C51WH51: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
pub(crate) const C51WH52: &str = "------'|oOo|===[]=-    ||      ||      |  ||=======_|__";
pub(crate) const C51WH53: &str = "/~\\____|___|/~\\_|    O=======O=======O |__|+-/~\\_|     ";
pub(crate) const C51WH54: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";

pub(crate) const C51WH41: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
pub(crate) const C51WH42: &str = "------'|oOo|===[]=- O=======O=======O  |  ||=======_|__";
pub(crate) const C51WH43: &str = "/~\\____|___|/~\\_|      ||      ||      |__|+-/~\\_|     ";
pub(crate) const C51WH44: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";

pub(crate) const C51WH31: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
pub(crate) const C51WH32: &str = "------'|oOo|==[]=- O=======O=======O   |  ||=======_|__";
pub(crate) const C51WH33: &str = "/~\\____|___|/~\\_|      ||      ||      |__|+-/~\\_|     ";
pub(crate) const C51WH34: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";

pub(crate) const C51WH21: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
pub(crate) const C51WH22: &str = "------'|oOo|=[]=- O=======O=======O    |  ||=======_|__";
pub(crate) const C51WH23: &str = "/~\\____|___|/~\\_|      ||      ||      |__|+-/~\\_|     ";
pub(crate) const C51WH24: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";

pub(crate) const C51WH11: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
pub(crate) const C51WH12: &str = "------'|oOo|=[]=-      ||      ||      |  ||=======_|__";
pub(crate) const C51WH13: &str = "/~\\____|___|/~\\_|  O=======O=======O   |__|+-/~\\_|     ";
pub(crate) const C51WH14: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";
