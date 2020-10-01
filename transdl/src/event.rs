use crate::{get_error, ll};
use std::collections::BTreeSet;
use std::convert::TryFrom;

pub enum Event {
    Active,
    KeyDown {
        key: Option<KeyCode>,
        modifiers: BTreeSet<Modifier>,
    },
    KeyUp {
        key: Option<KeyCode>,
        modifiers: BTreeSet<Modifier>,
    },
    MouseMotion {
        x: u16,
        y: u16,
        x_rel: i16,
        y_rel: i16,
        buttons: BTreeSet<MouseButton>,
    },
    MouseButtonDown {
        button: Option<MouseButton>,
        x: u16,
        y: u16,
    },
    MouseButtonUp {
        button: Option<MouseButton>,
        x: u16,
        y: u16,
    },
    JoystickAxisMotion,
    JoystickBallMotion,
    JoystickHatMotion,
    JoystickButtonDown,
    JoystickButtonUp,
    Quit,
    VideoResize,
    VideoExpose,
    UserEvent,
}

impl Event {
    pub fn wait() -> Result<Event, String> {
        let mut raw = ll::SDL_Event { type_: 0 };
        let res =
            unsafe { ll::SDL_WaitEvent(&mut raw as *mut ll::SDL_Event) };

        if res == 0 {
            return Err(get_error()
                .unwrap_or_else(|| "Unknown SDL error".to_string()));
        }

        match unsafe { raw.type_ } as u32 {
            ll::SDL_EventType_SDL_ACTIVEEVENT => {
                // TODO: Incomplete
                Ok(Event::Active)
            }
            ll::SDL_EventType_SDL_KEYDOWN => {
                let e = unsafe { raw.key };
                let sym = e.keysym;

                let key = KeyCode::try_from(sym.sym as isize).ok();
                let modifiers = u32_to_modifiers(sym.mod_);

                Ok(Event::KeyDown { key, modifiers })
            }
            ll::SDL_EventType_SDL_KEYUP => {
                let e = unsafe { raw.key };
                let sym = e.keysym;

                let key = KeyCode::try_from(sym.sym as isize).ok();
                let modifiers = u32_to_modifiers(sym.mod_);

                Ok(Event::KeyUp { key, modifiers })
            }
            ll::SDL_EventType_SDL_MOUSEMOTION => {
                let e = unsafe { raw.motion };

                Ok(Event::MouseMotion {
                    x: e.x,
                    y: e.y,
                    x_rel: e.xrel,
                    y_rel: e.yrel,
                    buttons: u8_flags_to_mouse_buttons(e.state),
                })
            }
            ll::SDL_EventType_SDL_MOUSEBUTTONDOWN => {
                let e = unsafe { raw.button };

                Ok(Event::MouseButtonDown {
                    x: e.x,
                    y: e.y,
                    button: u8_to_mouse_button(e.button),
                })
            }
            ll::SDL_EventType_SDL_MOUSEBUTTONUP => {
                let e = unsafe { raw.button };

                Ok(Event::MouseButtonUp {
                    x: e.x,
                    y: e.y,
                    button: u8_to_mouse_button(e.button),
                })
            }
            ll::SDL_EventType_SDL_JOYAXISMOTION => {
                // TODO: Incomplete
                Ok(Event::JoystickAxisMotion)
            }
            ll::SDL_EventType_SDL_JOYBALLMOTION => {
                // TODO: Incomplete
                Ok(Event::JoystickBallMotion)
            }
            ll::SDL_EventType_SDL_JOYHATMOTION => {
                // TODO: Incomplete
                Ok(Event::JoystickHatMotion)
            }
            ll::SDL_EventType_SDL_JOYBUTTONDOWN => {
                // TODO: Incomplete
                Ok(Event::JoystickButtonDown)
            }
            ll::SDL_EventType_SDL_JOYBUTTONUP => {
                // TODO: Incomplete
                Ok(Event::JoystickButtonUp)
            }
            ll::SDL_EventType_SDL_QUIT => Ok(Event::Quit),
            ll::SDL_EventType_SDL_VIDEORESIZE => {
                // TODO: Incomplete
                Ok(Event::VideoResize)
            }
            ll::SDL_EventType_SDL_VIDEOEXPOSE => {
                // TODO: Incomplete
                Ok(Event::VideoExpose)
            }
            ll::SDL_EventType_SDL_USEREVENT => {
                // TODO: Incomplete
                Ok(Event::UserEvent)
            }
            v => Err(format!("Unknown event type {:?}", v)),
        }
    }
}

pub fn push_user_event() {
    let res = unsafe {
        let mut event = ll::SDL_Event { type_: 0 };
        event.user.type_ = ll::SDL_EventType_SDL_USEREVENT as u8;
        event.user.code = 0;
        event.user.data1 = std::ptr::null_mut();
        event.user.data2 = std::ptr::null_mut();
        ll::SDL_PushEvent(&mut event as *mut ll::SDL_Event)
    };
    if res != 0 {
        eprintln!("Couldn't push event: {:?}", crate::get_error());
    }
}

pub fn enable_key_repeat() {
    unsafe {
        ll::SDL_EnableKeyRepeat(
            ll::SDL_DEFAULT_REPEAT_DELAY as i32,
            ll::SDL_DEFAULT_REPEAT_INTERVAL as i32,
        );
    }
}

