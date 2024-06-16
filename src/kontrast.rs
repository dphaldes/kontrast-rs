#[cxx_qt::bridge]
mod ffi {

    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qml_singleton]
        #[qproperty(QColor, text_color)]
        #[qproperty(i32, text_hue)]
        #[qproperty(i32, text_saturation)]
        #[qproperty(i32, text_lightness)]
        #[qproperty(i32, font_size)]
        #[qproperty(QColor, background_color)]
        #[qproperty(i32, background_hue)]
        #[qproperty(i32, background_saturation)]
        #[qproperty(i32, background_lightness)]
        #[qproperty(QString, font_size_label)]
        #[qproperty(f32, contrast)]
        #[qproperty(QColor, display_text_color)]
        #[qproperty(QColor, grabbed_color)]
        type Kontrast = super::KontrastImpl;

        #[qinvokable]
        fn random(self: Pin<&mut Kontrast>);

        #[qinvokable]
        fn reverse(self: Pin<&mut Kontrast>);
    }

    impl cxx_qt::Constructor<()> for Kontrast {}
}

use std::pin::Pin;

use cxx_qt_lib::{QColor, QString};
use rand::{thread_rng, Rng};

#[derive(Default)]
pub struct KontrastImpl {
    text_color: QColor,
    text_hue: i32,
    text_saturation: i32,
    text_lightness: i32,
    font_size: i32,
    background_color: QColor,
    background_hue: i32,
    background_saturation: i32,
    background_lightness: i32,
    font_size_label: QString,
    contrast: f32,
    display_text_color: QColor,
    grabbed_color: QColor,
}

impl ffi::Kontrast {
    fn random(mut self: Pin<&mut Self>) {
        let mut rng = thread_rng();
        loop {
            let text_color = QColor::from_rgb(
                rng.gen_range(0..256),
                rng.gen_range(0..256),
                rng.gen_range(0..256),
            );

            let bg_color = QColor::from_rgb(
                rng.gen_range(0..256),
                rng.gen_range(0..256),
                rng.gen_range(0..256),
            );

            if contrast(&text_color, &bg_color) > 3.5 {
                self.as_mut().set_text_color(text_color);
                self.as_mut().set_background_color(bg_color);
                self.as_mut().recalc_contrast();
                self.as_mut().font_size_changed();
                break;
            }
        }
    }

    fn reverse(mut self: Pin<&mut Self>) {
        let text_color = self.text_color.clone();
        let background_color = self.background_color.clone();
        self.as_mut().set_text_color(background_color);
        self.as_mut().set_background_color(text_color);
        self.as_mut().recalc_contrast();
        self.as_mut().font_size_changed();
    }

    fn recalc_contrast(mut self: Pin<&mut Self>) {
        let text_color = self.text_color.clone();
        let background_color = self.background_color.clone();
        self.as_mut()
            .set_contrast(contrast(&text_color, &background_color));
    }
}

impl cxx_qt::Initialize for ffi::Kontrast {
    fn initialize(self: core::pin::Pin<&mut Self>) {
        self.random();
    }
}

fn luminosity(color: &QColor) -> f32 {
    let red = color.red_f();
    let green = color.green_f();
    let blue = color.blue_f();

    let red_lum = if red <= 0.03928 {
        red / 12.92
    } else {
        ((red + 0.055) / 1.055).powf(2.4)
    };
    let green_lum = if green <= 0.03928 {
        green / 12.92
    } else {
        ((green + 0.055) / 1.055).powf(2.4)
    };
    let blue_lum = if blue <= 0.03928 {
        blue / 12.92
    } else {
        ((blue + 0.055) / 1.055).powf(2.4)
    };

    return 0.2126 * red_lum + 0.7152 * green_lum + 0.0722 * blue_lum;
}

fn contrast(text_color: &QColor, bg_color: &QColor) -> f32 {
    let lum1 = luminosity(&text_color);
    let lum2 = luminosity(&bg_color);

    if lum1 > lum2 {
        return (lum1 + 0.05) / (lum2 + 0.05);
    }

    return (lum2 + 0.05) / (lum1 + 0.05);
}
