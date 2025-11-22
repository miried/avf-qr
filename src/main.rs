#[cfg(feature = "core-image")]
mod image_delegate;
mod utils;
#[cfg(feature = "vision")]
mod vision_delegate;

use log::info;

use dispatch2::{DispatchQueue, DispatchQueueAttr};
use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_av_foundation::*;
use objc2_foundation::NSRunLoop;

#[expect(unsafe_op_in_unsafe_fn)]
unsafe fn new_device_input() -> Retained<AVCaptureDeviceInput> {
    let av_capture_device = AVCaptureDevice::defaultDeviceWithMediaType(
        AVMediaTypeVideo.expect("AVMediaTypeVideo should be available"),
    )
    .expect("AVCaptureDevice should be available");

    let av_capture_device_input =
        AVCaptureDeviceInput::deviceInputWithDevice_error(&av_capture_device)
            .expect("AVCaptureDeviceInput should be available");

    info!("manufacturer: {}", av_capture_device.manufacturer());
    info!("name: {}", av_capture_device.localizedName());
    info!(
        "position: {}",
        utils::position_to_str(av_capture_device.position())
    );

    av_capture_device_input
}

#[cfg(feature = "vision")]
#[expect(unsafe_op_in_unsafe_fn)]
unsafe fn new_vision_delegate()
-> Retained<ProtocolObject<dyn AVCaptureVideoDataOutputSampleBufferDelegate>> {
    use objc2_vision::VNDetectBarcodesRequest;

    use crate::vision_delegate::VisionDelegate;

    info!("Using Vision framework for barcode detection");

    let vn_detect_barcodes_request = VNDetectBarcodesRequest::new();

    let delegate = VisionDelegate::new(vn_detect_barcodes_request);

    ProtocolObject::from_retained(delegate.clone())
}

#[cfg(feature = "core-image")]
#[expect(unsafe_op_in_unsafe_fn)]
unsafe fn new_core_image_delegate()
-> Retained<ProtocolObject<dyn AVCaptureVideoDataOutputSampleBufferDelegate>> {
    use crate::image_delegate::ImageDelegate;
    use objc2_core_image::{CIContext, CIDetector, CIDetectorTypeQRCode};

    info!("Using Core Image for barcode detection");

    let context = CIContext::new();
    let qr_detector =
        CIDetector::detectorOfType_context_options(CIDetectorTypeQRCode, Some(&context), None)
            .expect("Should create QR code detector");

    let delegate = ImageDelegate::new(qr_detector);

    ProtocolObject::from_retained(delegate)
}

#[expect(unsafe_op_in_unsafe_fn)]
unsafe fn new_sample_buffer_delegate()
-> Retained<ProtocolObject<dyn AVCaptureVideoDataOutputSampleBufferDelegate>> {
    #[cfg(all(feature = "vision", not(feature = "core-image")))]
    {
        new_vision_delegate()
    }
    #[cfg(all(not(feature = "vision"), feature = "core-image"))]
    {
        new_core_image_delegate()
    }
    #[cfg(all(feature = "vision", feature = "core-image"))]
    {
        compile_error!(
            "Only one of the features 'vision' or 'core-image' can be enabled at a time"
        );
    }
    #[cfg(not(any(feature = "vision", feature = "core-image")))]
    {
        compile_error!("Either feature 'vision' or 'core-image' must be enabled");
    }
}

#[expect(unsafe_op_in_unsafe_fn)]
unsafe fn new_data_output(
    sample_buffer_delegate: &ProtocolObject<dyn AVCaptureVideoDataOutputSampleBufferDelegate>,
) -> Retained<AVCaptureVideoDataOutput> {
    let av_capture_video_data_output = AVCaptureVideoDataOutput::new();

    let sample_buffer_callback_queue =
        DispatchQueue::new("avf_qr_queue", DispatchQueueAttr::SERIAL);
    av_capture_video_data_output.setSampleBufferDelegate_queue(
        Some(sample_buffer_delegate),
        Some(&sample_buffer_callback_queue),
    );

    av_capture_video_data_output
}

#[expect(unsafe_op_in_unsafe_fn)]
unsafe fn new_capture_session(
    device_input: Retained<AVCaptureDeviceInput>,
    video_output: Retained<AVCaptureVideoDataOutput>,
) -> Retained<AVCaptureSession> {
    let av_capture_session = AVCaptureSession::new();
    av_capture_session.addInput(&device_input);
    av_capture_session.addOutput(&video_output);

    av_capture_session
}

#[expect(unsafe_op_in_unsafe_fn)]
unsafe fn run_av_capture_session(
    sample_buffer_delegate: &ProtocolObject<dyn AVCaptureVideoDataOutputSampleBufferDelegate>,
) -> Retained<AVCaptureSession> {
    let av_capture_device_input = new_device_input();
    let av_capture_video_data_output = new_data_output(sample_buffer_delegate);

    let av_capture_session =
        new_capture_session(av_capture_device_input, av_capture_video_data_output);

    av_capture_session.startRunning();

    av_capture_session
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let av_capture_video_data_output_delegate = unsafe { new_sample_buffer_delegate() };
    let av_capture_session =
        unsafe { run_av_capture_session(&av_capture_video_data_output_delegate) };

    info!("Started AVCaptureSession, waiting for barcode...");

    let run_loop = NSRunLoop::currentRunLoop();
    run_loop.run();

    unsafe { av_capture_session.stopRunning() };
}
