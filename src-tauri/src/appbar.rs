use std::mem;
use tauri::{Error, WebviewWindow};
use windows::Win32::Foundation::{HWND, LPARAM};
use windows::Win32::UI::Shell::{
    SHAppBarMessage, ABE_TOP, ABM_NEW, ABM_REMOVE, ABM_SETPOS, APPBARDATA,
};
use windows::Win32::UI::WindowsAndMessaging::WM_USER;

pub fn make_window_appbar(webview: WebviewWindow, width: i32, height: i32) -> Result<(), Error> {
    let hwnd = get_appbar_hwnd(webview)?;
    let rect = windows::Win32::Foundation::RECT {
        left: 0,
        top: 0,
        right: width,
        bottom: height,
    };
    let alignment = ABE_TOP;
    let mut appbar_data = create_appbar_data(hwnd, alignment)?;

    new_appbar(appbar_data)?;
    appbar_data.rc = rect;
    appbar_data.lParam = LPARAM(Default::default());
    set_appbar_pos(appbar_data)?;
    Ok(())
}

pub fn destroy_appbar(webview: WebviewWindow) -> Result<(), Error> {
    let hwnd = get_appbar_hwnd(webview)?;

    let mut appbar_data = create_appbar_data(hwnd, 0)?;
    // Unregister the window as an appbar
    unsafe {
        SHAppBarMessage(ABM_REMOVE, &mut appbar_data);
    }

    Ok(())
}

fn create_appbar_data(hwnd: HWND, alignment: u32) -> Result<APPBARDATA, Error> {
    let mut abd = APPBARDATA::default();
    abd.hWnd = hwnd;
    abd.uCallbackMessage = WM_USER + 0x02;
    abd.uEdge = alignment;
    abd.cbSize = mem::size_of::<APPBARDATA>() as u32;
    return Ok(abd);
}

fn new_appbar(mut pdata: APPBARDATA) -> Result<(), Error> {
    unsafe {
        SHAppBarMessage(ABM_NEW, &mut pdata);
    }
    Ok(())
}

fn set_appbar_pos(mut pdata: APPBARDATA) -> Result<(), Error> {
    unsafe {
        SHAppBarMessage(ABM_SETPOS, &mut pdata);
    }
    Ok(())
}
fn get_appbar_hwnd(webview: WebviewWindow) -> Result<HWND, Error> {
    // Attempt to retrieve the main webview window
    /* let webview = app
    .get_webview_window("main")
    .ok_or(Error::WebviewNotFound)?; */

    // Retrieve the HWND from the webview
    let hwnd = webview.hwnd()?;
    // convert the HWND to a isize
    let hwnd = hwnd.0 as isize;
    Ok(windows::Win32::Foundation::HWND(hwnd))
}
