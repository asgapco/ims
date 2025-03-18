//! A module which provides functions to compress css and javascript files during production.

use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};
use minify_js::{minify, Session, TopLevelMode};
use std::{
    fs::{read_dir, read_to_string, File, OpenOptions},
    io::{Read, Write},
};

/// A constant for the path to the public/theme folder in the codebase.
const COMMON_STATIC_SOURCE_CODE_FOLDER: &str = "./../src";
/// A constant for the environment variable name.
const PACKAGE_ENVIRONMENT_VARIABLE: &str = "PKG_ENV";
/// A constant for the `prod` value of the `pkg_env` environment variable.
const PRODUCTION_PKG_ENV_VARIABLE_VALUE: &str = "prod";

/// A function which minifies both css and js files using `lightningcss` and `minify_js` when
/// the `PKG_ENV` environment and it is set to the value of `prod`.
///
/// # Error
///
/// This function returns the unit type when the minification process runs successfully otherwise
/// it returns a standard error.
pub fn compress() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(pkg_env_var) = std::env::var(PACKAGE_ENVIRONMENT_VARIABLE) {
        // A for loop that loops over each file name containing in the `colorschemes` and `themes` folders
        // and minifies it using the `lightningcss` minifier.
        if pkg_env_var == PRODUCTION_PKG_ENV_VARIABLE_VALUE {
            for file in read_dir(format!("{COMMON_STATIC_SOURCE_CODE_FOLDER}/"))? {
                let file_path = file?.path();
                let source = read_to_string(file_path.clone())?;

                let mut stylesheet = StyleSheet::parse(&source, ParserOptions::default())
                    .map_err(|err| format!("{err}\n{:?}", file_path.file_name().unwrap()))?;

                stylesheet.minify(MinifyOptions::default())?;
                let minified_css = stylesheet.to_css(PrinterOptions::default())?;

                let mut old_css_file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(file_path)?;
                old_css_file.write_all(minified_css.code.as_bytes())?;
                old_css_file.flush()?;
            }

            // A for loop that loops over each file name containing in the `public/static` folder and minifies
            // it using the `minify-js` minifier.
            for file in read_dir(COMMON_STATIC_SOURCE_CODE_FOLDER)? {
                let file_path = file?.path();
                if file_path.is_file() {
                    let mut code = Vec::new();
                    let mut js_file = File::open(file_path.clone())?;
                    js_file.read_to_end(&mut code)?;

                    drop(js_file);

                    let mut out = Vec::new();
                    minify(&Session::new(), TopLevelMode::Global, &code, &mut out)
                        .map_err(|err| format!("{err}\n{:?}", file_path.file_name().unwrap()))?;

                    let mut old_js_file = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(file_path)?;
                    old_js_file.write_all(&out)?;
                    old_js_file.flush()?;
                }
            }
        }
    }

    Ok(())
}
