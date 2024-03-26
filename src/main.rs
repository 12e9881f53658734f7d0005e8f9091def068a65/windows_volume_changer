#![windows_subsystem = "windows"]
use windows::Win32::{
    Media::Audio::{
        MMDeviceEnumerator,
        IMMDeviceEnumerator,
        Endpoints::IAudioEndpointVolume,
        eRender,
        eConsole
    },
    System::Com::{
        CLSCTX_INPROC_SERVER,
        CoCreateInstance,
        CoCreateGuid,
        CoInitializeEx,
        COINIT_MULTITHREADED
    }
};

fn main(){
    let _ = unsafe{let _ = CoInitializeEx(None, COINIT_MULTITHREADED);};
    let e: IMMDeviceEnumerator = unsafe{CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_INPROC_SERVER)}.unwrap();

    let endpoint_volume: IAudioEndpointVolume = unsafe{e.GetDefaultAudioEndpoint(eRender, eConsole).unwrap().Activate(CLSCTX_INPROC_SERVER, None)}.unwrap();
    let volume_level: f32 = 1.0;

    let guid: windows::core::GUID = unsafe{CoCreateGuid()}.unwrap();
    unsafe{endpoint_volume.SetMasterVolumeLevelScalar(volume_level, &guid)}.unwrap();
    unsafe{endpoint_volume.SetMute(false, &guid)}.unwrap();

    println!("Volume changed successfully.");
}