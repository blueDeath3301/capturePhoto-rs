use std::{env, fs::OpenOptions};
use windows::{
    core::HSTRING,
    Media::{
        Capture::{MediaCapture, MediaCaptureInitializationSettings, PhotoCaptureSource},
        MediaProperties::ImageEncodingProperties,
    },
    Storage::StorageFile,
};

fn main() -> windows::core::Result<()> {
    //
    // Create new MediaCapture instance and initialize it.
    // https://learn.microsoft.com/uwp/api/windows.media.capture.photocapturesource
    //

    let media_capture = MediaCapture::new()?;

    let settings = MediaCaptureInitializationSettings::new()?;
    settings.SetPhotoCaptureSource(PhotoCaptureSource::VideoPreview)?;

    media_capture
        .InitializeWithSettingsAsync(&settings)?
        .get()?;

    //
    // Prepare file for output. MediaCapture requires a StorageFile
    // pointing to an existing file with a non-extended absolute path.
    // https://learn.microsoft.com/uwp/api/windows.media.mediaproperties.imageencodingproperties.createjpeg
    //

    let encoding = ImageEncodingProperties::CreateJpeg()?;

    let path = env::current_dir().unwrap().join("Output.jpg");
    let _ = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path)
        .unwrap()
        .set_len(0);

    let file = StorageFile::GetFileFromPathAsync(&HSTRING::from(path.as_os_str()))?.get()?;

    //
    // Capture photo from video source
    // https://learn.microsoft.com/uwp/api/windows.media.capture.mediacapture.capturephototostoragefileasync
    //

    media_capture
        .CapturePhotoToStorageFileAsync(&encoding, &file)?
        .get()?;

    Ok(())
}