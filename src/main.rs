use cxx_kde_frameworks::kcoreaddons::{KAboutData, License};
use cxx_kde_frameworks::ki18n::{i18nc, KLocalizedContext, KLocalizedString};
use cxx_qt_lib::{QByteArray, QGuiApplication, QQmlApplicationEngine, QString, QUrl};

mod kontrast;

fn main() {
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    KLocalizedString::set_application_domain(&QByteArray::from("konstrast"));

    let mut about_data = KAboutData::from(
        QString::from("konstrast"),
        i18nc("@title".into(), "Kontrast".into()),
        QString::from("TEST"),
        i18nc(
            "@title".into(),
            "A constrast checker application. Now oxidized!".into(),
        ),
        License::GPL_V3,
    );

    if let Some(about_data) = about_data.as_mut() {
        about_data
            .add_author(
                &i18nc("@info:credit".into(), "Carl Schwan".into()),
                &i18nc("@info:credit".into(), "Maintainer and creator".into()),
                &QString::from("carl@carlschwan.eu"),
                &QString::from("https://carlschwan.eu"),
                &QUrl::from("https://carlschwan.eu/avatar.png"),
            )
            .add_credit(
                &i18nc("@info:credit".into(), "Wikipedia".into()),
                &i18nc(
                    "@info:credit".into(),
                    "Text on the main page CC-BY-SA-4.0".into(),
                ),
                &QString::from(""),
                &QString::from(""),
                &QUrl::from(""),
            )
            .add_author(
                &i18nc("@info:credit".into(), "Carson Black".into()),
                &i18nc(
                    "@info:credit".into(),
                    "SQLite backend for favorite colors".into(),
                ),
                &QString::from(""),
                &QString::from(""),
                &QUrl::from(""),
            )
            .set_translator(
                &i18nc("NAME OF TRANSLATORS".into(), "Your names".into()),
                &i18nc("EMAIL OF TRANSLATORS".into(), "Your emails".into()),
            );
    }

    KAboutData::set_application_data(about_data);

    if let Some(mut engine) = engine.as_mut() {
        KLocalizedContext::initialize_engine(engine.as_mut().as_qqmlengine());
        engine.load(&QUrl::from("qrc:/qt/qml/org/kde/kontrast/src/qml/Main.qml"));
    }

    // Start the app
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
