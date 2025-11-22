use std::sync::atomic::{AtomicUsize, Ordering};

use log::{debug, info};

use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{AnyThread, DefinedClass, define_class, msg_send};
use objc2_av_foundation::*;
use objc2_core_media::CMSampleBuffer;
use objc2_foundation::{NSArray, NSDictionary, NSObjectProtocol};
use objc2_vision::{VNDetectBarcodesRequest, VNImageRequestHandler};

pub struct Ivars {
    frame_count: AtomicUsize,
    vn_detect_barcodes_request: Retained<VNDetectBarcodesRequest>,
}

define_class!(
    #[unsafe(super(NSObject))]
    #[ivars = Ivars]
    pub struct VisionDelegate;

    unsafe impl NSObjectProtocol for VisionDelegate {}

    unsafe impl AVCaptureVideoDataOutputSampleBufferDelegate for VisionDelegate {
        #[expect(non_snake_case)]
        #[expect(unsafe_op_in_unsafe_fn)]
        #[unsafe(method(captureOutput:didOutputSampleBuffer:fromConnection:))]
        unsafe fn captureOutput_didOutputSampleBuffer_fromConnection(
            &self,
            _output: &AVCaptureOutput,
            sample_buffer: &CMSampleBuffer,
            _connection: &AVCaptureConnection,
        ) {
            let vn_image_request_handler = VNImageRequestHandler::initWithCMSampleBuffer_options(
                VNImageRequestHandler::alloc(),
                sample_buffer,
                &NSDictionary::new(),
            );

            let requests = NSArray::from_slice(&[self.ivars().vn_detect_barcodes_request.as_ref()]);
            vn_image_request_handler
                .performRequests_error(&requests)
                .expect("Image requests should perform");

            if let Some(barcode_observations) = self.ivars().vn_detect_barcodes_request.results() {
                for observation in barcode_observations.iter() {
                    if let Some(payload_string) = observation.payloadStringValue() {
                        info!("Detected barcode payload");
                        println!("{}", payload_string);

                        std::process::exit(0);
                    }
                }
            }

            let prev_count = self.ivars().frame_count.fetch_add(1, Ordering::SeqCst);
            debug!("Processed frame: {}", prev_count);
        }
    }
);

impl VisionDelegate {
    pub fn new(vn_detect_barcodes_request: Retained<VNDetectBarcodesRequest>) -> Retained<Self> {
        let this = Self::alloc().set_ivars(Ivars {
            frame_count: AtomicUsize::new(0),
            vn_detect_barcodes_request,
        });
        unsafe { msg_send![super(this), init] }
    }
}
