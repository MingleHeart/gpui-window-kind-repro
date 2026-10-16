# GPUI 0.3.9 Windows dialog focus reproduction

A minimal, standalone GPUI app for comparing `WindowKind::Dialog` and `WindowKind::Normal` on Windows. It uses the published `gpui-pre` and `gpui-pre-platform` 0.3.9 crates, with no local patch.

## Run

```powershell
cargo run
```

## Reproduce

1. Leave another application window visible behind the reproduction app's main window.
2. Click **Open WindowKind::Dialog**, then close the child with its title bar X or **Close child**.
3. Repeat several times. Watch the main window and the other application window during closure.
4. Compare with **Open WindowKind::Normal**.

Expected: closing either child returns to the appropriate foreground window without a visible flash. With the stock Windows backend, the dialog close path hides the child before re-enabling its owner, then calls `SetForegroundWindow(parent_hwnd)` during `WM_DESTROY`. This can briefly change the foreground order. The normal window does not use that modal owner path.

The `on_window_should_close` callback calls `remove_window()` to reproduce the close path used by ConfTool. Both child windows use the native Windows title bar so the difference is limited to `WindowKind`.

Upstream snapshot: `gpui-pre-windows 0.3.9`, Zed revision `93d745668805029476e6d7fbb19652116fed51de`.

## Observed on Windows

With another application window behind the main window, foreground-window sampling during the native title-bar close path showed:

- `Dialog` child → unrelated application window → reproduction app main window.
- `Normal` child → reproduction app main window directly.

The intermediate activation explains the visible flash. The sample uses unmodified published crates, so it can accompany an upstream issue report without ConfTool's local workaround.
