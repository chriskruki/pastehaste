#[cfg(windows)]
use windows::Win32::UI::Input::KeyboardAndMouse::*;

/// Release all modifier keys (Ctrl, Alt, Shift, Win) so they don't interfere with SendInput.
#[cfg(windows)]
pub fn release_modifiers() {
    let modifiers = [
        VIRTUAL_KEY(0x10), // VK_SHIFT
        VIRTUAL_KEY(0x11), // VK_CONTROL
        VIRTUAL_KEY(0x12), // VK_MENU (Alt)
        VIRTUAL_KEY(0x5B), // VK_LWIN
        VIRTUAL_KEY(0x5C), // VK_RWIN
        VIRTUAL_KEY(0xA0), // VK_LSHIFT
        VIRTUAL_KEY(0xA1), // VK_RSHIFT
        VIRTUAL_KEY(0xA2), // VK_LCONTROL
        VIRTUAL_KEY(0xA3), // VK_RCONTROL
        VIRTUAL_KEY(0xA4), // VK_LMENU (Left Alt)
        VIRTUAL_KEY(0xA5), // VK_RMENU (Right Alt)
    ];

    let inputs: Vec<INPUT> = modifiers
        .iter()
        .map(|&vk| INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        })
        .collect();

    unsafe {
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}

/// Simulate typing a string character-by-character using SendInput (Unicode).
#[cfg(windows)]
pub fn send_string(text: &str) {
    let inputs: Vec<INPUT> = text
        .encode_utf16()
        .flat_map(|code| {
            [
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: VIRTUAL_KEY(0),
                            wScan: code,
                            dwFlags: KEYEVENTF_UNICODE,
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                },
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: VIRTUAL_KEY(0),
                            wScan: code,
                            dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                },
            ]
        })
        .collect();

    if !inputs.is_empty() {
        unsafe {
            SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
        }
    }
}

/// Simulate Ctrl+C keypress to copy selected text.
#[cfg(windows)]
pub fn send_ctrl_c() {
    let vk_control = VIRTUAL_KEY(0x11); // VK_CONTROL
    let vk_c = VIRTUAL_KEY(0x43); // 'C'

    let inputs = [
        // Ctrl down
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk_control,
                    wScan: 0,
                    dwFlags: KEYBD_EVENT_FLAGS(0),
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        // C down
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk_c,
                    wScan: 0,
                    dwFlags: KEYBD_EVENT_FLAGS(0),
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        // C up
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk_c,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        // Ctrl up
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk_control,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];

    unsafe {
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}

#[cfg(not(windows))]
pub fn release_modifiers() {
    eprintln!("SendInput is only supported on Windows");
}

#[cfg(not(windows))]
pub fn send_string(_text: &str) {
    eprintln!("SendInput is only supported on Windows");
}

#[cfg(not(windows))]
pub fn send_ctrl_c() {
    eprintln!("SendInput is only supported on Windows");
}
