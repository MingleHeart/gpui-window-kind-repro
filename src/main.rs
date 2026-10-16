use gpui::{
    App, Bounds, Context, TitlebarOptions, Window, WindowBounds, WindowKind, WindowOptions, div,
    prelude::*, px, rgb, size,
};
use gpui_platform::application;

struct MainWindow;

struct ChildWindow {
    label: &'static str,
}

fn button(
    label: &'static str,
    on_click: impl Fn(&mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(label)
        .px_4()
        .py_3()
        .bg(rgb(0xe8eef8))
        .border_1()
        .border_color(rgb(0x8da7cc))
        .rounded_md()
        .cursor_pointer()
        .hover(|style| style.bg(rgb(0xd8e7fa)))
        .child(label)
        .on_click(move |_, window, cx| on_click(window, cx))
}

fn open_child(kind: WindowKind, label: &'static str, cx: &mut App) {
    let bounds = Bounds::centered(None, size(px(520.), px(300.)), cx);
    let options = WindowOptions {
        kind,
        titlebar: Some(TitlebarOptions {
            title: Some(label.into()),
            ..Default::default()
        }),
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        ..Default::default()
    };

    if let Err(error) = cx.open_window(options, move |window, cx| {
        // Match ConfTool's native close path: let GPUI remove the child window.
        window.on_window_should_close(cx, |window, _| {
            window.remove_window();
            false
        });
        cx.new(|_| ChildWindow { label })
    }) {
        eprintln!("Could not open {label}: {error:#}");
    }
}

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .bg(rgb(0xf8fafc))
            .text_color(rgb(0x1f2937))
            .child("GPUI 0.3.9 window focus reproduction")
            .child("Put another app window behind this one, then open and close each child.")
            .child(button("Open WindowKind::Dialog", |_, cx| {
                open_child(WindowKind::Dialog, "WindowKind::Dialog", cx);
            }))
            .child(button("Open WindowKind::Normal", |_, cx| {
                open_child(WindowKind::Normal, "WindowKind::Normal", cx);
            }))
    }
}

impl Render for ChildWindow {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .bg(rgb(0xffffff))
            .text_color(rgb(0x1f2937))
            .child(self.label)
            .child("Close with this button or the title bar X.")
            .child(button("Close child", |window, _| window.remove_window()))
    }
}

fn main() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(760.), px(440.)), cx);
        cx.open_window(
            WindowOptions {
                kind: WindowKind::Normal,
                titlebar: Some(TitlebarOptions {
                    title: Some("GPUI window kind reproduction".into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| MainWindow),
        )
        .expect("could not open the main window");
        cx.activate(true);
    });
}
