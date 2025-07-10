use iced::{
    alignment::Horizontal,
    widget::{button, container, image as iced_image, text},
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
        match (&self.selected_file, &self.loaded_image) {
            (None, _) => button(text("Pick an image file"))
                .on_press(InstaxFramerMessage::PickImage)
                .into(),
            (Some(_), None) => container(text("Loading..."))
                .align_x(Horizontal::Center)
                .width(460)
                .height(620)
                .into(),
            (Some(selected_file), Some(_)) => container(
                iced_image(selected_file)
                    .width(460)
                    .height(620)
                    .content_fit(iced::ContentFit::Contain),
            )
            .width(460)
            .height(620)
            .into(),
        }
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

                return Task::future(async move {
                    let image_load_result = tokio::task::spawn_blocking(move || {
                        ImageReader::open(selected_file)
                            .map_err(|_| InstaxFramerMessage::ImageLoadingFailed)
                            .and_then(|image_reader| {
                                image_reader
                                    .decode()
                                    .map_err(|_| InstaxFramerMessage::ImageLoadingFailed)
                            })
                    })
                    .await;

                    match image_load_result {
                        Ok(Ok(image)) => InstaxFramerMessage::ImageLoadingFinished(image),
                        Ok(Err(e)) => {
                            eprintln!("Error loading or decoding image: {e:?}");
                            InstaxFramerMessage::ImageLoadingFailed
                        }
                        Err(e) => {
                            eprintln!(
                                "Blocking image loading task panicked or was cancelled: {e:?}"
                            );
                            InstaxFramerMessage::ImageLoadingFailed
                        }
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

    pub(crate) fn start_up_tasks(&self) -> Task<InstaxFramerMessage> {
        if let Some(selected_file) = &self.selected_file {
            Task::done(InstaxFramerMessage::ImagePicked(selected_file.to_owned()))
        } else {
            Task::none()
        }
    }
}
