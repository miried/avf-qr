use std::sync::atomic::{AtomicUsize, Ordering};

use log::{debug, info};

use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{AnyThread, DefinedClass, define_class, msg_send};
use objc2_av_foundation::*;
use objc2_core_image::{CIDetector, CIImage, CIQRCodeFeature};
use objc2_core_media::CMSampleBuffer;
use objc2_foundation::NSObjectProtocol;

pub struct Ivars {
    frame_count: AtomicUsize,
    qr_detector: Retained<CIDetector>,
}

define_class!(
    #[unsafe(super(NSObject))]
    #[ivars = Ivars]
    pub struct ImageDelegate;

    unsafe impl NSObjectProtocol for ImageDelegate {}

    unsafe impl AVCaptureVideoDataOutputSampleBufferDelegate for ImageDelegate {
        #[expect(non_snake_case)]
        #[expect(unsafe_op_in_unsafe_fn)]
        #[unsafe(method(captureOutput:didOutputSampleBuffer:fromConnection:))]
        unsafe fn captureOutput_didOutputSampleBuffer_fromConnection(
            &self,
            _output: &AVCaptureOutput,
            sample_buffer: &CMSampleBuffer,
            _connection: &AVCaptureConnection,
        ) {
            let image_buffer = CMSampleBuffer::image_buffer(sample_buffer)
                .expect("Should get image buffer from sample buffer");

            let ci_image = CIImage::imageWithCVImageBuffer(&image_buffer);

            self.ivars()
                .qr_detector
                .featuresInImage(&ci_image)
                .iter()
                .for_each(|feature| {
                    let qr_code_feature = feature
                        .downcast::<CIQRCodeFeature>()
                        .expect("Should be a QR code feature");
                    let message_string = qr_code_feature
                        .messageString()
                        .expect("Should have message string");
                    info!("Detected QR code");

                    println!("{}", message_string);
                    std::process::exit(0);
                });

            let prev_count = self.ivars().frame_count.fetch_add(1, Ordering::SeqCst);
            debug!("Processed frame: {}", prev_count);
        }
    }
);

impl ImageDelegate {
    pub fn new(qr_detector: Retained<CIDetector>) -> Retained<Self> {
        let this = Self::alloc().set_ivars(Ivars {
            frame_count: AtomicUsize::new(0),
            qr_detector,
        });
        unsafe { msg_send![super(this), init] }
    }
}
