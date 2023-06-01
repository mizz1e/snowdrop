use {
    clap::{ArgAction, Parser, ValueEnum},
    std::{fmt, mem, str},
};

pub use crate::parser::EnumFromStrValueParser;

mod parser;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, ValueEnum)]
#[repr(u8)]
pub enum Impacts {
    /// Don't show any.
    #[default]
    Hidden,
    /// Show client (red), and server (blue).
    Both,
    /// Show client (red).
    Client,
    /// Show server (blue).
    Server,
}

#[derive(Debug)]
#[elysium_macros::console]
pub enum Console {
    /// Toggle buddah mode.
    ///
    /// Forces you to have 1 health.
    ///
    /// Requires sv_cheats.
    Buddah,

    /// Print the position of the player.
    #[console(name = "getpos")]
    GetPos,

    /// Toggle invincibility.
    ///
    /// Requires sv_cheats.
    God,

    /// Restart the game in n seconds.
    ///
    /// Requires sv_cheats.
    #[console(name = "mp_restartgame")]
    MpRestartGame {
        #[arg(default = 0.0)]
        seconds: f32,
    },

    /// Toggle collision.
    ///
    /// Requires sv_cheats.
    #[console(name = "noclip")]
    NoClip,

    /// Toggle NPC detection of the player.
    ///
    /// Requires sv_cheats.
    #[console(name = "notarget")]
    NoTarget,

    /// Set the coordinates of the player.
    #[console(name = "setpos")]
    SetPos { x: f32, y: f32, z: f32 },

    /// Air acceleration modifier.
    #[console(name = "sv_airaccelerate")]
    SvAirAccelerate {
        #[arg(default = 0.0)]
        accelerate: f32,
    },

    /// Automatically bunny hop.
    #[console(name = "sv_autobunnyhopping")]
    SvAutoBunnyHopping {
        #[arg(default = false)]
        bunny_hopping: bool,
    },

    /// Whether cheats are enabled on the server.
    SvCheats {
        #[arg(default = false)]
        cheats: bool,
    },

    /// Whether bunny hopping is allowed.
    #[console(name = "sv_enablebunnyhopping")]
    SvEnableBunnyHopping {
        #[arg(default = false)]
        bunny_hopping: bool,
    },

    /// Gravity applied to all entities.
    SvGravity {
        #[arg(default = 800.0)]
        gravity: f32,
    },

    /// Whether to verify content with the server.
    SvPure {
        #[arg(default = true)]
        pure: bool,
    },

    /// Show bullet impacts.    
    #[console(name = "sv_showimpacts")]
    SvShowImpacts {
        #[arg(default = Impacts::default())]
        #[arg(parser = EnumFromStrValueParser::<Impacts>::new())]
        impacts: Impacts,
    },
}

impl Impacts {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Hidden => "hidden",
            Self::Both => "both",
            Self::Client => "client",
            Self::Server => "server",
        }
    }

    #[inline]
    pub const fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub fn from_label(impacts: &str) -> Option<Self> {
        let impacts = match impacts {
            "hidden" => Self::Hidden,
            "both" => Self::Both,
            "client" => Self::Client,
            "server" => Self::Server,
            _ => return None,
        };

        Some(impacts)
    }

    pub const fn from_u8(impacts: u8) -> Option<Self> {
        if matches!(impacts, 0..=3) {
            Some(unsafe { Self::from_u8_unchecked(impacts) })
        } else {
            None
        }
    }

    #[inline]
    pub const unsafe fn from_u8_unchecked(impacts: u8) -> Self {
        mem::transmute(impacts)
    }
}

impl fmt::Display for Impacts {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}

impl str::FromStr for Impacts {
    type Err = &'static str;

    fn from_str(impacts: &str) -> Result<Self, Self::Err> {
        if let Some(impacts) = Self::from_label(impacts) {
            return Ok(impacts);
        }

        if let Some(impacts) = impacts.parse::<u8>().ok().and_then(Self::from_u8) {
            return Ok(impacts);
        }

        Err("invalid value for impacts")
    }
}

fn main() {
    let command = Console::parse_from(std::env::args_os().skip(1));

    println!("{command:?}");
}
