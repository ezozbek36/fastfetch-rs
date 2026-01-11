//! Logo database with ASCII art for various distributions

use crate::output::Color;

/// Logo definition with ASCII art and optional color
#[derive(Debug, Clone)]
pub struct LogoDefinition {
    pub lines: &'static [&'static str],
    pub color: Option<Color>,
}

/// Get logo for Arch Linux
pub fn arch_linux() -> LogoDefinition {
    LogoDefinition {
        lines: &[
            "                   -`                  ",
            "                  .o+`                 ",
            "                 `ooo/                 ",
            "                `+oooo:                ",
            "               `+oooooo:               ",
            "               -+oooooo+:              ",
            "             `/:-:++oooo+:             ",
            "            `/++++/+++++++:            ",
            "           `/++++++++++++++:           ",
            "          `/+++ooooooooooooo/`         ",
            "         ./ooosssso++osssssso+`        ",
            "        .oossssso-````/ossssss+`       ",
            "       -osssssso.      :ssssssso.      ",
            "      :osssssss/        osssso+++.     ",
            "     /ossssssss/        +ssssooo/-     ",
            "   `/ossssso+/:-        -:/+osssso+-   ",
            "  `+sso+:-`                 `.-/+oso:  ",
            " `++:.                           `-/+/ ",
            " .`                                 `/ ",
        ],
        color: Some(Color::BrightCyan),
    }
}

/// Get logo for Ubuntu
pub fn ubuntu() -> LogoDefinition {
    LogoDefinition {
        lines: &[
            "            .-/+oossssoo+/-.            ",
            "        `:+ssssssssssssssssss+:`        ",
            "      -+ssssssssssssssssssyyssss+-      ",
            "    .ossssssssssssssssso/.  .ossssso.   ",
            "   +sssssssssssssso:.         .+sssso+  ",
            "  +ssssssssso+:-`               :ysssss+",
            "  ossso+/:.`                     .ossso ",
            " `ossso-                          -ysss ",
            "  :oooo:          .`           `.oooo:  ",
            "   /ooooo/-..             `../ooooo/   ",
            "    -/ooooooooo++++++oooooooooo/-`     ",
            "      `-/+ooooooooooooooo+/:.`         ",
        ],
        color: Some(Color::BrightRed),
    }
}

/// Get logo for Debian
pub fn debian() -> LogoDefinition {
    LogoDefinition {
        lines: &[
            "       _,met$$$$$gg.          ",
            "    ,g$$$$$$$$$$$$$$$P.       ",
            "  ,g$$P\"     \"\"\"Y$$.\"  .      ",
            " ,$$P'              `$$$.     ",
            "',$$P       ,ggs.     `$$b:   ",
            "`d$$'     ,$P\"'   .    $$$    ",
            " $$P      d$'     ,    $$P    ",
            " $$:      $$.   -    ,d$$'    ",
            " $$;      Y$b._   _,d$P'      ",
            " Y$$.    `.`\"Y$$$$P\"'         ",
            " `$$b      \"-.__              ",
            "  `Y$$                        ",
            "   `Y$$.                      ",
            "     `$$b.                    ",
            "       `Y$$b.                 ",
            "          `\"Y$b._             ",
            "              `\"\"\"\"           ",
        ],
        color: Some(Color::BrightRed),
    }
}

/// Get logo for Fedora
pub fn fedora() -> LogoDefinition {
    LogoDefinition {
        lines: &[
            "             /',                      ",
            "            //  '                     ",
            "           //  /                      ",
            "          //  /                       ",
            "         //  /                        ",
            "        //  /                         ",
            "       //  /                          ",
            "      //  /                           ",
            "     //  /                            ",
            "    ',   '                            ",
            "      ',                              ",
            "        ',                            ",
            "          ',_,,,_,,                   ",
        ],
        color: Some(Color::BrightBlue),
    }
}

/// Get logo for CachyOS
pub fn cachyos() -> LogoDefinition {
    // CachyOS is Arch-based, use Arch logo with different color
    arch_linux()
}

