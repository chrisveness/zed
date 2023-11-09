// This file was generated by the `theme_importer`.
// Be careful when modifying it by hand.

use gpui::rgba;

use crate::{
    Appearance, StatusColorsRefinement, ThemeColorsRefinement, UserTheme, UserThemeFamily,
    UserThemeStylesRefinement,
};

pub fn nord() -> UserThemeFamily {
    UserThemeFamily {
        name: "Nord".into(),
        author: "Sven Greb (svengreb)".into(),
        themes: vec![UserTheme {
            name: "Nord".into(),
            appearance: Appearance::Dark,
            styles: UserThemeStylesRefinement {
                colors: ThemeColorsRefinement {
                    border: Some(rgba(0x3b4252ff).into()),
                    border_variant: Some(rgba(0x3b4252ff).into()),
                    border_focused: Some(rgba(0x3b4252ff).into()),
                    border_selected: Some(rgba(0x3b4252ff).into()),
                    border_transparent: Some(rgba(0x3b4252ff).into()),
                    border_disabled: Some(rgba(0x3b4252ff).into()),
                    elevated_surface_background: Some(rgba(0x2e3440ff).into()),
                    surface_background: Some(rgba(0x2e3440ff).into()),
                    background: Some(rgba(0x2e3440ff).into()),
                    element_background: Some(rgba(0x88bfd0ee).into()),
                    element_hover: Some(rgba(0x3b4252ff).into()),
                    element_selected: Some(rgba(0x88bfd0ff).into()),
                    drop_target_background: Some(rgba(0x88bfd099).into()),
                    ghost_element_hover: Some(rgba(0x3b4252ff).into()),
                    text: Some(rgba(0xd8dee9ff).into()),
                    tab_inactive_background: Some(rgba(0x2e3440ff).into()),
                    tab_active_background: Some(rgba(0x3b4252ff).into()),
                    editor_background: Some(rgba(0x2e3440ff).into()),
                    editor_gutter_background: Some(rgba(0x2e3440ff).into()),
                    editor_line_number: Some(rgba(0x4c566aff).into()),
                    editor_active_line_number: Some(rgba(0xd8dee9ff).into()),
                    terminal_background: Some(rgba(0x2e3440ff).into()),
                    terminal_ansi_bright_black: Some(rgba(0x4c566aff).into()),
                    terminal_ansi_bright_red: Some(rgba(0xbf616aff).into()),
                    terminal_ansi_bright_green: Some(rgba(0xa3be8cff).into()),
                    terminal_ansi_bright_yellow: Some(rgba(0xebcb8bff).into()),
                    terminal_ansi_bright_blue: Some(rgba(0x81a1c1ff).into()),
                    terminal_ansi_bright_magenta: Some(rgba(0xb48eacff).into()),
                    terminal_ansi_bright_cyan: Some(rgba(0x8fbcbbff).into()),
                    terminal_ansi_bright_white: Some(rgba(0xeceff4ff).into()),
                    terminal_ansi_black: Some(rgba(0x3b4252ff).into()),
                    terminal_ansi_red: Some(rgba(0xbf616aff).into()),
                    terminal_ansi_green: Some(rgba(0xa3be8cff).into()),
                    terminal_ansi_yellow: Some(rgba(0xebcb8bff).into()),
                    terminal_ansi_blue: Some(rgba(0x81a1c1ff).into()),
                    terminal_ansi_magenta: Some(rgba(0xb48eacff).into()),
                    terminal_ansi_cyan: Some(rgba(0x88bfd0ff).into()),
                    terminal_ansi_white: Some(rgba(0xe5e9f0ff).into()),
                    ..Default::default()
                },
                status: StatusColorsRefinement {
                    deleted: Some(rgba(0xbf616aff).into()),
                    error: Some(rgba(0xbf616aff).into()),
                    hidden: Some(rgba(0xd8dee966).into()),
                    warning: Some(rgba(0xebcb8bff).into()),
                    ..Default::default()
                },
                syntax: Some(UserSyntaxTheme {
                    highlights: vec![
                        ("comment".into(), rgba(0x606e87ff).into()),
                        ("punctuation".into(), rgba(0x81a1c1ff).into()),
                        ("something".into(), rgba(0xa3be8cff).into()),
                    ],
                }),
            },
        }],
    }
}
