#[derive(Default)]
pub struct KontrastStruct {
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
    display_text_color: QColor,
    grabbed_color: QColor,
}

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
        #[qproperty(QColor, text_color, READ, WRITE, NOTIFY = text_color_changed)]
        #[qproperty(i32, text_hue, READ, WRITE, NOTIFY = text_color_changed)]
        #[qproperty(i32, text_saturation, READ, WRITE, NOTIFY = text_color_changed)]
        #[qproperty(i32, text_lightness, READ, WRITE, NOTIFY = text_color_changed)]
        #[qproperty(i32, font_size, READ, WRITE, NOTIFY = font_size_changed)]
        #[qproperty(QColor, background_color, READ, WRITE, NOTIFY = bg_color_changed)]
        #[qproperty(i32, background_hue, READ, WRITE, NOTIFY = bg_color_changed)]
        #[qproperty(i32, background_saturation, READ, WRITE, NOTIFY = bg_color_changed)]
        #[qproperty(i32, background_lightness, READ, WRITE, NOTIFY = bg_color_changed)]
        #[qproperty(QString, font_size_label, READ, WRITE,  NOTIFY = font_size_changed)]
        #[qproperty(f32, contrast, READ = contrast,  NOTIFY = contrast_changed)]
        #[qproperty(QColor, display_text_color, READ, WRITE,   NOTIFY = contrast_changed)]
        #[qproperty(QColor, grabbed_color)]
        type Kontrast = super::KontrastStruct;

        #[qinvokable]
        fn random(self: Pin<&mut Kontrast>);

        #[qinvokable]
        fn reverse(self: Pin<&mut Kontrast>);

        #[qinvokable]
        fn contrast(self: &Kontrast) -> f32;

        #[cxx_name = "text_color_changed"]
        #[qsignal]
        fn text_color_changed(self: Pin<&mut Kontrast>);

        #[cxx_name = "font_size_changed"]
        #[qsignal]
        fn font_size_changed(self: Pin<&mut Kontrast>);

        #[cxx_name = "bg_color_changed"]
        #[qsignal]
        fn bg_color_changed(self: Pin<&mut Kontrast>);

        #[cxx_name = "contrast_changed"]
        #[qsignal]
        fn contrast_changed(self: Pin<&mut Kontrast>);

    }

    impl cxx_qt::Constructor<()> for Kontrast {}
}

use std::pin::Pin;

use cxx_qt_lib::{QColor, QString};
use rand::{thread_rng, Rng};

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
            self.as_mut().set_text_color(text_color);
            self.as_mut().set_background_color(bg_color);

            if self.contrast() > 3.5 {
                self.as_mut().contrast_changed();
                self.as_mut().text_color_changed();
                self.as_mut().bg_color_changed();
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
        self.as_mut().font_size_changed();
    }

    fn contrast(self: &Self) -> f32 {
        let lum1 = luminosity(self.text_color());
        let lum2 = luminosity(self.background_color());

        if lum1 > lum2 {
            return (lum1 + 0.05) / (lum2 + 0.05);
        }

        return (lum2 + 0.05) / (lum1 + 0.05);
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