pub fn disable_key_repeat() {
    unsafe {
        ll::SDL_EnableKeyRepeat(0, 0);
    }
}

#[repr(isize)]
#[derive(Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
pub enum KeyCode {
    BackSpace = ll::SDLKey_SDLK_BACKSPACE as isize,
    Tab = ll::SDLKey_SDLK_TAB as isize,
    Clear = ll::SDLKey_SDLK_CLEAR as isize,
    Return = ll::SDLKey_SDLK_RETURN as isize,
    Pause = ll::SDLKey_SDLK_PAUSE as isize,
    Escape = ll::SDLKey_SDLK_ESCAPE as isize,
    Space = ll::SDLKey_SDLK_SPACE as isize,
    Exclaim = ll::SDLKey_SDLK_EXCLAIM as isize,
    QuoteDouble = ll::SDLKey_SDLK_QUOTEDBL as isize,
    Hash = ll::SDLKey_SDLK_HASH as isize,
    Dollar = ll::SDLKey_SDLK_DOLLAR as isize,
    Ampersand = ll::SDLKey_SDLK_AMPERSAND as isize,
    Quote = ll::SDLKey_SDLK_QUOTE as isize,
    LeftParen = ll::SDLKey_SDLK_LEFTPAREN as isize,
    RightParen = ll::SDLKey_SDLK_RIGHTPAREN as isize,
    Asterisk = ll::SDLKey_SDLK_ASTERISK as isize,
    Plus = ll::SDLKey_SDLK_PLUS as isize,
    Comma = ll::SDLKey_SDLK_COMMA as isize,
    Minus = ll::SDLKey_SDLK_MINUS as isize,
    Period = ll::SDLKey_SDLK_PERIOD as isize,
    Slash = ll::SDLKey_SDLK_SLASH as isize,
    Num0 = ll::SDLKey_SDLK_0 as isize,
    Num1 = ll::SDLKey_SDLK_1 as isize,
    Num2 = ll::SDLKey_SDLK_2 as isize,
    Num3 = ll::SDLKey_SDLK_3 as isize,
    Num4 = ll::SDLKey_SDLK_4 as isize,
    Num5 = ll::SDLKey_SDLK_5 as isize,
    Num6 = ll::SDLKey_SDLK_6 as isize,
    Num7 = ll::SDLKey_SDLK_7 as isize,
    Num8 = ll::SDLKey_SDLK_8 as isize,
    Num9 = ll::SDLKey_SDLK_9 as isize,
    Colon = ll::SDLKey_SDLK_COLON as isize,
    Semicolon = ll::SDLKey_SDLK_SEMICOLON as isize,
    Less = ll::SDLKey_SDLK_LESS as isize,
    Equals = ll::SDLKey_SDLK_EQUALS as isize,
    Greater = ll::SDLKey_SDLK_GREATER as isize,
    Question = ll::SDLKey_SDLK_QUESTION as isize,
    At = ll::SDLKey_SDLK_AT as isize,
    LeftBracket = ll::SDLKey_SDLK_LEFTBRACKET as isize,
    Backslash = ll::SDLKey_SDLK_BACKSLASH as isize,
    RightBracket = ll::SDLKey_SDLK_RIGHTBRACKET as isize,
    Caret = ll::SDLKey_SDLK_CARET as isize,
    Underscore = ll::SDLKey_SDLK_UNDERSCORE as isize,
    Backquote = ll::SDLKey_SDLK_BACKQUOTE as isize,
    A = ll::SDLKey_SDLK_a as isize,
    B = ll::SDLKey_SDLK_b as isize,
    C = ll::SDLKey_SDLK_c as isize,
    D = ll::SDLKey_SDLK_d as isize,
    E = ll::SDLKey_SDLK_e as isize,
    F = ll::SDLKey_SDLK_f as isize,
    G = ll::SDLKey_SDLK_g as isize,
    H = ll::SDLKey_SDLK_h as isize,
    I = ll::SDLKey_SDLK_i as isize,
    J = ll::SDLKey_SDLK_j as isize,
    K = ll::SDLKey_SDLK_k as isize,
    L = ll::SDLKey_SDLK_l as isize,
    M = ll::SDLKey_SDLK_m as isize,
    N = ll::SDLKey_SDLK_n as isize,
    O = ll::SDLKey_SDLK_o as isize,
    P = ll::SDLKey_SDLK_p as isize,
    Q = ll::SDLKey_SDLK_q as isize,
    R = ll::SDLKey_SDLK_r as isize,
    S = ll::SDLKey_SDLK_s as isize,
    T = ll::SDLKey_SDLK_t as isize,
    U = ll::SDLKey_SDLK_u as isize,
    V = ll::SDLKey_SDLK_v as isize,
    W = ll::SDLKey_SDLK_w as isize,
    X = ll::SDLKey_SDLK_x as isize,
    Y = ll::SDLKey_SDLK_y as isize,
    Z = ll::SDLKey_SDLK_z as isize,
    Delete = ll::SDLKey_SDLK_DELETE as isize,
    World0 = ll::SDLKey_SDLK_WORLD_0 as isize,
    World1 = ll::SDLKey_SDLK_WORLD_1 as isize,
    World2 = ll::SDLKey_SDLK_WORLD_2 as isize,
    World3 = ll::SDLKey_SDLK_WORLD_3 as isize,
    World4 = ll::SDLKey_SDLK_WORLD_4 as isize,
    World5 = ll::SDLKey_SDLK_WORLD_5 as isize,
    World6 = ll::SDLKey_SDLK_WORLD_6 as isize,
    World7 = ll::SDLKey_SDLK_WORLD_7 as isize,
    World8 = ll::SDLKey_SDLK_WORLD_8 as isize,
    World9 = ll::SDLKey_SDLK_WORLD_9 as isize,
    World10 = ll::SDLKey_SDLK_WORLD_10 as isize,
    World11 = ll::SDLKey_SDLK_WORLD_11 as isize,
    World12 = ll::SDLKey_SDLK_WORLD_12 as isize,
    World13 = ll::SDLKey_SDLK_WORLD_13 as isize,
    World14 = ll::SDLKey_SDLK_WORLD_14 as isize,
    World15 = ll::SDLKey_SDLK_WORLD_15 as isize,
    World16 = ll::SDLKey_SDLK_WORLD_16 as isize,
    World17 = ll::SDLKey_SDLK_WORLD_17 as isize,
    World18 = ll::SDLKey_SDLK_WORLD_18 as isize,
    World19 = ll::SDLKey_SDLK_WORLD_19 as isize,
    World20 = ll::SDLKey_SDLK_WORLD_20 as isize,
    World21 = ll::SDLKey_SDLK_WORLD_21 as isize,
    World22 = ll::SDLKey_SDLK_WORLD_22 as isize,
    World23 = ll::SDLKey_SDLK_WORLD_23 as isize,
    World24 = ll::SDLKey_SDLK_WORLD_24 as isize,
    World25 = ll::SDLKey_SDLK_WORLD_25 as isize,
    World26 = ll::SDLKey_SDLK_WORLD_26 as isize,
    World27 = ll::SDLKey_SDLK_WORLD_27 as isize,
    World28 = ll::SDLKey_SDLK_WORLD_28 as isize,
    World29 = ll::SDLKey_SDLK_WORLD_29 as isize,
    World30 = ll::SDLKey_SDLK_WORLD_30 as isize,
    World31 = ll::SDLKey_SDLK_WORLD_31 as isize,
    World32 = ll::SDLKey_SDLK_WORLD_32 as isize,
    World33 = ll::SDLKey_SDLK_WORLD_33 as isize,
    World34 = ll::SDLKey_SDLK_WORLD_34 as isize,
    World35 = ll::SDLKey_SDLK_WORLD_35 as isize,
    World36 = ll::SDLKey_SDLK_WORLD_36 as isize,
    World37 = ll::SDLKey_SDLK_WORLD_37 as isize,
    World38 = ll::SDLKey_SDLK_WORLD_38 as isize,
    World39 = ll::SDLKey_SDLK_WORLD_39 as isize,
    World40 = ll::SDLKey_SDLK_WORLD_40 as isize,
    World41 = ll::SDLKey_SDLK_WORLD_41 as isize,
    World42 = ll::SDLKey_SDLK_WORLD_42 as isize,
    World43 = ll::SDLKey_SDLK_WORLD_43 as isize,
    World44 = ll::SDLKey_SDLK_WORLD_44 as isize,
    World45 = ll::SDLKey_SDLK_WORLD_45 as isize,
    World46 = ll::SDLKey_SDLK_WORLD_46 as isize,
    World47 = ll::SDLKey_SDLK_WORLD_47 as isize,
    World48 = ll::SDLKey_SDLK_WORLD_48 as isize,
    World49 = ll::SDLKey_SDLK_WORLD_49 as isize,
    World50 = ll::SDLKey_SDLK_WORLD_50 as isize,
    World51 = ll::SDLKey_SDLK_WORLD_51 as isize,
    World52 = ll::SDLKey_SDLK_WORLD_52 as isize,
    World53 = ll::SDLKey_SDLK_WORLD_53 as isize,
    World54 = ll::SDLKey_SDLK_WORLD_54 as isize,
    World55 = ll::SDLKey_SDLK_WORLD_55 as isize,
    World56 = ll::SDLKey_SDLK_WORLD_56 as isize,
    World57 = ll::SDLKey_SDLK_WORLD_57 as isize,
    World58 = ll::SDLKey_SDLK_WORLD_58 as isize,
    World59 = ll::SDLKey_SDLK_WORLD_59 as isize,
    World60 = ll::SDLKey_SDLK_WORLD_60 as isize,
    World61 = ll::SDLKey_SDLK_WORLD_61 as isize,
    World62 = ll::SDLKey_SDLK_WORLD_62 as isize,
    World63 = ll::SDLKey_SDLK_WORLD_63 as isize,
    World64 = ll::SDLKey_SDLK_WORLD_64 as isize,
    World65 = ll::SDLKey_SDLK_WORLD_65 as isize,
    World66 = ll::SDLKey_SDLK_WORLD_66 as isize,
    World67 = ll::SDLKey_SDLK_WORLD_67 as isize,
    World68 = ll::SDLKey_SDLK_WORLD_68 as isize,
    World69 = ll::SDLKey_SDLK_WORLD_69 as isize,
    World70 = ll::SDLKey_SDLK_WORLD_70 as isize,
    World71 = ll::SDLKey_SDLK_WORLD_71 as isize,
    World72 = ll::SDLKey_SDLK_WORLD_72 as isize,
    World73 = ll::SDLKey_SDLK_WORLD_73 as isize,
    World74 = ll::SDLKey_SDLK_WORLD_74 as isize,
    World75 = ll::SDLKey_SDLK_WORLD_75 as isize,
    World76 = ll::SDLKey_SDLK_WORLD_76 as isize,
    World77 = ll::SDLKey_SDLK_WORLD_77 as isize,
    World78 = ll::SDLKey_SDLK_WORLD_78 as isize,
    World79 = ll::SDLKey_SDLK_WORLD_79 as isize,
    World80 = ll::SDLKey_SDLK_WORLD_80 as isize,
    World81 = ll::SDLKey_SDLK_WORLD_81 as isize,
    World82 = ll::SDLKey_SDLK_WORLD_82 as isize,
    World83 = ll::SDLKey_SDLK_WORLD_83 as isize,
    World84 = ll::SDLKey_SDLK_WORLD_84 as isize,
    World85 = ll::SDLKey_SDLK_WORLD_85 as isize,
    World86 = ll::SDLKey_SDLK_WORLD_86 as isize,
    World87 = ll::SDLKey_SDLK_WORLD_87 as isize,
    World88 = ll::SDLKey_SDLK_WORLD_88 as isize,
    World89 = ll::SDLKey_SDLK_WORLD_89 as isize,
    World90 = ll::SDLKey_SDLK_WORLD_90 as isize,
    World91 = ll::SDLKey_SDLK_WORLD_91 as isize,
    World92 = ll::SDLKey_SDLK_WORLD_92 as isize,
    World93 = ll::SDLKey_SDLK_WORLD_93 as isize,
    World94 = ll::SDLKey_SDLK_WORLD_94 as isize,
    World95 = ll::SDLKey_SDLK_WORLD_95 as isize,
    KeyPad0 = ll::SDLKey_SDLK_KP0 as isize,
    KeyPad1 = ll::SDLKey_SDLK_KP1 as isize,
    KeyPad2 = ll::SDLKey_SDLK_KP2 as isize,
    KeyPad3 = ll::SDLKey_SDLK_KP3 as isize,
    KeyPad4 = ll::SDLKey_SDLK_KP4 as isize,
    KeyPad5 = ll::SDLKey_SDLK_KP5 as isize,
    KeyPad6 = ll::SDLKey_SDLK_KP6 as isize,
    KeyPad7 = ll::SDLKey_SDLK_KP7 as isize,
    KeyPad8 = ll::SDLKey_SDLK_KP8 as isize,
    KeyPad9 = ll::SDLKey_SDLK_KP9 as isize,
    KeyPadPeriod = ll::SDLKey_SDLK_KP_PERIOD as isize,
    KeyPadDivide = ll::SDLKey_SDLK_KP_DIVIDE as isize,
    KeyPadMultiply = ll::SDLKey_SDLK_KP_MULTIPLY as isize,
    KeyPadMinus = ll::SDLKey_SDLK_KP_MINUS as isize,
    KeyPadPlus = ll::SDLKey_SDLK_KP_PLUS as isize,
    KeyPadEnter = ll::SDLKey_SDLK_KP_ENTER as isize,
    KeyPadEquals = ll::SDLKey_SDLK_KP_EQUALS as isize,
    Up = ll::SDLKey_SDLK_UP as isize,
    Down = ll::SDLKey_SDLK_DOWN as isize,
    Right = ll::SDLKey_SDLK_RIGHT as isize,
    Left = ll::SDLKey_SDLK_LEFT as isize,
    Insert = ll::SDLKey_SDLK_INSERT as isize,
    Home = ll::SDLKey_SDLK_HOME as isize,
    End = ll::SDLKey_SDLK_END as isize,
    PageUp = ll::SDLKey_SDLK_PAGEUP as isize,
    PageDown = ll::SDLKey_SDLK_PAGEDOWN as isize,
    F1 = ll::SDLKey_SDLK_F1 as isize,
    F2 = ll::SDLKey_SDLK_F2 as isize,
    F3 = ll::SDLKey_SDLK_F3 as isize,
    F4 = ll::SDLKey_SDLK_F4 as isize,
    F5 = ll::SDLKey_SDLK_F5 as isize,
    F6 = ll::SDLKey_SDLK_F6 as isize,
    F7 = ll::SDLKey_SDLK_F7 as isize,
    F8 = ll::SDLKey_SDLK_F8 as isize,
    F9 = ll::SDLKey_SDLK_F9 as isize,
    F10 = ll::SDLKey_SDLK_F10 as isize,
    F11 = ll::SDLKey_SDLK_F11 as isize,
    F12 = ll::SDLKey_SDLK_F12 as isize,
    F13 = ll::SDLKey_SDLK_F13 as isize,
    F14 = ll::SDLKey_SDLK_F14 as isize,
    F15 = ll::SDLKey_SDLK_F15 as isize,
    NumLock = ll::SDLKey_SDLK_NUMLOCK as isize,
    CapsLock = ll::SDLKey_SDLK_CAPSLOCK as isize,
    ScrollLock = ll::SDLKey_SDLK_SCROLLOCK as isize,
    RightShift = ll::SDLKey_SDLK_RSHIFT as isize,
    LeftShift = ll::SDLKey_SDLK_LSHIFT as isize,
    RightCtrl = ll::SDLKey_SDLK_RCTRL as isize,
    LeftCtrl = ll::SDLKey_SDLK_LCTRL as isize,
    RightAlt = ll::SDLKey_SDLK_RALT as isize,
    LeftAlt = ll::SDLKey_SDLK_LALT as isize,
    RightMeta = ll::SDLKey_SDLK_RMETA as isize,
    LeftMeta = ll::SDLKey_SDLK_LMETA as isize,
    /// Left "Windows" key
    LeftSuper = ll::SDLKey_SDLK_LSUPER as isize,
    /// Right "Windows" key
    RightSuper = ll::SDLKey_SDLK_RSUPER as isize,
    /// "Alt Gr" key
    Mode = ll::SDLKey_SDLK_MODE as isize,
    /// Multi-key compose key
    Compose = ll::SDLKey_SDLK_COMPOSE as isize,
    Help = ll::SDLKey_SDLK_HELP as isize,
    Print = ll::SDLKey_SDLK_PRINT as isize,
    SysReq = ll::SDLKey_SDLK_SYSREQ as isize,
    Break = ll::SDLKey_SDLK_BREAK as isize,
    Menu = ll::SDLKey_SDLK_MENU as isize,
    /// Power Macintosh power key
    Power = ll::SDLKey_SDLK_POWER as isize,
    /// Some european keyboards
    Euro = ll::SDLKey_SDLK_EURO as isize,
    /// Atari keyboard has Undo
    Undo = ll::SDLKey_SDLK_UNDO as isize,
}

