use crate::recipe::Recipe;
use std::ffi::c_int;

#[allow(non_snake_case)]
extern "C" {
    pub fn SetConsoleParams(
        bBorderless: c_int,
        bAlwaysOnTop: c_int,
        wndPos: c_int,
        wndCX: c_int,
        wndCY: c_int,
    ) -> c_int;
}

pub fn set_window_position(recipe: &Recipe) {
    #[rustfmt::skip]
        let pos = {
        let pos = recipe.get("console", "position");

        match pos.as_str() {
            "top left"     | "top_left"     | "top-left"     | "topleft"     | "tl" => 1 as c_int,
            "bottom left"  | "bottom_left"  | "bottom-left"  | "bottomleft"  | "bl" => 2 as c_int,
            "top right"    | "top_right"    | "top-right"    | "topright"    | "tr" => 3 as c_int,
            "bottom right" | "bottom_right" | "bottom-right" | "bottomright" | "br" => 4 as c_int,
            _ => {
                println!("Unknown position `{:?}`, defaulting to `top left`", pos);
                0 as c_int
            }
        }
    };
    let borderless = recipe.get_bool("console", "borderless");
    let always_on_top = recipe.get_bool("console", "stay on top");
    let width = {
        match recipe.get("console", "width").parse::<c_int>() {
            Ok(height) => height,
            Err(_) => {
                println!("Failed parsing `[console] width:` to an integer, defaulting to 800");
                800 as c_int
            }
        }
    };
    let height = {
        match recipe.get("console", "height").parse::<c_int>() {
            Ok(height) => height,
            Err(_) => {
                println!("Failed parsing `[console] height:` to an integer, defaulting to 600");
                600 as c_int
            }
        }
    };
    dbg!(&borderless);
    dbg!(&always_on_top);
    dbg!(&pos);
    dbg!(&width);
    dbg!(&height);
    unsafe {
        dbg!(SetConsoleParams(
            borderless as c_int,
            always_on_top as c_int,
            pos,
            width,
            height,
        ));
    }
}
