use cxx_kde_frameworks::kcoreaddons::{KAboutData, KAuthor, KCredit, KTranslator, License};
use cxx_kde_frameworks::ki18n::{i18nc, KLocalizedContext, KLocalizedString};
use cxx_qt_lib::{QByteArray, QGuiApplication, QQmlApplicationEngine, QString, QUrl};

mod kontrast;

fn main() {
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    KLocalizedString::set_application_domain(&QByteArray::from("konstrast"));

    let mut about_data = KAboutData::from(
        QString::from("konstrast"),
        i18nc("@title", "Kontrast"),
        QString::from("TEST"),
        i18nc("@title", "A constrast checker application. Now oxidized!"),
        License::GPL_V3,
    );

    if let Some(about_data) = about_data.as_mut() {
        about_data
            .add_author(KAuthor {
                name: i18nc("@info:credit", "Carl Schwan"),
                task: i18nc("@info:credit", "Maintainer and creator"),
                email_address: QString::from("carl@carlschwan.eu"),
                web_address: QString::from("https://carlschwan.eu"),
                avatar_url: QUrl::from("https://carlschwan.eu/avatar.png"),
            })
            .add_credit(KCredit {
                name: i18nc("@info:credit", "Wikipedia"),
                task: i18nc("@info:credit", "Text on the main page CC-BY-SA-4.0"),
                ..Default::default()
            })
            .add_author(KAuthor {
                name: i18nc("@info:credit", "Carson Black"),
                task: i18nc("@info:credit", "SQLite backend for favorite colors"),
                ..Default::default()
            })
            .set_translator(KTranslator {
                name: i18nc("NAME OF TRANSLATORS", "Your names"),
                email_address: i18nc("EMAIL OF TRANSLATORS", "Your emails"),
            });
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
