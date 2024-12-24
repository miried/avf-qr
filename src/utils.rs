use objc2_av_foundation::AVCaptureDevicePosition;

pub fn position_to_str(position: AVCaptureDevicePosition) -> &'static str {
    match position {
        AVCaptureDevicePosition::Front => "Front",
        AVCaptureDevicePosition::Back => "Back",
        AVCaptureDevicePosition::Unspecified => "Unspecified",
        _ => "Invalid",
    }
}
