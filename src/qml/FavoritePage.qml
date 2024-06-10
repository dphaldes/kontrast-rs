/**
 * SPDX-FileCopyrightText: (C) 2020 Carl Schwan <carl@carlschwan.eu>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

import QtQuick 2.1
import QtQuick.Controls 2.14 as QQC2
import QtQuick.Layouts 1.14
import QtQuick.Window 2.14
import org.kde.kirigami 2.12 as Kirigami
import org.kde.kontrast 1.0

Kirigami.ScrollablePage {
    id: root

    property bool isMobile: Window.width <= Kirigami.Units.gridUnit * 30

    function copyColorTextToClipboard(colorText, passiveMessageText) {
        clipboard.content = colorText;
        inlineMessage.showPassive(passiveMessageText);
    }

    function copyBackground(colorText) {
        copyColorTextToClipboard(colorText, i18nc("@info:inline", "Background color copied to clipboard"));
    }

    function copyText(colorText) {
        copyColorTextToClipboard(colorText, i18nc("@info:inline", "Text color copied to clipboard"));
    }

    title: i18nc("@title:menu", "Favorite Colors")

    ListView {
        id: listview

        model: ColorStore
        spacing: Kirigami.Units.smallSpacing

        delegate: QQC2.ItemDelegate {
            width: ListView.view.width

            background: Rectangle {
                anchors.fill: parent
                color: model.backgroundColor
            }

            contentItem: ColumnLayout {
                id: layout

                Kirigami.Heading {
                    Layout.fillWidth: true
                    level: 3
                    text: "Lorem Ipsum"
                    color: model.textColor

                    QQC2.Button {
                        anchors.right: parent.right
                        icon.name: "edit-delete-remove"
                        QQC2.ToolTip.text: i18nc("@action:tooltip", "Remove")
                        QQC2.ToolTip.visible: hovered
                        onClicked: {
                            ColorStore.removeColor(model.index);
                        }
                    }

                }

                Text {
                    Layout.fillWidth: true
                    text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque et dolor velit. Morbi elementum libero non vehicula porta. Suspendisse potenti. Suspendisse eu sapien lectus."
                    wrapMode: Text.WordWrap
                    color: model.textColor
                }

                RowLayout {
                    Layout.fillWidth: true

                    QQC2.Button {
                        text: i18nc("@action:button", "Text: %1", model.textColor)
                        icon.source: "edit-copy"
                        onClicked: copyText(model.textColor)
                    }

                    QQC2.Button {
                        text: i18nc("@action:button", "Background: %1", model.backgroundColor)
                        icon.source: "edit-copy"
                        onClicked: copyBackground(model.backgroundColor)
                    }

                }

                QQC2.Button {
                    text: i18nc("@action:button", "Apply")
                    icon.name: "dialog-ok-apply"
                    onClicked: {
                        Kontrast.textColor = model.textColor;
                        Kontrast.backgroundColor = model.backgroundColor;
                        contrastChecker.trigger();
                    }
                }

            }

        }

    }

    // Workaround for QTBUG-21989
    TextInput {
        id: clipboard

        property string content

        visible: false
        onContentChanged: {
            text = content;
            selectAll();
            copy();
        }
    }

    footer: Kirigami.InlineMessage {
        id: inlineMessage

        function showPassive(message) {
            text = message;
            visible = true;
            timer.running = true;
        }

        type: Kirigami.MessageType.Information
        position: Kirigami.InlineMessage.Footer
        text: i18nc("@info:inline", "Color copied to clipboard")
        visible: false

        Timer {
            id: timer

            interval: Kirigami.Units.humanMoment
            onTriggered: inlineMessage.visible = false
        }

    }

}