/// Get logo for Manjaro
pub fn manjaro() -> LogoDefinition {
    LogoDefinition {
        lines: &[
            "██████████████████  ████████",
            "██████████████████  ████████",
            "██████████████████  ████████",
            "██████████████████  ████████",
            "████████            ████████",
            "████████  ████████  ████████",
            "████████  ████████  ████████",
            "████████  ████████  ████████",
            "████████  ████████  ████████",
            "████████  ████████  ████████",
            "████████  ████████  ████████",
            "████████  ████████  ████████",
            "████████  ████████  ████████",
            "████████  ████████  ████████",
        ],
        color: Some(Color::BrightGreen),
    }
}

/// Get logo for Gentoo
pub fn gentoo() -> LogoDefinition {
    LogoDefinition {
        lines: &[
            "         -/osy+:.              ",
            "        :ooooooo+/`            ",
            "       :oooooooooo+.           ",
            "      -+oooooooooooo.          ",
            "     .+ooooooooooooo/          ",
            "     :ooooooooooooo+-          ",
            "     /ooooooooooo+/-           ",
            "     /ooooooooo+/.             ",
            "     :ooooooo+/-               ",
            "      -+ooo+/.                 ",
            "       `:/-`                   ",
        ],
        color: Some(Color::Magenta),
    }
}

/// Get logo for openSUSE
pub fn opensuse() -> LogoDefinition {
    LogoDefinition {
        lines: &[
            "           .;ldkO0000Okdl;.           ",
            "       .;d00xl:^''''''^:ok00d;.       ",
            "     .d00l'                'o00d.     ",
            "   .d0Kd'  Okxol:;,.          :O0d.   ",
            "  .OKKKK0kOKKKKKKKKKKKOxo:,    lKO.  ",
            " ,0KKKKKKKKKKKKKKKK0d:,,,,;cxKKK0:  ",
            ".OKKKKKKKKKKKKKKKKk.        ,d0KK0. ",
            ":KKKKKKKKKKKKKKKK:            .OK:  ",
            "dKKKKKKKKKKKKKK0.              .c,  ",
            "dKKKKKKKKKKKKKK0.                   ",
            ":KKKKKKKKKKKKKKKK:            .OK:  ",
            ".OKKKKKKKKKKKKKKKKk.        ;d0KK0. ",
            " ,0KKKKKKKKKKKKKKKK0kc:,,;xKKKKK0:  ",
            "  .OKKKK0xdxkOOOO0KKKKKKKKKKK0.    ",
            "   .d0Ko.     ...''',;:clxKKd.     ",
            "     'l0Kk:.            .d0l.       ",
            "       .;lxOkdl:;,,;:ldO0d;.        ",
            "           .,cdk00000xc:.           ",
        ],
        color: Some(Color::BrightGreen),
    }
}

/// Get default generic Linux logo
pub fn generic_linux() -> LogoDefinition {
    LogoDefinition {
        lines: &[
            "        #####        ",
            "       #######       ",
            "       ##O#O##       ",
            "       #######       ",
            "     ###########     ",
            "    #############    ",
            "   ###############   ",
            "   ################  ",
            "  #################  ",
            "#####################",
            "#####################",
            "  #################  ",
        ],
        color: Some(Color::White),
    }
}

/// Detect distribution from /etc/os-release and return appropriate logo
pub fn detect_logo() -> LogoDefinition {
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/etc/os-release")
            .or_else(|_| std::fs::read_to_string("/usr/lib/os-release"))
        {
            for line in content.lines() {
                if let Some(id) = line.strip_prefix("ID=") {
                    let id = id.trim_matches('"').to_lowercase();
                    return match id.as_str() {
                        "arch" | "archlinux" => arch_linux(),
                        "cachyos" => cachyos(),
                        "manjaro" => manjaro(),
                        "ubuntu" => ubuntu(),
                        "debian" => debian(),
                        "fedora" => fedora(),
                        "gentoo" => gentoo(),
                        "opensuse" | "opensuse-leap" | "opensuse-tumbleweed" => opensuse(),
                        _ => generic_linux(),
                    };
                }
            }
        }
        generic_linux()
    }

    #[cfg(not(target_os = "linux"))]
    {
        generic_linux()
    }
}
