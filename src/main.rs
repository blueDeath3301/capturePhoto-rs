
/*
We are going to capture photos from the camera and save them to a file as explained on the README.md file
using Windows-rs crate to interact with the Windows API.
 */


pub mod main2;

use windows::{
    Foundation::{IAsyncOperation, Size},
    Media::Capture::{CameraCaptureUI, CameraCaptureUIPhotoFormat, CameraCaptureUIMode},
    Storage::StorageFile,
};

use windows::core::HRESULT;
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize};
use windows::Win32::System::Com::COINIT_APARTMENTTHREADED;



fn capture_photo() -> Result<(), std::io::Error> {
    let capture_ui = CameraCaptureUI::new()?;
    capture_ui.PhotoSettings()?.SetFormat(CameraCaptureUIPhotoFormat(0))?; //JPEG
    capture_ui.PhotoSettings()?.SetCroppedSizeInPixels(Size{Width: 200.0, Height: 200.0})?;

    let photo: IAsyncOperation<StorageFile> = capture_ui.CaptureFileAsync(CameraCaptureUIMode(1))?;

    // Wait for the photo to be captured
    //let _photo = photo.Completed();

    //we must handle the any error that may occur for the AsynchOperationCompletionHandler
    match photo.Status() {
        Ok(_) => {
            let photo = photo.GetResults()?;
            println!("Photo captured: {:?}", photo.Path()?);
        }
        Err(e) => {
            println!("Error capturing photo: {:?}", e);
        }
    }

    Ok(())
}


fn main() {

    //initialize COM in STA mode
    unsafe {
       let init_com = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

         if init_com != HRESULT(0) {
              println!("Error initializing COM for this thread: {:?}", init_com);
              return 
         }
    
    }   
    let capture = capture_photo();
    match capture {
        Ok(_) => println!("Photo captured successfully"),
        Err(e) => println!("Error capturing photo: {:?}", e),
    }
    //uninitialize COM
    unsafe {
        CoUninitialize();
    }

}
 
 