impl TryFrom<isize> for KeyCode {
    type Error = String;

    fn try_from(input: isize) -> Result<KeyCode, String> {
        use KeyCode as K;
        match input {
            i if i == K::BackSpace as isize => Ok(K::BackSpace),
            i if i == K::Tab as isize => Ok(K::Tab),
            i if i == K::Clear as isize => Ok(K::Clear),
            i if i == K::Return as isize => Ok(K::Return),
            i if i == K::Pause as isize => Ok(K::Pause),
            i if i == K::Escape as isize => Ok(K::Escape),
            i if i == K::Space as isize => Ok(K::Space),
            i if i == K::Exclaim as isize => Ok(K::Exclaim),
            i if i == K::QuoteDouble as isize => Ok(K::QuoteDouble),
            i if i == K::Hash as isize => Ok(K::Hash),
            i if i == K::Dollar as isize => Ok(K::Dollar),
            i if i == K::Ampersand as isize => Ok(K::Ampersand),
            i if i == K::Quote as isize => Ok(K::Quote),
            i if i == K::LeftParen as isize => Ok(K::LeftParen),
            i if i == K::RightParen as isize => Ok(K::RightParen),
            i if i == K::Asterisk as isize => Ok(K::Asterisk),
            i if i == K::Plus as isize => Ok(K::Plus),
            i if i == K::Comma as isize => Ok(K::Comma),
            i if i == K::Minus as isize => Ok(K::Minus),
            i if i == K::Period as isize => Ok(K::Period),
            i if i == K::Slash as isize => Ok(K::Slash),
            i if i == K::Num0 as isize => Ok(K::Num0),
            i if i == K::Num1 as isize => Ok(K::Num1),
            i if i == K::Num2 as isize => Ok(K::Num2),
            i if i == K::Num3 as isize => Ok(K::Num3),
            i if i == K::Num4 as isize => Ok(K::Num4),
            i if i == K::Num5 as isize => Ok(K::Num5),
            i if i == K::Num6 as isize => Ok(K::Num6),
            i if i == K::Num7 as isize => Ok(K::Num7),
            i if i == K::Num8 as isize => Ok(K::Num8),
            i if i == K::Num9 as isize => Ok(K::Num9),
            i if i == K::Colon as isize => Ok(K::Colon),
            i if i == K::Semicolon as isize => Ok(K::Semicolon),
            i if i == K::Less as isize => Ok(K::Less),
            i if i == K::Equals as isize => Ok(K::Equals),
            i if i == K::Greater as isize => Ok(K::Greater),
            i if i == K::Question as isize => Ok(K::Question),
            i if i == K::At as isize => Ok(K::At),
            i if i == K::LeftBracket as isize => Ok(K::LeftBracket),
            i if i == K::Backslash as isize => Ok(K::Backslash),
            i if i == K::RightBracket as isize => Ok(K::RightBracket),
            i if i == K::Caret as isize => Ok(K::Caret),
            i if i == K::Underscore as isize => Ok(K::Underscore),
            i if i == K::Backquote as isize => Ok(K::Backquote),
            i if i == K::A as isize => Ok(K::A),
            i if i == K::B as isize => Ok(K::B),
            i if i == K::C as isize => Ok(K::C),
            i if i == K::D as isize => Ok(K::D),
            i if i == K::E as isize => Ok(K::E),
            i if i == K::F as isize => Ok(K::F),
            i if i == K::G as isize => Ok(K::G),
            i if i == K::H as isize => Ok(K::H),
            i if i == K::I as isize => Ok(K::I),
            i if i == K::J as isize => Ok(K::J),
            i if i == K::K as isize => Ok(K::K),
            i if i == K::L as isize => Ok(K::L),
            i if i == K::M as isize => Ok(K::M),
            i if i == K::N as isize => Ok(K::N),
            i if i == K::O as isize => Ok(K::O),
            i if i == K::P as isize => Ok(K::P),
            i if i == K::Q as isize => Ok(K::Q),
            i if i == K::R as isize => Ok(K::R),
            i if i == K::S as isize => Ok(K::S),
            i if i == K::T as isize => Ok(K::T),
            i if i == K::U as isize => Ok(K::U),
            i if i == K::V as isize => Ok(K::V),
            i if i == K::W as isize => Ok(K::W),
            i if i == K::X as isize => Ok(K::X),
            i if i == K::Y as isize => Ok(K::Y),
            i if i == K::Z as isize => Ok(K::Z),
            i if i == K::Delete as isize => Ok(K::Delete),
            i if i == K::World0 as isize => Ok(K::World0),
            i if i == K::World1 as isize => Ok(K::World1),
            i if i == K::World2 as isize => Ok(K::World2),
            i if i == K::World3 as isize => Ok(K::World3),
            i if i == K::World4 as isize => Ok(K::World4),
            i if i == K::World5 as isize => Ok(K::World5),
            i if i == K::World6 as isize => Ok(K::World6),
            i if i == K::World7 as isize => Ok(K::World7),
            i if i == K::World8 as isize => Ok(K::World8),
            i if i == K::World9 as isize => Ok(K::World9),
            i if i == K::World10 as isize => Ok(K::World10),
            i if i == K::World11 as isize => Ok(K::World11),
            i if i == K::World12 as isize => Ok(K::World12),
            i if i == K::World13 as isize => Ok(K::World13),
            i if i == K::World14 as isize => Ok(K::World14),
            i if i == K::World15 as isize => Ok(K::World15),
            i if i == K::World16 as isize => Ok(K::World16),
            i if i == K::World17 as isize => Ok(K::World17),
            i if i == K::World18 as isize => Ok(K::World18),
            i if i == K::World19 as isize => Ok(K::World19),
            i if i == K::World20 as isize => Ok(K::World20),
            i if i == K::World21 as isize => Ok(K::World21),
            i if i == K::World22 as isize => Ok(K::World22),
            i if i == K::World23 as isize => Ok(K::World23),
            i if i == K::World24 as isize => Ok(K::World24),
            i if i == K::World25 as isize => Ok(K::World25),
            i if i == K::World26 as isize => Ok(K::World26),
            i if i == K::World27 as isize => Ok(K::World27),
            i if i == K::World28 as isize => Ok(K::World28),
            i if i == K::World29 as isize => Ok(K::World29),
            i if i == K::World30 as isize => Ok(K::World30),
            i if i == K::World31 as isize => Ok(K::World31),
            i if i == K::World32 as isize => Ok(K::World32),
            i if i == K::World33 as isize => Ok(K::World33),
            i if i == K::World34 as isize => Ok(K::World34),
            i if i == K::World35 as isize => Ok(K::World35),
            i if i == K::World36 as isize => Ok(K::World36),
            i if i == K::World37 as isize => Ok(K::World37),
            i if i == K::World38 as isize => Ok(K::World38),
            i if i == K::World39 as isize => Ok(K::World39),
            i if i == K::World40 as isize => Ok(K::World40),
            i if i == K::World41 as isize => Ok(K::World41),
            i if i == K::World42 as isize => Ok(K::World42),
            i if i == K::World43 as isize => Ok(K::World43),
            i if i == K::World44 as isize => Ok(K::World44),
            i if i == K::World45 as isize => Ok(K::World45),
            i if i == K::World46 as isize => Ok(K::World46),
            i if i == K::World47 as isize => Ok(K::World47),
            i if i == K::World48 as isize => Ok(K::World48),
            i if i == K::World49 as isize => Ok(K::World49),
            i if i == K::World50 as isize => Ok(K::World50),
            i if i == K::World51 as isize => Ok(K::World51),
            i if i == K::World52 as isize => Ok(K::World52),
            i if i == K::World53 as isize => Ok(K::World53),
            i if i == K::World54 as isize => Ok(K::World54),
            i if i == K::World55 as isize => Ok(K::World55),
            i if i == K::World56 as isize => Ok(K::World56),
            i if i == K::World57 as isize => Ok(K::World57),
            i if i == K::World58 as isize => Ok(K::World58),
            i if i == K::World59 as isize => Ok(K::World59),
            i if i == K::World60 as isize => Ok(K::World60),
            i if i == K::World61 as isize => Ok(K::World61),
            i if i == K::World62 as isize => Ok(K::World62),
            i if i == K::World63 as isize => Ok(K::World63),
            i if i == K::World64 as isize => Ok(K::World64),
            i if i == K::World65 as isize => Ok(K::World65),
            i if i == K::World66 as isize => Ok(K::World66),
            i if i == K::World67 as isize => Ok(K::World67),
            i if i == K::World68 as isize => Ok(K::World68),
            i if i == K::World69 as isize => Ok(K::World69),
            i if i == K::World70 as isize => Ok(K::World70),
            i if i == K::World71 as isize => Ok(K::World71),
            i if i == K::World72 as isize => Ok(K::World72),
            i if i == K::World73 as isize => Ok(K::World73),
            i if i == K::World74 as isize => Ok(K::World74),
            i if i == K::World75 as isize => Ok(K::World75),
            i if i == K::World76 as isize => Ok(K::World76),
            i if i == K::World77 as isize => Ok(K::World77),
            i if i == K::World78 as isize => Ok(K::World78),
            i if i == K::World79 as isize => Ok(K::World79),
            i if i == K::World80 as isize => Ok(K::World80),
            i if i == K::World81 as isize => Ok(K::World81),
            i if i == K::World82 as isize => Ok(K::World82),
            i if i == K::World83 as isize => Ok(K::World83),
            i if i == K::World84 as isize => Ok(K::World84),
            i if i == K::World85 as isize => Ok(K::World85),
            i if i == K::World86 as isize => Ok(K::World86),
            i if i == K::World87 as isize => Ok(K::World87),
            i if i == K::World88 as isize => Ok(K::World88),
            i if i == K::World89 as isize => Ok(K::World89),
            i if i == K::World90 as isize => Ok(K::World90),
            i if i == K::World91 as isize => Ok(K::World91),
            i if i == K::World92 as isize => Ok(K::World92),
            i if i == K::World93 as isize => Ok(K::World93),
            i if i == K::World94 as isize => Ok(K::World94),
            i if i == K::World95 as isize => Ok(K::World95),
            i if i == K::KeyPad0 as isize => Ok(K::KeyPad0),
            i if i == K::KeyPad1 as isize => Ok(K::KeyPad1),
            i if i == K::KeyPad2 as isize => Ok(K::KeyPad2),
            i if i == K::KeyPad3 as isize => Ok(K::KeyPad3),
            i if i == K::KeyPad4 as isize => Ok(K::KeyPad4),
            i if i == K::KeyPad5 as isize => Ok(K::KeyPad5),
            i if i == K::KeyPad6 as isize => Ok(K::KeyPad6),
            i if i == K::KeyPad7 as isize => Ok(K::KeyPad7),
            i if i == K::KeyPad8 as isize => Ok(K::KeyPad8),
            i if i == K::KeyPad9 as isize => Ok(K::KeyPad9),
            i if i == K::KeyPadPeriod as isize => Ok(K::KeyPadPeriod),
            i if i == K::KeyPadDivide as isize => Ok(K::KeyPadDivide),
            i if i == K::KeyPadMultiply as isize => Ok(K::KeyPadMultiply),
            i if i == K::KeyPadMinus as isize => Ok(K::KeyPadMinus),
            i if i == K::KeyPadPlus as isize => Ok(K::KeyPadPlus),
            i if i == K::KeyPadEnter as isize => Ok(K::KeyPadEnter),
            i if i == K::KeyPadEquals as isize => Ok(K::KeyPadEquals),
            i if i == K::Up as isize => Ok(K::Up),
            i if i == K::Down as isize => Ok(K::Down),
            i if i == K::Right as isize => Ok(K::Right),
            i if i == K::Left as isize => Ok(K::Left),
            i if i == K::Insert as isize => Ok(K::Insert),
            i if i == K::Home as isize => Ok(K::Home),
            i if i == K::End as isize => Ok(K::End),
            i if i == K::PageUp as isize => Ok(K::PageUp),
            i if i == K::PageDown as isize => Ok(K::PageDown),
            i if i == K::F1 as isize => Ok(K::F1),
            i if i == K::F2 as isize => Ok(K::F2),
            i if i == K::F3 as isize => Ok(K::F3),
            i if i == K::F4 as isize => Ok(K::F4),
            i if i == K::F5 as isize => Ok(K::F5),
            i if i == K::F6 as isize => Ok(K::F6),
            i if i == K::F7 as isize => Ok(K::F7),
            i if i == K::F8 as isize => Ok(K::F8),
            i if i == K::F9 as isize => Ok(K::F9),
            i if i == K::F10 as isize => Ok(K::F10),
            i if i == K::F11 as isize => Ok(K::F11),
            i if i == K::F12 as isize => Ok(K::F12),
            i if i == K::F13 as isize => Ok(K::F13),
            i if i == K::F14 as isize => Ok(K::F14),
            i if i == K::F15 as isize => Ok(K::F15),
            i if i == K::NumLock as isize => Ok(K::NumLock),
            i if i == K::CapsLock as isize => Ok(K::CapsLock),
            i if i == K::ScrollLock as isize => Ok(K::ScrollLock),
            i if i == K::RightShift as isize => Ok(K::RightShift),
            i if i == K::LeftShift as isize => Ok(K::LeftShift),
            i if i == K::RightCtrl as isize => Ok(K::RightCtrl),
            i if i == K::LeftCtrl as isize => Ok(K::LeftCtrl),
            i if i == K::RightAlt as isize => Ok(K::RightAlt),
            i if i == K::LeftAlt as isize => Ok(K::LeftAlt),
            i if i == K::RightMeta as isize => Ok(K::RightMeta),
            i if i == K::LeftMeta as isize => Ok(K::LeftMeta),
            i if i == K::LeftSuper as isize => Ok(K::LeftSuper),
            i if i == K::RightSuper as isize => Ok(K::RightSuper),
            i if i == K::Mode as isize => Ok(K::Mode),
            i if i == K::Compose as isize => Ok(K::Compose),
            i if i == K::Help as isize => Ok(K::Help),
            i if i == K::Print as isize => Ok(K::Print),
            i if i == K::SysReq as isize => Ok(K::SysReq),
            i if i == K::Break as isize => Ok(K::Break),
            i if i == K::Menu as isize => Ok(K::Menu),
            i if i == K::Power as isize => Ok(K::Power),
            i if i == K::Euro as isize => Ok(K::Euro),
            i if i == K::Undo as isize => Ok(K::Undo),

            i => Err(format!("No key with index {}", i)),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Ord, PartialOrd)]
pub enum Modifier {
    LeftShift = 0x0001,
    RightShift = 0x0002,
    LeftCtrl = 0x0040,
    RightCtrl = 0x0080,
    LeftAlt = 0x0100,
    RightAlt = 0x0200,
    LeftMeta = 0x0400,
    RightMeta = 0x0800,
    Num = 0x1000,
    Caps = 0x2000,
    Mode = 0x4000,
}

fn u32_to_modifiers(raw: u32) -> BTreeSet<Modifier> {
    let mut modifiers = BTreeSet::new();
    if Modifier::LeftShift as u32 & raw != 0 {
        modifiers.insert(Modifier::LeftShift);
    }
    if Modifier::RightShift as u32 & raw != 0 {
        modifiers.insert(Modifier::RightShift);
    }
    if Modifier::LeftCtrl as u32 & raw != 0 {
        modifiers.insert(Modifier::LeftCtrl);
    }
    if Modifier::RightCtrl as u32 & raw != 0 {
        modifiers.insert(Modifier::RightCtrl);
    }
    if Modifier::LeftAlt as u32 & raw != 0 {
        modifiers.insert(Modifier::LeftAlt);
    }
    if Modifier::RightAlt as u32 & raw != 0 {
        modifiers.insert(Modifier::RightAlt);
    }
    if Modifier::LeftMeta as u32 & raw != 0 {
        modifiers.insert(Modifier::LeftMeta);
    }
    if Modifier::RightMeta as u32 & raw != 0 {
        modifiers.insert(Modifier::RightMeta);
    }
    if Modifier::Num as u32 & raw != 0 {
        modifiers.insert(Modifier::Num);
    }
    if Modifier::Caps as u32 & raw != 0 {
        modifiers.insert(Modifier::Caps);
    }
    if Modifier::Mode as u32 & raw != 0 {
        modifiers.insert(Modifier::Mode);
    }
    modifiers
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd)]
#[repr(u8)]
pub enum MouseButton {
    Left = 1,
    Middle = 2,
    Right = 3,
    WheelUp = 4,
    WheelDown = 5,
    X1 = 6,
    X2 = 7,
}

