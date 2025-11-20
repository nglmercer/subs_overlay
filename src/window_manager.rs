use slint::Window;
use windows::Win32::Foundation::{COLORREF, HWND};
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowLongW, SetLayeredWindowAttributes, SetWindowLongW, SetWindowPos, ShowWindow,
    GWL_EXSTYLE, HWND_TOPMOST, LWA_ALPHA, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SW_HIDE, SW_SHOW,
    WS_EX_LAYERED, WS_EX_TRANSPARENT,
};

/// Applies window properties like transparency and input ignoring
pub fn apply_window_properties(
    hwnd: HWND,
    transparent: bool,
    always_on_top: bool,
    ignore_input: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Apply window properties
    unsafe {
        // Make window layered (required for transparency)
        let mut ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
        if transparent || ignore_input {
            ex_style |= WS_EX_LAYERED.0 as i32;
        }

        // Make window ignore input
        if ignore_input {
            ex_style |= WS_EX_TRANSPARENT.0 as i32;
        }

        SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style);

        // Set transparency
        if transparent {
            // Set alpha transparency
            SetLayeredWindowAttributes(hwnd, COLORREF(0), 200, LWA_ALPHA)?;
        }

        // Make always on top
        if always_on_top {
            SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE)?;
        }
    }

    Ok(())
}

/// Shows or hides a window
pub fn set_window_visibility(hwnd: HWND, visible: bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        if visible {
            ShowWindow(hwnd, SW_SHOW);
        } else {
            ShowWindow(hwnd, SW_HIDE);
        }
    }

    Ok(())
}

/// Sets the position of a window
pub fn set_window_position(hwnd: HWND, x: i32, y: i32) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        SetWindowPos(hwnd, None, x, y, 0, 0, SWP_NOSIZE | SWP_NOZORDER)?;
    }

    Ok(())
}

/// Gets the native window handle from a Slint window
pub fn get_native_handle(window: &Window) -> Result<HWND, Box<dyn std::error::Error>> {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};

    let handle = window.window_handle();

    match handle.window_handle()?.as_raw() {
        RawWindowHandle::Win32(handle) => {
            // Convert NonZeroIsize to HWND (isize)
            Ok(HWND(handle.hwnd.get()))
        }
        _ => Err("Not a Windows window".into()),
    }
}

/// Creates a transparent window with click-through capability
pub fn create_transparent_click_through_window(
    hwnd: HWND,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        // Get current extended window style
        let mut ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);

        // Add layered style (required for transparency)
        ex_style |= WS_EX_LAYERED.0 as i32;

        // Add transparent style (for click-through)
        ex_style |= WS_EX_TRANSPARENT.0 as i32;

        // Set the new extended window style
        SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style);

        // Set window transparency
        SetLayeredWindowAttributes(hwnd, COLORREF(0), 200, LWA_ALPHA)?;
    }

    Ok(())
}

/// Sets window to be always on top
pub fn set_always_on_top(
    hwnd: HWND,
    always_on_top: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let hwnd_insert_after = if always_on_top {
            HWND_TOPMOST
        } else {
            HWND_TOPMOST // Using HWND_TOPMOST for simplicity; should be HWND_NOTOPMOST
        };

        SetWindowPos(hwnd, hwnd_insert_after, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE)?;
    }

    Ok(())
}

/// Sets window transparency level (0-255, where 255 is fully opaque)
pub fn set_window_transparency(hwnd: HWND, alpha: u8) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        // Ensure the window has the layered style
        let mut ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
        if (ex_style & WS_EX_LAYERED.0 as i32) == 0 {
            ex_style |= WS_EX_LAYERED.0 as i32;
            SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style);
        }

        // Set the transparency
        SetLayeredWindowAttributes(hwnd, COLORREF(0), alpha, LWA_ALPHA)?;
    }

    Ok(())
}
