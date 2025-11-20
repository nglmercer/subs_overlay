fn main() {
    slint_build::compile("ui/overlay.slint").expect("Failed to compile overlay.slint");
    // Comentamos app-window.slint por ahora para evitar errores de compilación
    // slint_build::compile("ui/app-window.slint").unwrap();
}
