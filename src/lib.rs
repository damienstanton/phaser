//! # Phase Distintinctions in Type Theory, Harper, 2021
//! 
//! [Youtube link][yt]
//!
//! [yt]: https://youtu.be/7DYkyB1Rm3I
//!
//! ```text
//! signature QUEUE = sig
//!   type elt
//!   type t
//!   val emp : t
//!   val ins : elt * t -> t
//!   val rem : t -> (elt * t) option
//! end
//!
//! structure QL :> QUEUE = struct
//!   type elt = bool
//!   type t = elt list
//!   val emp = nil
//!   val ins = cons
//!   fun rem nil = NONE
//!     | let val (x,q') = rev q in SOME (x, rev q')
//! end
//! ```
//!
//! - Types such as $QL.elt \rightharpoonup QL.t$ threaten the phase distinction
//! - $q: QUEUE \vdash M : QUEUE$ separates into two components
//!   - $M^{st}$ defines $elt$ and $t$ in terms of $q^{st}.elt$ and $q^{st}.t$.
//!   - $M^{dy}$ defines $emp$, etc. in terms of types and values in $q$.
#![allow(dead_code, unused_imports, unused_variables)]

/// is a simple example based on the SML data structure presented in the talk.
pub trait Queue<T> {
    fn emp() -> Self;
    fn ins(&mut self, elt: &T);
    fn rem(&mut self, elt: &T) -> Option<T>;
}

use std::collections::VecDeque;

/// is a toy data structure.
impl Queue<bool> for VecDeque<bool> {
    /// produces an empty vector.
    fn emp() -> Self { VecDeque::default() }

    /// inserts an element into the queue.
    fn ins(&mut self, elt: &bool) {
        self.push_front(*elt)
    }

    /// deletes the given entry in the queue, returning a reference to `T`, if
    /// any exists.
    fn rem(&mut self, elt: &bool) -> Option<bool> {
        let i = self.binary_search(elt).or::<usize>(Ok(0)).unwrap();
        let found = i >= 1;
        if !found { return None; }
        self.remove(i)
    }
}