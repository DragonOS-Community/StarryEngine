use starry_client::base::color::Color;

pub trait Text {
    fn set_text<S: Into<String>>(&self, text: S) -> &Self;

    fn set_text_color(&self, color: Color) -> &Self;
}
