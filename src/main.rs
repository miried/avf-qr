mod delegate;
mod utils;

use delegate::Delegate;
use dispatch2::{DispatchQueue, DispatchQueueAttr};
use objc2::runtime::ProtocolObject;
use objc2_av_foundation::*;
use objc2_foundation::{NSArray, NSDate, NSRunLoop};

#[expect(unsafe_op_in_unsafe_fn)]
unsafe fn run_av_capture_session() {
    let run_loop = NSRunLoop::currentRunLoop();

    //
    // Initialize capture device
    //
    let av_capture_device = AVCaptureDevice::defaultDeviceWithMediaType(
        AVMediaTypeVideo.expect("AVMediaTypeVideo should be available"),
    )
    .expect("AVCaptureDevice should be available");
    println!("manufacturer: {}", av_capture_device.manufacturer());
    println!("name: {}", av_capture_device.localizedName());
    println!(
        "position: {}",
        utils::position_to_str(av_capture_device.position())
    );

    let av_capture_device_input =
        AVCaptureDeviceInput::deviceInputWithDevice_error(&av_capture_device)
            .expect("AVCaptureDeviceInput should be available");

    //
    //   Initialize capture session
    //
    let av_capture_metadata_output = AVCaptureMetadataOutput::new();
    let av_capture_session = AVCaptureSession::new();
    av_capture_session.addInput(&av_capture_device_input);
    av_capture_session.addOutput(&av_capture_metadata_output);

    //
    //   Initialize metadata output
    //
    let delegate = Delegate::new();
    let objects_delegate = ProtocolObject::from_ref(&*delegate);
    let objects_callback_queue = DispatchQueue::new("avf_qr_queue", DispatchQueueAttr::SERIAL);

    av_capture_metadata_output
        .setMetadataObjectsDelegate_queue(Some(objects_delegate), Some(&objects_callback_queue));

    let available_types = av_capture_metadata_output.availableMetadataObjectTypes();
    println!("{:?}", available_types);

    let meta_data_object_types = NSArray::from_slice(&[AVMetadataObjectTypeQRCode]);
    av_capture_metadata_output.setMetadataObjectTypes(Some(&meta_data_object_types));

    //
    //   Run capture session
    //
    av_capture_session.startRunning();
    let stop_time = NSDate::now().dateByAddingTimeInterval(5.0);
    run_loop.runUntilDate(&stop_time);
    av_capture_session.stopRunning();
}

fn main() {
    unsafe { run_av_capture_session() };
}
