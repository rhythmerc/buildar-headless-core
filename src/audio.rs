//! A game's sound, taken from its library as the game makes it: a C function that gives up to so many frames of its
//! final mix (16-bit, left and right interleaved) and says how many it gave. Callable from any thread (the library keeps
//! the mix behind a lock), so it can go to the runtime's sound thread.

/// The library's function that gives its mix: `(out, frames) -> frames given`.
pub type TakeMix = unsafe extern "C" fn(*mut i16, u32) -> u32;

/// Where a game's sound is taken from.
#[derive(Clone, Copy)]
pub struct AudioTap(TakeMix);

impl AudioTap {
    /// The tap of a library's mix function.
    ///
    /// # Safety
    /// `take` must write at most the frames it is asked for, two samples each, and be callable from any thread.
    pub unsafe fn new(take: TakeMix) -> Self {
        Self(take)
    }

    /// Up to `out.len() / 2` frames of the game's mix (left and right interleaved): the frames given.
    pub fn take(&self, out: &mut [i16]) -> usize {
        // SAFETY: the library writes at most the frames asked for, two samples each (`new`).
        unsafe { (self.0)(out.as_mut_ptr(), (out.len() / 2) as u32) as usize }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    unsafe extern "C" fn three_frames(out: *mut i16, frames: u32) -> u32 {
        let n = frames.min(3);
        for i in 0..(2 * n) as usize {
            // SAFETY: at most the frames asked for.
            unsafe { *out.add(i) = i as i16 }
        }
        n
    }

    #[test]
    fn a_tap_takes_what_the_game_gives() {
        let tap = unsafe { AudioTap::new(three_frames) };
        let mut out = [0i16; 10];
        assert_eq!(tap.take(&mut out), 3);
        assert_eq!(out[..6], [0, 1, 2, 3, 4, 5]);
        assert_eq!(tap.take(&mut out[..3]), 1, "an odd sample left over is not a frame");
    }
}
