fn main() {
    windows::build!(
        Windows::Win32::Graphics::Direct3D11::*,
        Windows::Win32::Graphics::Direct3D12::*,
        Windows::Win32::Graphics::Dxgi::*,
        Windows::Win32::Graphics::Direct2D::*,
        Windows::Win32::Graphics::DirectWrite::*,
        Windows::Win32::Graphics::Imaging::*,
        Windows::Win32::System::Threading::{
            CreateEventW,
            WaitForSingleObject,
        },
        Windows::Win32::Foundation::{
            FILETIME,
        },
    );
}
