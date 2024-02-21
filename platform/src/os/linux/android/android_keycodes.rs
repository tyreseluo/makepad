use crate::event::KeyCode;
/*
// Extracted from https://android.googlesource.com/platform/frameworks/base/+/95c1165/core/java/android/view/KeyEvent.java
    Unknown = 0,
    SoftLeft = 1,
    SoftRight = 2,
    Home = 3,
    Back = 4,
    Call = 5,
    Endcall = 6,
    Key0 = 7,
    Key1 = 8,
    Key2 = 9,
    Key3 = 10,
    Key4 = 11,
    Key5 = 12,
    Key6 = 13,
    Key7 = 14,
    Key8 = 15,
    Key9 = 16,
    Star = 17,
    Pound = 18,
    DpadUp = 19,
    DpadDown = 20,
    DpadLeft = 21,
    DpadRight = 22,
    DpadCenter = 23,
    VolumeUp = 24,
    VolumeDown = 25,
    Power = 26,
    Camera = 27,
    Clear = 28,
    KeyA = 29,
    KeyB = 30,
    KeyC = 31,
    KeyD = 32,
    KeyE = 33,
    KeyF = 34,
    KeyG = 35,
    KeyH = 36,
    KeyI = 37,
    KeyJ = 38,
    KeyK = 39,
    KeyL = 40,
    KeyM = 41,
    KeyN = 42,
    KeyO = 43,
    KeyP = 44,
    KeyQ = 45,
    KeyR = 46,
    KeyS = 47,
    KeyT = 48,
    KeyU = 49,
    KeyV = 50,
    KeyW = 51,
    KeyX = 52,
    KeyY = 53,
    KeyZ = 54,
    Comma = 55,
    Period = 56,
    AltLeft = 57,
    AltRight = 58,
    ShiftLeft = 59,
    ShiftRight = 60,
    Tab = 61,
    Space = 62,
    Sym = 63,
    Explorer = 64,
    Envelope = 65,
    Enter = 66,
    Del = 67,
    Grave = 68,
    Minus = 69,
    Equals = 70,
    LeftBracket = 71,
    RightBracket = 72,
    Backslash = 73,
    Semicolon = 74,
    Apostrophe = 75,
    Slash = 76,
    At = 77,
    Num = 78,
    Headsethook = 79,
    Focus = 80,
    Plus = 81,
    Menu = 82,
    Notification = 83,
    Search = 84,
    MediaStop = 86,
    MediaNext = 87,
    MediaPrevious = 88,
    MediaRewind = 89,
    MediaFastForward = 90,
    Mute = 91,
    PageUp = 92,
    PageDown = 93,
    Pictsymbols = 94,
    SwitchCharset = 95,
    ButtonA = 96,
    ButtonB = 97,
    ButtonC = 98,
    ButtonX = 99,
    ButtonY = 100,
    ButtonZ = 101,
    ButtonL1 = 102,
    ButtonR1 = 103,
    ButtonL2 = 104,
    ButtonR2 = 105,
    ButtonThumbl = 106,
    ButtonThumbr = 107,
    ButtonStart = 108,
    ButtonSelect = 109,
    ButtonMode = 110,
    Escape = 111,
    ForwardDel = 112,
    CtrlLeft = 113,
    CtrlRight = 114,
    CapsLock = 115,
    ScrollLock = 116,
    MetaLeft = 117,
    MetaRight = 118,
    Function = 119,
    Sysrq = 120,
    Break = 121,
    MoveHome = 122,
    MoveEnd = 123,
    Insert = 124,
    Forward = 125,
    MediaPlay = 126,
    MediaPause = 127,
    MediaClose = 128,
    MediaEject = 129,
    MediaRecord = 130,
    F1 = 131,
    F2 = 132,
    F3 = 133,
    F4 = 134,
    F5 = 135,
    F6 = 136,
    F7 = 137,
    F8 = 138,
    F9 = 139,
    F10 = 140,
    F11 = 141,
    F12 = 142,
    NumLock = 143,
    Numpad0 = 144,
    Numpad1 = 145,
    Numpad2 = 146,
    Numpad3 = 147,
    Numpad4 = 148,
    Numpad5 = 149,
    Numpad6 = 150,
    Numpad7 = 151,
    Numpad8 = 152,
    Numpad9 = 153,
    NumpadDivide = 154,
    NumpadMultiply = 155,
    NumpadSubtract = 156,
    NumpadAdd = 157,
    NumpadDot = 158,
    NumpadComma = 159,
    NumpadEnter = 160,
    NumpadEquals = 161,
    NumpadLeftParen = 162,
    NumpadRightParen = 163,
    VolumeMute = 164,
    Info = 165,
    ChannelUp = 166,
    ChannelDown = 167,
    ZoomIn = 168,
    ZoomOut = 169,
    Tv = 170,
    Window = 171,
    Guide = 172,
    Dvr = 173,
    Bookmark = 174,
    Captions = 175,
    Settings = 176,
    TvPower = 177,
    TvInput = 178,
    StbPower = 179,
    StbInput = 180,
    AvrPower = 181,
    AvrInput = 182,
    ProgRed = 183,
    ProgGreen = 184,
    ProgYellow = 185,
    ProgBlue = 186,
    AppSwitch = 187,
    Button1 = 188,
    Button2 = 189,
    Button3 = 190,
    Button4 = 191,
    Button5 = 192,
    Button6 = 193,
    Button7 = 194,
    Button8 = 195,
    Button9 = 196,
    Button10 = 197,
    Button11 = 198,
    Button12 = 199,
    Button13 = 200,
    Button14 = 201,
    Button15 = 202,
    Button16 = 203,
    LanguageSwitch = 204,
    MannerMode = 205,
    Key3DMode = 206,
    Contacts = 207,
    Calendar = 208,
    Music = 209,
    Calculator = 210,
    ZenkakuHankaku = 211,
    Eisu = 212,
    Muhenkan = 213,
    Henkan = 214,
    KatakanaHiragana = 215,
    Yen = 216,
    Ro = 217,
    Kana = 218,
    Assist = 219,
    BrightnessDown = 220,
    BrightnessUp = 221,
    MediaAudioTrack = 222,
    Sleep = 223,
    Wakeup = 224,
    Pairing = 225,
    MediaTopMenu = 226,
    Key11 = 227,
    Key12 = 228,
    LastChannel = 229,
    TvDataService = 230,
    VoiceAssist = 231,
    TvRadioService = 232,
    TvTeletext = 233,
    TvNumberEntry = 234,
    TvTerrestrialAnalog = 235,
    TvTerrestrialDigital = 236,
    TvSatellite = 237,
    TvSatelliteBs = 238,
    TvSatelliteCs = 239,
    TvSatelliteService = 240,
    TvNetwork = 241,
    TvAntennaCable = 242,
    TvInputHdmi1 = 243,
    TvInputHdmi2 = 244,
    TvInputHdmi3 = 245,
    TvInputHdmi4 = 246,
    TvInputComposite1 = 247,
    TvInputComposite2 = 248,
    TvInputComponent1 = 249,
    TvInputComponent2 = 250,
    TvInputVga1 = 251,
    TvAudioDescription = 252,
    TvAudioDescriptionMixUp = 253,
    TvAudioDescriptionMixDown = 254,
    TvZoomMode = 255,
    TvContentsMenu = 256,
    TvMediaContextMenu = 257,
    TvTimerProgramming = 258,
    Help = 259,
    NavigatePrevious = 260,
    NavigateNext = 261,
    NavigateIn = 262,
    NavigateOut = 263,
    StemPrimary = 264,
    Stem1 = 265,
    Stem2 = 266,
    Stem3 = 267,
    DpadUpLeft = 268,
    DpadDownLeft = 269,
    DpadUpRight = 270,
    DpadDownRight = 271,
    MediaSkipForward = 272,
    MediaSkipBackward = 273,
    MediaStepForward = 274,
    MediaStepBackward = 275,
    SoftSleep = 276,
    Cut = 277,
    Copy = 278,
    Paste = 279,
    SystemNavigationUp = 280,
    SystemNavigationDown = 281,
    SystemNavigationLeft = 282,
    SystemNavigationRight = 283,
    AllApps = 284,
    Refresh = 285,
    ThumbsUp = 286,
    ThumbsDown = 287,
    ProfileSwitch = 288,
    VideoApp1 = 289,
    VideoApp2 = 290,
    VideoApp3 = 291,
    VideoApp4 = 292,
    VideoApp5 = 293,
    VideoApp6 = 294,
    VideoApp7 = 295,
    VideoApp8 = 296,
    FeaturedApp1 = 297,
    FeaturedApp2 = 298,
    FeaturedApp3 = 299,
    FeaturedApp4 = 300,
    DemoApp1 = 301,
    DemoApp2 = 302,
    DemoApp3 = 303,
    DemoApp4 = 304,
*/

