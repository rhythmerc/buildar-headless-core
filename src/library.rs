//! A game's library: built from the player's disc, loaded at run time from where they keep it, and never unloaded (the
//! game runs on its own threads in it until the process ends). Its API is C functions, looked up by name.
use std::path::Path;

/// A loaded game library. One a process: a game keeps its state in its library's globals.
pub struct GameLibrary {
    path: String,
    // (never unloaded: the game's threads run in it until the process ends)
    library: std::mem::ManuallyDrop<libloading::Library>,
}

impl GameLibrary {
    /// The library at `path`.
    ///
    /// # Safety
    /// Loading runs the library's initialisers: it must be a game library built for the API its engine expects.
    pub unsafe fn load(path: &Path) -> Result<Self, String> {
        // SAFETY: the caller vouches for the library (above).
        let library = unsafe { libloading::Library::new(path) }.map_err(|e| format!("{}: {e}", path.display()))?;
        Ok(Self { path: path.display().to_string(), library: std::mem::ManuallyDrop::new(library) })
    }

    /// The C function `name`, as `F` (an `unsafe extern "C" fn` type).
    ///
    /// # Safety
    /// `F` must be the function's real signature in the library's API.
    pub unsafe fn function<F: Copy>(&self, name: &str) -> Result<F, String> {
        assert_eq!(size_of::<F>(), size_of::<*const ()>(), "a function pointer");
        let symbol = format!("{name}\0");
        // SAFETY: the caller vouches for the signature; the library is never unloaded, so the pointer stays good.
        unsafe {
            let found = self
                .library
                .get::<*const ()>(symbol.as_bytes())
                .map_err(|e| format!("{}: {name}: {e}", self.path))?;
            let pointer: *const () = *found;
            Ok(std::mem::transmute_copy::<*const (), F>(&pointer))
        }
    }

    /// The C function `name` if the library has it (one built before it came: none).
    ///
    /// # Safety
    /// As [`GameLibrary::function`].
    pub unsafe fn optional<F: Copy>(&self, name: &str) -> Option<F> {
        // SAFETY: as the caller vouches.
        unsafe { self.function(name).ok() }
    }

    /// Where it was loaded from.
    pub fn path(&self) -> &str {
        &self.path
    }
}

/// A game library and the data it needs, as an integration test finds them: the paths in environment variables
/// `vars`, or none (the test skips itself, saying so) when any is unset.
pub fn test_paths<const N: usize>(vars: [&str; N]) -> Option<[std::path::PathBuf; N]> {
    let found: Vec<_> = vars.iter().filter_map(|v| std::env::var_os(v).map(std::path::PathBuf::from)).collect();
    if found.len() < N {
        eprintln!("{} not all set: skipped", vars.join(", "));
        return None;
    }
    found.try_into().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The C library's own functions, from a library every Linux has.
    #[test]
    #[cfg(target_os = "linux")]
    fn functions_are_found_by_name() {
        // SAFETY: the C library; strlen's signature.
        let libc = unsafe { GameLibrary::load(Path::new("libc.so.6")) }.unwrap();
        type Strlen = unsafe extern "C" fn(*const std::ffi::c_char) -> usize;
        let strlen: Strlen = unsafe { libc.function("strlen") }.unwrap();
        assert_eq!(unsafe { strlen(c"four".as_ptr()) }, 4);
        assert!(unsafe { libc.optional::<Strlen>("no_such_function_here") }.is_none());
        assert!(unsafe { libc.function::<Strlen>("no_such_function_here") }.unwrap_err().contains("no_such_function_here"));
    }

    #[test]
    fn a_test_without_its_paths_skips() {
        assert!(test_paths(["HEADLESS_CORE_SURELY_UNSET_VARIABLE"]).is_none());
    }
}
