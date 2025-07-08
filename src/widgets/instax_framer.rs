use iced::{
    widget::{button, image as iced_image, text},
    Element, Task,
};
use image::{DynamicImage, ImageReader};
use native_dialog::{DialogBuilder, MessageLevel};
use rfd::FileDialog;
use std::{borrow::Cow, path::PathBuf};

#[derive(Debug, Default)]
pub struct InstaxFramer {
    selected_file: Option<PathBuf>,
    loaded_image: Option<DynamicImage>,
}

#[derive(Debug, Clone)]
pub enum InstaxFramerMessage {
    PickImage,
    ImagePicked(PathBuf),
    ImageLoadingFailed,
    ImageLoadingFinished(DynamicImage),
}

impl InstaxFramer {
    pub(crate) fn init() -> InstaxFramer {
        Self {
            ..Default::default()
        }
    }

    pub(crate) fn view(&self) -> Element<'_, InstaxFramerMessage> {
        let (Some(selected_file), Some(_)) = (&self.selected_file, &self.loaded_image) else {
            return button(text("Pick an image file"))
                .on_press(InstaxFramerMessage::PickImage)
                .into();
        };

        iced_image(selected_file)
            .width(460)
            .height(620)
            .content_fit(iced::ContentFit::Contain)
            .into()
    }

    pub(crate) fn update(&mut self, message: InstaxFramerMessage) -> Task<InstaxFramerMessage> {
        match message {
            InstaxFramerMessage::PickImage => {
                let file = FileDialog::new()
                    .add_filter("image", &["jpg", "jpeg", "png"])
                    .pick_file();

                if let Some(file) = file {
                    return Task::done(InstaxFramerMessage::ImagePicked(file));
                }
            }
            InstaxFramerMessage::ImagePicked(selected_file) => {
                self.selected_file = Some(selected_file.clone());

                return Task::future(async {
                    match ImageReader::open(selected_file) {
                        Ok(image_reader) => match image_reader.decode() {
                            Ok(image) => InstaxFramerMessage::ImageLoadingFinished(image),
                            Err(_) => InstaxFramerMessage::ImageLoadingFailed,
                        },
                        Err(_) => InstaxFramerMessage::ImageLoadingFailed,
                    }
                });
            }
            InstaxFramerMessage::ImageLoadingFailed => {
                let _ = DialogBuilder::message()
                    .set_title("Image loading failed...!")
                    .set_text(format!(
                        "Failed to load image at {}",
                        self.selected_file
                            .as_ref()
                            .map(|path| path.to_string_lossy())
                            .unwrap_or(Cow::Borrowed("Unknown")),
                    ))
                    .set_level(MessageLevel::Error)
                    .alert()
                    .show();
            }
            InstaxFramerMessage::ImageLoadingFinished(dynamic_image) => {
                self.loaded_image = Some(dynamic_image);
            }
        }

        Task::none()
    }
}