pub(crate) fn android_to_makepad_key_code(key_code: u32) -> KeyCode {
    match key_code {
        4 => KeyCode::Back,

        7 => KeyCode::Key0,
        8 => KeyCode::Key1,
        9 => KeyCode::Key2,
        10 => KeyCode::Key3,
        11 => KeyCode::Key4,
        12 => KeyCode::Key5,
        13 => KeyCode::Key6,
        14 => KeyCode::Key7,
        15 => KeyCode::Key8,
        16 => KeyCode::Key9,

        19 => KeyCode::ArrowUp,
        20 => KeyCode::ArrowDown,
        21 => KeyCode::ArrowLeft,
        22 => KeyCode::ArrowRight,

        29 => KeyCode::KeyA,
        30 => KeyCode::KeyB,
        31 => KeyCode::KeyC,
        32 => KeyCode::KeyD,
        33 => KeyCode::KeyE,
        34 => KeyCode::KeyF,
        35 => KeyCode::KeyG,
        36 => KeyCode::KeyH,
        37 => KeyCode::KeyI,
        38 => KeyCode::KeyJ,
        39 => KeyCode::KeyK,
        40 => KeyCode::KeyL,
        41 => KeyCode::KeyM,
        42 => KeyCode::KeyN,
        43 => KeyCode::KeyO,
        44 => KeyCode::KeyP,
        45 => KeyCode::KeyQ,
        46 => KeyCode::KeyR,
        47 => KeyCode::KeyS,
        48 => KeyCode::KeyT,
        49 => KeyCode::KeyU,
        50 => KeyCode::KeyV,
        51 => KeyCode::KeyW,
        52 => KeyCode::KeyX,
        53 => KeyCode::KeyY,
        54 => KeyCode::KeyZ,
        55 => KeyCode::Comma,
        56 => KeyCode::Period,
        57 => KeyCode::Alt,
        58 => KeyCode::Alt,
        59 => KeyCode::Shift,
        60 => KeyCode::Shift,
        61 => KeyCode::Tab,
        62 => KeyCode::Space,

        66 => KeyCode::ReturnKey,
        67 => KeyCode::Backspace,
        68 => KeyCode::Backtick,
        69 => KeyCode::Minus,
        70 => KeyCode::Equals,
        71 => KeyCode::LBracket,
        72 => KeyCode::RBracket,
        73 => KeyCode::Backslash,
        74 => KeyCode::Semicolon,
        75 => KeyCode::Quote,
        76 => KeyCode::Slash,

        92 => KeyCode::PageUp,
        93 => KeyCode::PageDown,

        111 => KeyCode::Escape,
        112 => KeyCode::Delete,
        113 => KeyCode::Control,
        114 => KeyCode::Control,
        115 => KeyCode::Capslock,
        116 => KeyCode::ScrollLock,

        120 => KeyCode::PrintScreen,
        121 => KeyCode::Pause,
        122 => KeyCode::Home,
        123 => KeyCode::End,
        124 => KeyCode::Insert,

        131 => KeyCode::F1,
        132 => KeyCode::F2,
        133 => KeyCode::F3,
        134 => KeyCode::F4,
        135 => KeyCode::F5,
        136 => KeyCode::F6,
        137 => KeyCode::F7,
        138 => KeyCode::F8,
        139 => KeyCode::F9,
        140 => KeyCode::F10,
        141 => KeyCode::F11,
        142 => KeyCode::F12,
        143 => KeyCode::Numlock,
        144 => KeyCode::Numpad0,
        145 => KeyCode::Numpad1,
        146 => KeyCode::Numpad2,
        147 => KeyCode::Numpad3,
        148 => KeyCode::Numpad4,
        149 => KeyCode::Numpad5,
        150 => KeyCode::Numpad6,
        151 => KeyCode::Numpad7,
        152 => KeyCode::Numpad8,
        153 => KeyCode::Numpad9,
        154 => KeyCode::NumpadDivide,
        155 => KeyCode::NumpadMultiply,
        156 => KeyCode::NumpadSubtract,
        157 => KeyCode::NumpadAdd,
        158 => KeyCode::NumpadDecimal,
        159 => KeyCode::NumpadDecimal,
        160 => KeyCode::NumpadEnter,
        161 => KeyCode::NumpadEquals,

        _ => KeyCode::Unknown
    }
}
