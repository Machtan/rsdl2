use sdl2_sys as sys;
use std::mem;
use libc::c_void;
use init::InitGuard;
use std::ffi::CStr;
use keyboard::{Keycode, Scancode};
use keymod::Keymod;
use common::{Error, get_error};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppEvent {
    WillEnterBackground,
    DidEnterBackground,
    WillEnterForeground,
    DidEnterForeground,
    Terminating,
    LowMemory,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AudioDeviceKind {
    Output,
    Capture(u8), // zero if an output device, non-zero if a capture device.
}

#[derive(Debug, Clone)]
pub enum EventKind {
    App(AppEvent),

    AudioDeviceAdded {
        device_index: u32,
        device: AudioDeviceKind,
    },
    AudioDeviceRemoved {
        device_id: u32,
        device: AudioDeviceKind,
    },

    MouseButtonDown(MouseButtonEvent),
    MouseButtonUp(MouseButtonEvent),
    MouseMotion(MouseMotionEvent),
    MouseWheel(MouseWheelEvent),

    FingerDown(TouchFingerEvent),
    FingerUp(TouchFingerEvent),
    FingerMotion {
        event: TouchFingerEvent,
        dx: f32,
        dy: f32,
    },
    DollarRecord(DollarGestureEvent),
    DollarGesture(DollarGestureEvent),
    MultiGesture(MultiGestureEvent),

    KeyDown(Keysym),
    KeyRepeat(Keysym),
    KeyUp(Keysym),
    KeymapChanged,
    TextEditing(TextEditingEvent),
    TextInput(String),

    ControllerDeviceAdded { device_index: i32 },
    Controller {
        instance_id: i32,
        event: ControllerEvent,
    },

    JoyDeviceAdded { device_index: i32 },
    Joy { instance_id: i32, event: JoyEvent },

    Window(WindowEvent),
    // SysWmEvent(SysWmEventData), // Disabled by default
    DropFile(String),

    ClipboardUpdate,
    RenderDeviceReset,
    RenderTargetsReset,
    Quit,

    User(UserEvent),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseKind {
    Mouse(u32),
    Touch,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseButton {
    Whatever,
}

impl MouseButton {
    pub fn from_raw(_button: u8) -> Option<MouseButton> {
        Some(MouseButton::Whatever)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseButtonEvent {
    // pub window_id: u32, // The window with mouse focus, if any
    pub which: MouseKind, // The mouse instance id, or SDL_TOUCH_MOUSEID
    pub button: MouseButton, // The mouse button index
    pub clicks: u8, // 1 for single-click, 2 for double-click, etc.
    pub x: i32, // X coordinate, relative to window
    pub y: i32, // Y coordinate, relative to window
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseMotionEvent {
    // pub window_id: u32, // The window with mouse focus, if any
    pub which: MouseKind, // The mouse instance id, or SDL_TOUCH_MOUSEID
    pub held_buttons_flags: u32, // The current button state
    pub x: i32, // X coordinate, relative to window
    pub y: i32, // Y coordinate, relative to window
    pub xrel: i32, // The relative motion in the X direction
    pub yrel: i32, // The relative motion in the Y direction
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseWheelEvent {
    // pub window_id: u32, // The window with mouse focus, if any
    pub which: MouseKind, // The mouse instance id, or SDL_TOUCH_MOUSEID
    pub x: i32, // The amount scrolled horizontally, positive to the right and negative to the left
    pub y: i32, /* The amount scrolled vertically, positive away from the user and negative toward the user */
    pub direction: u32, /* Set to one of the SDL_MOUSEWHEEL_* defines. When FLIPPED the values in X and Y will be opposite. Multiply by -1 to change them back */
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TouchFingerEvent {
    pub touch_id: i64, // The touch device id
    pub finger_id: i64,
    pub x: f32, // Normalized in the range 0...1
    pub y: f32, // Normalized in the range 0...1
    pub pressure: f32, // Normalized in the range 0...1
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DollarGestureEvent {
    pub touch_id: i64, // The touch device id
    pub gesture_id: i64,
    pub num_fingers: u32,
    pub error: f32,
    pub x: f32, // Normalized center of gesture
    pub y: f32, // Normalized center of gesture
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MultiGestureEvent {
    pub touch_id: i64, // The touch device index
    pub d_theta: f32,
    pub d_dist: f32,
    pub x: f32,
    pub y: f32,
    pub num_fingers: u16,
}

// typedef struct SDL_Keysym
// {
// SDL_Scancode scancode;      /* SDL physical key code - see ::SDL_Scancode for details */
// SDL_Keycode sym;            /* SDL virtual key code - see ::SDL_Keycode for details */
// Uint16 mod;                 /* current key modifiers */
// Uint32 unused;
// } SDL_Keysym;
//

#[derive(Debug, Clone, Copy)]
pub struct Keysym {
    pub scancode: Scancode,
    pub keycode: Keycode,
    pub mods: Keymod,
}

impl Keysym {
    fn from_raw(raw: sys::SDL_Keysym) -> Keysym {
        Keysym {
            scancode: Scancode::from_value(raw.scancode).expect("Invalid scancode"),
            keycode: Keycode::from_value(raw.sym).expect("Invalid keycode"),
            mods: Keymod::from_bits(raw._mod).expect("Invalid modifiers"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextEditingEvent {
    // pub window_id: u32, // The window with keyboard focus, if any
    pub text: String, // The editing text
    pub start: usize, // The start cursor of selected editing text
    pub length: usize, // The length of selected editing text
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ControllerButton {
    Whatever,
}

impl ControllerButton {
    pub fn from_raw(_raw: u8) -> Option<ControllerButton> {
        Some(ControllerButton::Whatever)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ControllerAxis {
    Whatever,
}

impl ControllerAxis {
    pub fn from_raw(_raw: u8) -> Option<ControllerAxis> {
        Some(ControllerAxis::Whatever)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ControllerEvent {
    ButtonDown(ControllerButton),
    ButtonUp(ControllerButton),
    AxisMotion(ControllerAxis, i16),
    Removed,
    Remapped,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JoyButton {
    Whatever,
}

impl JoyButton {
    pub fn from_raw(_raw: u8) -> Option<JoyButton> {
        Some(JoyButton::Whatever)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JoyAxis {
    Whatever,
}

impl JoyAxis {
    pub fn from_raw(_raw: u8) -> Option<JoyAxis> {
        Some(JoyAxis::Whatever)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JoyHatPosition {
    Whatever,
}

impl JoyHatPosition {
    pub fn from_raw(_raw: u8) -> Option<JoyHatPosition> {
        Some(JoyHatPosition::Whatever)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JoyEvent {
    ButtonDown(JoyButton),
    ButtonUp(JoyButton),
    BallMotion { ball: u8, xrel: i16, yrel: i16 },
    HatMotion { hat: u8, pos: JoyHatPosition },
    AxisMotion(JoyAxis, i16),
    Removed,
}

// TODO: Wrap properly
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowEvent {
    // pub window_id: u32, // The associated window
    pub event: u8, // ::SDL_WindowEventID
    pub data1: i32, // event dependent data
    pub data2: i32, // event dependent data
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UserEvent {
    pub kind: u32, // ::SDL_USEREVENT through ::SDL_LASTEVENT-1
    pub code: i32, // User defined event code
    pub data1: *mut c_void, // User defined data pointer
    pub data2: *mut c_void, // User defined data pointer
}

#[derive(Debug, Clone)]
pub struct Event {
    pub kind: EventKind,
    pub timestamp: u32,
    pub window_id: Option<u32>,
}

unsafe fn wrap_event(mut event: sys::SDL_Event) -> Event {
    use self::EventKind::*;
    let mut window_id = None;
    let (type_, timestamp) = {
        let common = *event.common();
        (common.type_, common.timestamp)
    };
    let kind = match type_ {
        sys::SDL_FIRSTEVENT => unreachable!(),
        sys::SDL_APP_WILLENTERBACKGROUND => App(AppEvent::WillEnterBackground),
        sys::SDL_APP_DIDENTERBACKGROUND => App(AppEvent::DidEnterBackground),
        sys::SDL_APP_WILLENTERFOREGROUND => App(AppEvent::WillEnterForeground),
        sys::SDL_APP_DIDENTERFOREGROUND => App(AppEvent::DidEnterForeground),
        sys::SDL_APP_TERMINATING => App(AppEvent::Terminating),
        sys::SDL_APP_LOWMEMORY => App(AppEvent::LowMemory),
        sys::SDL_AUDIODEVICEADDED => {
            let raw = *event.adevice();
            AudioDeviceAdded {
                device_index: raw.which,
                device: if raw.iscapture == 0 {
                    AudioDeviceKind::Output
                } else {
                    AudioDeviceKind::Capture(raw.iscapture)
                },
            }
        }
        sys::SDL_AUDIODEVICEREMOVED => {
            let raw = *event.adevice();
            AudioDeviceRemoved {
                device_id: raw.which,
                device: if raw.iscapture == 0 {
                    AudioDeviceKind::Output
                } else {
                    AudioDeviceKind::Capture(raw.iscapture)
                },
            }
        }
        sys::SDL_MOUSEBUTTONDOWN => {
            let raw = *event.button();
            window_id = Some(raw.windowID);
            MouseButtonDown(MouseButtonEvent {
                which: if raw.which == sys::SDL_TOUCH_MOUSEID {
                    MouseKind::Touch
                } else {
                    MouseKind::Mouse(raw.which)
                },
                button: MouseButton::from_raw(raw.button).expect("Invalid mouse button"),
                clicks: raw.clicks,
                x: raw.x,
                y: raw.y,
            })
        }
        sys::SDL_MOUSEBUTTONUP => {
            let raw = *event.button();
            window_id = Some(raw.windowID);
            MouseButtonUp(MouseButtonEvent {
                which: if raw.which == sys::SDL_TOUCH_MOUSEID {
                    MouseKind::Touch
                } else {
                    MouseKind::Mouse(raw.which)
                },
                button: MouseButton::from_raw(raw.button).expect("Invalid mouse button"),
                clicks: raw.clicks,
                x: raw.x,
                y: raw.y,
            })
        }
        sys::SDL_MOUSEMOTION => {
            let raw = *event.motion();
            window_id = Some(raw.windowID);
            MouseMotion(MouseMotionEvent {
                which: if raw.which == sys::SDL_TOUCH_MOUSEID {
                    MouseKind::Touch
                } else {
                    MouseKind::Mouse(raw.which)
                },
                held_buttons_flags: raw.state,
                x: raw.x,
                y: raw.y,
                xrel: raw.xrel,
                yrel: raw.yrel,
            })
        }
        sys::SDL_MOUSEWHEEL => {
            let raw = *event.wheel();
            window_id = Some(raw.windowID);
            MouseWheel(MouseWheelEvent {
                which: if raw.which == sys::SDL_TOUCH_MOUSEID {
                    MouseKind::Touch
                } else {
                    MouseKind::Mouse(raw.which)
                },
                x: raw.x,
                y: raw.y,
                direction: raw.direction,
            })
        }
        sys::SDL_FINGERDOWN => {
            let raw = *event.tfinger();
            FingerDown(TouchFingerEvent {
                touch_id: raw.touchId,
                finger_id: raw.fingerId,
                x: raw.x,
                y: raw.y,
                pressure: raw.pressure,
            })
        }
        sys::SDL_FINGERUP => {
            let raw = *event.tfinger();
            FingerUp(TouchFingerEvent {
                touch_id: raw.touchId,
                finger_id: raw.fingerId,
                x: raw.x,
                y: raw.y,

                pressure: raw.pressure,
            })
        }
        sys::SDL_FINGERMOTION => {
            let raw = *event.tfinger();
            FingerMotion {
                dx: raw.dx,
                dy: raw.dy,
                event: TouchFingerEvent {
                    touch_id: raw.touchId,
                    finger_id: raw.fingerId,
                    x: raw.x,
                    y: raw.y,
                    pressure: raw.pressure,
                },
            }
        }
        sys::SDL_MULTIGESTURE => {
            let raw = *event.mgesture();
            MultiGesture(MultiGestureEvent {
                touch_id: raw.touchId,
                d_theta: raw.dTheta,
                d_dist: raw.dDist,
                x: raw.x,
                y: raw.y,
                num_fingers: raw.numFingers,
            })
        }
        sys::SDL_DOLLARRECORD => {
            let raw = *event.dgesture();
            DollarRecord(DollarGestureEvent {
                touch_id: raw.touchId,
                gesture_id: raw.gestureId,
                num_fingers: raw.numFingers,
                error: raw.error,
                x: raw.x,
                y: raw.y,
            })
        }
        sys::SDL_KEYDOWN => {
            let raw = *event.key();
            window_id = Some(raw.windowID);
            let keysym = Keysym::from_raw(raw.keysym);
            if raw.repeat != 0 {
                KeyRepeat(keysym)
            } else {
                KeyDown(keysym)
            }
        }
        sys::SDL_KEYUP => {
            let raw = *event.key();
            window_id = Some(raw.windowID);
            KeyUp(Keysym::from_raw(raw.keysym))
        }
        sys::SDL_KEYMAPCHANGED => KeymapChanged,
        sys::SDL_TEXTEDITING => {
            let raw = *event.edit();
            window_id = Some(raw.windowID);
            let cstr = CStr::from_ptr(raw.text.as_ptr());
            let text = cstr.to_str().expect("SDL returned invalid UTF-8").to_owned();
            TextEditing(TextEditingEvent {
                text: text,
                start: raw.start as usize,
                length: raw.length as usize,
            })
        }
        sys::SDL_TEXTINPUT => {
            let raw = *event.text();
            window_id = Some(raw.windowID);
            let cstr = CStr::from_ptr(raw.text.as_ptr());
            let text = cstr.to_str().expect("SDL returned invalid UTF-8").to_owned();
            TextInput(text)
        }
        sys::SDL_CONTROLLERDEVICEADDED => {
            let raw = *event.cdevice();
            ControllerDeviceAdded { device_index: raw.which }
        }
        sys::SDL_CONTROLLERDEVICEREMOVED => {
            let raw = *event.cdevice();
            Controller {
                instance_id: raw.which,
                event: ControllerEvent::Removed,
            }
        }
        sys::SDL_CONTROLLERDEVICEREMAPPED => {
            let raw = *event.cdevice();
            Controller {
                instance_id: raw.which,
                event: ControllerEvent::Remapped,
            }
        }
        sys::SDL_CONTROLLERBUTTONDOWN => {
            let raw = *event.cbutton();
            Controller {
                instance_id: raw.which,
                event: ControllerEvent::ButtonDown(ControllerButton::from_raw(raw.button)
                    .expect("Invalid controller button")),
            }
        }
        sys::SDL_CONTROLLERBUTTONUP => {
            let raw = *event.cbutton();
            Controller {
                instance_id: raw.which,
                event: ControllerEvent::ButtonUp(ControllerButton::from_raw(raw.button)
                    .expect("Invalid controller button")),
            }
        }
        sys::SDL_CONTROLLERAXISMOTION => {
            let raw = *event.caxis();
            Controller {
                instance_id: raw.which,
                event: ControllerEvent::AxisMotion(ControllerAxis::from_raw(raw.axis)
                                                       .expect("Invalid controller axis"),
                                                   raw.value),
            }
        }
        sys::SDL_JOYDEVICEADDED => {
            let raw = *event.jdevice();
            JoyDeviceAdded { device_index: raw.which }
        }
        sys::SDL_JOYDEVICEREMOVED => {
            let raw = *event.jdevice();
            Joy {
                instance_id: raw.which,
                event: JoyEvent::Removed,
            }
        }
        sys::SDL_JOYBUTTONDOWN => {
            let raw = *event.jbutton();
            Joy {
                instance_id: raw.which,
                event: JoyEvent::ButtonDown(JoyButton::from_raw(raw.button)
                    .expect("Invalid joystick button")),
            }
        }
        sys::SDL_JOYBUTTONUP => {
            let raw = *event.jbutton();
            Joy {
                instance_id: raw.which,
                event: JoyEvent::ButtonUp(JoyButton::from_raw(raw.button)
                    .expect("Invalid joystick button")),
            }
        }
        sys::SDL_JOYBALLMOTION => {
            let raw = *event.jball();
            Joy {
                instance_id: raw.which,
                event: JoyEvent::BallMotion {
                    ball: raw.ball,
                    xrel: raw.xrel,
                    yrel: raw.yrel,
                },
            }
        }
        sys::SDL_JOYHATMOTION => {
            let raw = *event.jhat();
            Joy {
                instance_id: raw.which,
                event: JoyEvent::HatMotion {
                    hat: raw.hat,
                    pos: JoyHatPosition::from_raw(raw.value).expect("Invalid hat position"),
                },
            }
        }
        sys::SDL_JOYAXISMOTION => {
            let raw = *event.jaxis();
            Joy {
                instance_id: raw.which,
                event: JoyEvent::AxisMotion(JoyAxis::from_raw(raw.axis).expect("Invalid axis"),
                                            raw.value),
            }
        }
        sys::SDL_WINDOWEVENT => {
            let raw = *event.window();
            window_id = Some(raw.windowID);
            Window(WindowEvent {
                event: raw.event,
                data1: raw.data1,
                data2: raw.data2,
            })
        }
        sys::SDL_SYSWMEVENT => unimplemented!(), // Disabled by default
        sys::SDL_DROPFILE => {
            let raw = &*event.drop();
            let cstr = CStr::from_ptr(raw.file);
            let text = cstr.to_str().expect("SDL returned invalid UTF-8").to_owned();
            sys::SDL_free(raw.file as *mut c_void);
            DropFile(text)
        }
        sys::SDL_RENDER_DEVICE_RESET => RenderDeviceReset,
        sys::SDL_RENDER_TARGETS_RESET => RenderTargetsReset,
        sys::SDL_CLIPBOARDUPDATE => ClipboardUpdate,
        sys::SDL_QUIT => Quit,
        sys::SDL_USEREVENT...sys::SDL_LASTEVENT => {
            let raw = &*event.user();
            // TODO: There might be a window Id available here?
            // window_id = Some(raw.windowID);
            User(UserEvent {
                kind: raw.type_,
                code: raw.code,
                data1: raw.data1,
                data2: raw.data2,
            })
        }
        // sys::SDL_LASTEVENT => unreachable!(),
        _ => {
            unreachable!();
        }
    };
    Event {
        kind: kind,
        timestamp: timestamp,
        window_id: window_id,
    }
}

// Should this be clone?
#[derive(Debug)]
pub struct EventContext {
    _guard: InitGuard,
}

pub trait EventContextPrivate {
    fn new(guard: InitGuard) -> EventContext;
}

impl EventContextPrivate for EventContext {
    fn new(guard: InitGuard) -> EventContext {
        EventContext { _guard: guard }
    }
}

pub struct Events<'a> {
    context: &'a mut EventContext,
}
impl<'a> Iterator for Events<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.context.poll_event()
    }
}

impl EventContext {
    pub fn poll_event(&mut self) -> Option<Event> {
        let mut raw: sys::SDL_Event = unsafe { mem::uninitialized() };
        let res = unsafe { sys::SDL_PollEvent(&mut raw) };
        if res != 0 {
            Some(unsafe { wrap_event(raw) })
        } else {
            None
        }
    }
    
    pub fn wait_event(&mut self) -> Result<Event, Error> {
        let mut raw: sys::SDL_Event = unsafe { mem::uninitialized() };
        let res = unsafe { sys::SDL_WaitEvent(&mut raw) };
        if res != 0 {
            Ok(unsafe { wrap_event(raw) })
        } else {
            Err(get_error())
        }
    }

    pub fn events(&mut self) -> Events {
        Events { context: self }
    }
}