fn u8_flags_to_mouse_buttons(raw: u8) -> BTreeSet<MouseButton> {
    let mut mouse_buttons = BTreeSet::new();
    if 1 << ((MouseButton::Left as u8) - 1) & raw != 0 {
        mouse_buttons.insert(MouseButton::Left);
    }
    if 1 << ((MouseButton::Middle as u8) - 1) & raw != 0 {
        mouse_buttons.insert(MouseButton::Middle);
    }
    if 1 << ((MouseButton::Right as u8) - 1) & raw != 0 {
        mouse_buttons.insert(MouseButton::Right);
    }
    if 1 << ((MouseButton::WheelUp as u8) - 1) & raw != 0 {
        mouse_buttons.insert(MouseButton::WheelUp);
    }
    if 1 << ((MouseButton::WheelDown as u8) - 1) & raw != 0 {
        mouse_buttons.insert(MouseButton::WheelDown);
    }
    if 1 << ((MouseButton::X1 as u8) - 1) & raw != 0 {
        mouse_buttons.insert(MouseButton::X1);
    }
    if 1 << ((MouseButton::X2 as u8) - 1) & raw != 0 {
        mouse_buttons.insert(MouseButton::X2);
    }
    mouse_buttons
}

fn u8_to_mouse_button(raw: u8) -> Option<MouseButton> {
    match raw {
        r if r == MouseButton::Left as u8 => Some(MouseButton::Left),
        r if r == MouseButton::Middle as u8 => Some(MouseButton::Middle),
        r if r == MouseButton::Right as u8 => Some(MouseButton::Right),
        r if r == MouseButton::WheelUp as u8 => Some(MouseButton::WheelUp),
        r if r == MouseButton::WheelDown as u8 => {
            Some(MouseButton::WheelDown)
        }
        r if r == MouseButton::X1 as u8 => Some(MouseButton::X1),
        r if r == MouseButton::X2 as u8 => Some(MouseButton::X2),
        _ => None,
    }
}
