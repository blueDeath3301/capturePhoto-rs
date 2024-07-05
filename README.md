# capturePhoto-rs

This module will show a demo of how to take a photo using the built in camera. The goal is to write a module that will take pictures
and send them to an attcker's IP.

WARNING!! DOES NOT WORK (yet) STILL UNDER DEVELOPMENT. Throws up the error: Os {code: -2147483634, kind: Uncategorized, message: "A method was called at an unexpected time." }


The documentation from Microsoft (C# based) states the following;
		
		To capture a photo, create a new CameraCaptureUI object. By using the object's PhotoSettings property, you can specify properties for the returned photo, such as the image format of the photo. By default, the camera capture UI supports cropping the photo before it's returned. This can be disabled with the AllowCropping property. This example sets the CroppedSizeInPixels to request that the returned image be 200 x 200 in pixels.

		Call CaptureFileAsync and specify CameraCaptureUIMode.Photo to specify that a photo should be captured. The method returns a StorageFile instance containing the image if the capture is successful. If the user cancels the capture, the returned object is null.

		<code>

				using Windows.Media.Capture;
				using Windows.Storage;

				CameraCaptureUI captureUI = new CameraCaptureUI();
				captureUI.PhotoSettings.Format = CameraCaptureUIPhotoFormat.Jpeg;
				captureUI.PhotoSettings.CroppedSizeInPixels = new Size(200, 200); 

				StorageFile photo = await captureUI.CaptureFileAsync(CameraCaptureUIMode.Photo);

				if (photo == null)
				{
				    // User cancelled photo capture
				    return;
				}

		The StorageFile containing the captured photo is given a dynamically generated name and saved in your app's local folder. To better organize your captured photos, you can move the file to a different folder.

		<code>
				StorageFolder destinationFolder = await ApplicationData.Current.LocalFolder.CreateFolderAsync("ProfilePhotoFolder", 
        CreationCollisionOption.OpenIfExists);

				await photo.CopyAsync(destinationFolder, "ProfilePhoto.jpg", NameCollisionOption.ReplaceExisting);
				await photo.DeleteAsync();



	********************************************************************************************************************
	Now let us look at the Rust Docs

	windows::Media::Capture::CameraCaptureUI

	pub struct CameraCaptureUI(/* private fields*/)

	impl CameraCaptureUI {

		pub fn new() -> Result<Self>

		pub fn PhotoSettings(&self) -> Result<CameraCaptureUIPhotoCaptureSettings>

		pub fn VideoSettings(&self) -> Result<CameraCaptureUIVideoCaptureSettings>

		pub fn CaptureFileAsync(&self, mode: CameraCaptureUIMode) -> Result<IAsyncOperation<StorageFile>>
	}

And for the Photo capture settings

		windows::Media::Capture::CameraCaptureUIPhotoCaptureSettings

		pub struct CameraCaptureUIPhotoCaptureSettings(/* private fields */);

		
		impl CameraCaptureUIPhotoCaptureSettings {

			pub fn Format(&self) -> Result<CameraCaptureUIPhotoFormat>
			pub fn SetFormat(&self, value: CameraCaptureUIPhotoFormat) -> Result<()>
			pub fn MaxResolution(&self) -> Result<CameraCaptureUIMaxPhotoResolution>

			pub fn SetMaxResolution(
			    &self,
			    value: CameraCaptureUIMaxPhotoResolution
			) -> Result<()>

			pub fn CroppedSizeInPixels(&self) -> Result<Size>
			pub fn SetCroppedSizeInPixels(&self, value: Size) -> Result<()>
			pub fn CroppedAspectRatio(&self) -> Result<Size>
			pub fn SetCroppedAspectRatio(&self, value: Size) -> Result<()>
			pub fn AllowCropping(&self) -> Result<bool>
			pub fn SetAllowCropping(&self, value: bool) -> Result<()>

		}

	Photo formats
				 windows::Media::Capture::CameraCaptureUIPhotoFormat

				#[repr(transparent)]
				pub struct CameraCaptureUIPhotoFormat(pub i32);

				Tuple Fields
				0: i32
				Implementations
				impl CameraCaptureUIPhotoFormat
				pub const Jpeg: Self = _
				pub const Png: Self = _
				pub const JpegXR: Self = _

CaptureUIModes
				windows::Media::Capture::CameraCaptureUIMode

				#[repr(transparent)]
				pub struct CameraCaptureUIMode(pub i32);

				Tuple Fields
				0: i32
				Implementations
				impl CameraCaptureUIMode
				pub const PhotoOrVideo: Self = _
				pub const Photo: Self = _
				pub const Video: Self = _