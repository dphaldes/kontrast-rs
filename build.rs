use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qrc("src/resources.qrc")
        .qml_module(QmlModule {
            uri: "org.kde.kontrast",
            qml_files: &[
                "src/qml/Main.qml",
                "src/qml/MainPage.qml",
                "src/qml/AboutPage.qml",
                "src/qml/FavoritePage.qml",
                "src/qml/HelpPage.qml",
            ],
            rust_files: &["src/kontrast.rs"],
            ..Default::default()
        })
        // .cc_builder(|cc| {
        //     cc.define("QML_IMPORT_TRACE", None);
        // })
        .build();
}
