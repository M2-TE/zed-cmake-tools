use zed_extension_api as zed;

// rustup default stable
// rustup target add wasm32-wasip2
// cargo build --target=wasm32-wasip2

struct CMakeTools {
    // ... state
}

impl zed::Extension for CMakeTools {
    fn new() -> Self {
        Self {}
    }
}

zed::register_extension!(CMakeTools);
