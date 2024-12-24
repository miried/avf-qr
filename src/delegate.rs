use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{AnyThread, define_class, msg_send};
use objc2_av_foundation::*;
use objc2_foundation::{NSArray, NSObjectProtocol};

#[derive(Clone)]
pub struct Ivars {}

define_class!(
    #[unsafe(super(NSObject))]
    #[ivars = Ivars]
    pub struct Delegate;

    unsafe impl NSObjectProtocol for Delegate {}

    unsafe impl AVCaptureMetadataOutputObjectsDelegate for Delegate {
        #[expect(non_snake_case)]
        #[unsafe(method(captureOutput:didOutputMetadataObjects:fromConnection:))]
        fn captureOutput_didOutputMetadataObjects_fromConnection(
            &self,
            _output: &AVCaptureMetadataOutput,
            _metadata_objects: &NSArray<AVMetadataObject>,
            _connection: &AVCaptureConnection,
        ) {
            println!("Metadata objects captured!");
        }
    }
);

impl Delegate {
    pub fn new() -> Retained<Self> {
        let this = Self::alloc().set_ivars(Ivars {});
        unsafe { msg_send![super(this), init] }
    }
}
