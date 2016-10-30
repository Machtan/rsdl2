use sdl2_sys as sys;
use std::mem;
use libc::c_void;


pub struct EventContext {
    _dummy: bool,
}

pub trait EventContextPrivate {
    fn new() -> EventContext;
}

impl EventContextPrivate for EventContext {
    fn new() -> EventContext {
        EventContext { _dummy: true }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AudioDeviceKind {
    Output,
    Capture(u8) // zero if an output device, non-zero if a capture device.
}

#[derive(Debug, Clone)]
pub enum EventKind {
    AppWillEnterBackground,
    AppDidEnterBackground,
    AppWillEnterForeground,
    AppDidEnterForeground,
    AppTerminating,
    AppLowMemory,
    AudioDeviceAdded { device_index: u32, device: AudioDeviceKind },
    AudioDeviceRemoved { device_id: u32, device: AudioDeviceKind },
    MouseButtonDown(MouseButtonEvent),
    MouseButtonUp(MouseButtonEvent),
    MouseMotion(MouseMotionEvent),
    MouseWheel(MouseWheelEvent),
    FingerDown(TouchFingerEvent),
    FingerUp(TouchFingerEvent),
    FingerMotion(TouchFingerEvent),
    DollarRecord(DollarGestureEvent),
    DollarGesture(DollarGestureEvent),
    KeyDown(KeyboardEvent),
    KeyUp(KeyboardEvent),
    KeymapChanged,
    TextEditing(TextEditingEvent),
    TextInput(String),
    /* The joystick device index for the ADDED event, instance id for the REMOVED or REMAPPED event */
    ControllerDeviceAdded { device_index: i32},
    ControllerDeviceRemoved { instance_id: i32},
    ControllerDeviceRemapped { instance_id: i32},
    ControllerButtonDown(ControllerButtonEvent),
    ControllerButtonUp(ControllerButtonEvent),
    ControllerAxisMotion(ControllerAxisEvent),
    JoyDeviceAdded { device_index: i32 },
    JoyDeviceRemoved { instance_id: i32 },
    JoyButtonDown(JoyButtonEvent),
    JoyButtonUp(JoyButtonEvent),
    JoyBallMotion(JoyBallEvent),
    JoyHatMotion(JoyHatEvent),
    JoyAxisMotion(JoyAxisEvent),
    Window(WindowEvent),
    // SysWmEvent(SysWmEventData), // Disabled by default
    DropFile(String),
    RenderDeviceReset,
    RenderTargetsReset,
    User(UserEvent),
    ClipboardUpdate,
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioDeviceEvent {
    which: u32, /* The audio device index for the ADDED event (valid until next SDL_GetNumAudioDevices() call), SDL_AudioDeviceID for the REMOVED event */
    iscapture: Option<u8>, // zero if an output device, non-zero if a capture device.
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseButtonEvent {
    pub window_id: u32, // The window with mouse focus, if any
    pub which: u32, // The mouse instance id, or SDL_TOUCH_MOUSEID
    pub button: u8, // The mouse button index
    pub pressed: bool, // ::SDL_PRESSED or ::SDL_RELEASED
    pub clicks: u8, // 1 for single-click, 2 for double-click, etc.
    pub x: i32, // X coordinate, relative to window
    pub y: i32, // Y coordinate, relative to window
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseMotionEvent {
    pub window_id: u32, // The window with mouse focus, if any
    pub which: u32, // The mouse instance id, or SDL_TOUCH_MOUSEID
    pub state: u32, // The current button state
    pub x: i32, // X coordinate, relative to window
    pub y: i32, // Y coordinate, relative to window
    pub xrel: i32, // The relative motion in the X direction
    pub yrel: i32, // The relative motion in the Y direction
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseWheelEvent {
    pub window_id: u32, // The window with mouse focus, if any
    pub which: u32, // The mouse instance id, or SDL_TOUCH_MOUSEID
    pub x: i32, // The amount scrolled horizontally, positive to the right and negative to the left
    pub y: i32, /* The amount scrolled vertically, positive away from the user and negative toward the user */
    pub direction: u32, /* Set to one of the SDL_MOUSEWHEEL_* defines. When FLIPPED the values in X and Y will be opposite. Multiply by -1 to change them back */
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TouchFingerEvent {
    touchId: sys::SDL_TouchID, // The touch device id
    fingerId: sys::SDL_FingerID,
    x: f32, // Normalized in the range 0...1
    y: f32, // Normalized in the range 0...1
    dx: f32, // Normalized in the range -1...1
    dy: f32, // Normalized in the range -1...1
    pressure: f32, // Normalized in the range 0...1
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DollarGestureEvent {
    touchId: sys::SDL_TouchID, // The touch device id
    gestureId: sys::SDL_GestureID,
    numFingers: u32,
    error: f32,
    x: f32, // Normalized center of gesture
    y: f32, // Normalized center of gesture
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowEvent {
    pub window_id: u32, // The associated window
    pub event: u8, // ::SDL_WindowEventID
    pub data1: i32, // event dependent data
    pub data2: i32, // event dependent data
}

// typedef struct SDL_Keysym
// {
// SDL_Scancode scancode;      /* SDL physical key code - see ::SDL_Scancode for details */
// SDL_Keycode sym;            /* SDL virtual key code - see ::SDL_Keycode for details */
// Uint16 mod;                 /* current key modifiers */
// Uint32 unused;
// } SDL_Keysym;
//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Scancode {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keycode {}

#[derive(Debug, Clone, Copy)]
pub struct Keysym {
    pub scancode: Scancode,
    pub keycode: Keycode,
    pub modifiers: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct KeyboardEvent {
    pub window_id: u32, // The window with keyboard focus, if any
    pub pressed: bool, // ::SDL_PRESSED or ::SDL_RELEASED
    pub repeat: bool, // Non-zero if this is a key repeat
    pub keysym: Keysym, // The key that was pressed or released
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextEditingEvent {
    pub window_id: u32, // The window with keyboard focus, if any
    pub text: String, // The editing text
    pub start: u8, // The start cursor of selected editing text
    pub length: u8, // The length of selected editing text
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ControllerButtonEvent {
    pub which: i32, // The joystick instance id
    pub button: u8, // The controller button (SDL_GameControllerButton)
    pub pressed: bool, // ::SDL_PRESSED or ::SDL_RELEASED
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ControllerAxisEvent {
    pub which: i32, // The joystick instance id
    pub axis: u8, // The controller axis (SDL_GameControllerAxis)
    pub value: i16, // The axis value (range: -32768 to 32767)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JoyAxisEvent {
    pub which: i32, // The joystick instance id
    pub axis: u8, // The joystick axis index
    pub value: i16, // The axis value (range: -32768 to 32767)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JoyBallEvent {
    pub which: i32, // The joystick instance id
    pub ball: u8, // The joystick trackball index
    pub xrel: i16, // The relative motion in the X direction
    pub yrel: i16, // The relative motion in the Y direction
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JoyHatEvent {
    pub which: i32, // The joystick instance id
    pub hat: u8, // The joystick hat index
    pub value: u8, /* The hat position value.
                *   \sa ::SDL_HAT_LEFTUP ::SDL_HAT_UP ::SDL_HAT_RIGHTUP
                *   \sa ::SDL_HAT_LEFT ::SDL_HAT_CENTERED ::SDL_HAT_RIGHT
                *   \sa ::SDL_HAT_LEFTDOWN ::SDL_HAT_DOWN ::SDL_HAT_RIGHTDOWN
                *
                *   Note that zero means the POV is centered.
                * */
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JoyButtonEvent {
    pub which: i32, // The joystick instance id
    pub button: u8, // The joystick button index
    pub pressed: bool, // ::SDL_PRESSED or ::SDL_RELEASED
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UserEvent {
    pub kind: u32, // ::SDL_USEREVENT through ::SDL_LASTEVENT-1
    pub windowID: u32, // The associated window if any
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

fn wrap_event(event: sys::SDL_Event) -> Event {
    use self::EventKind::*;
    let mut window_id = None;
    let (type_, timestamp) = {
        let common = *event.common();
        (common.type_, common.timestamp)
    };
    let kind = match type_ {
        sys::SDL_FIRSTEVENT => unreachable!(),
        sys::SDL_APP_WILLENTERBACKGROUND => AppWillEnterBackground,
        sys::SDL_APP_DIDENTERBACKGROUND => AppDidEnterBackground,
        sys::SDL_APP_WILLENTERFOREGROUND => AppWillEnterForeground,
        sys::SDL_APP_DIDENTERFOREGROUND => AppDidEnterForeground,
        sys::SDL_APP_TERMINATING => AppTerminating,
        sys::SDL_APP_LOWMEMORY => AppLowMemory,
        sys::SDL_AUDIODEVICEADDED => {
            let raw = *event.adevice();
            AudioDeviceAdded{
                device_index: raw.which,
                device: if raw.iscapture == 0 {
                    AudioDeviceKind::Output
                } else {
                    AudioDeviceKind::Capture(raw.iscapture)
                }
            }
        },
        sys::SDL_AUDIODEVICEREMOVED => {
            let raw = *event.adevice();
            AudioDeviceRemoved{
                device_id: raw.which,
                device: if raw.iscapture == 0 {
                    AudioDeviceKind::Output
                } else {
                    AudioDeviceKind::Capture(raw.iscapture)
                }
            }
        },
        sys::SDL_MOUSEBUTTONDOWN => MouseButtonDown {},
        sys::SDL_MOUSEBUTTONUP => MouseButtonUp {},
        sys::SDL_MOUSEMOTION => MouseMotion {},
        sys::SDL_MOUSEWHEEL => MouseWheel {},
        sys::SDL_FINGERDOWN => FingerDown {},
        sys::SDL_FINGERUP => FingerUp {},
        sys::SDL_FINGERMOTION => FingerMotion {},
        sys::SDL_MULTIGESTURE => MultiGesture {},
        sys::SDL_DOLLARRECORD => DollarRecord {},
        sys::SDL_KEYDOWN => KeyDown {},
        sys::SDL_KEYUP => KeyUp {},
        sys::SDL_KEYMAPCHANGED => KeyMapChanged {},
        sys::SDL_TEXTEDITING => TextEditing {},
        sys::SDL_TEXTINPUT => TextInput {},
        sys::SDL_CONTROLLERDEVICEADDED => ControllerDeviceAdded {},
        sys::SDL_CONTROLLERDEVICEREMOVED => ControllerDeviceRemoved {},
        sys::SDL_CONTROLLERDEVICEREMAPPED => ControllerDeviceRemapped {},
        sys::SDL_CONTROLLERBUTTONDOWN => ControllerButtonDown {},
        sys::SDL_CONTROLLERBUTTONUP => ControllerButtonUp {},
        sys::SDL_CONTROLLERAXISMOTION => ControllerAxisMotion {},
        sys::SDL_JOYDEVICEADDED => MouseButtonDown {},
        sys::SDL_JOYDEVICEREMOVED => MouseButtonDown {},
        sys::SDL_JOYBUTTONDOWN => MouseButtonDown {},
        sys::SDL_JOYBUTTONUP => MouseButtonDown {},
        sys::SDL_JOYBALLMOTION => MouseButtonDown {},
        sys::SDL_JOYHATMOTION => MouseButtonDown {},
        sys::SDL_JOYAXISMOTION => MouseButtonDown {},
        sys::SDL_WINDOWEVENT => MouseButtonDown {},
        sys::SDL_SYSWMEVENT => MouseButtonDown {},
        sys::SDL_DROPFILE => MouseButtonDown {},
        sys::SDL_RENDER_DEVICE_RESET => MouseButtonDown {},
        sys::SDL_RENDER_TARGETS_RESET => MouseButtonDown {},
        sys::SDL_CLIPBOARDUPDATE => MouseButtonDown {},
        sys::SDL_QUIT => MouseButtonDown {},
        sys::SDL_USEREVENT ... sys::SDL_LASTEVENT => MouseButtonDown {},
        sys::SDL_LASTEVENT => unreachable!(),
        _ => {
            unreachable!();
        }
    };
    Event {
        kind: kind,
        timestamp: timestamp,
        window_id, window_id,
    }
    unimplemented!();
}

impl EventContext {
    pub fn poll_event(&mut self) -> Option<Event> {
        let mut raw: sys::SDL_Event = unsafe { mem::uninitialized() };
        let res = unsafe { sys::SDL_PollEvent(&mut raw) };
        if res != 0 {
            Some(wrap_event(raw))
        } else {
            None
        }
    }

    pub fn get_events() {}
}
