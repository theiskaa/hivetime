/*
 * TODO: add documentation about file.
 */

/// A custom byte-position structure.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct BytePos(pub u32);

impl BytePos {
    /// Shitfts current position to [char]'s len utf8.
    pub fn shift(self, ch: char) -> Self {
        BytePos(self.0 + ch.len_utf8() as u32)
    }
}

/// A position structure that has start and end positions
/// implemented as [BytePos].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Position {
    pub start: BytePos,
    pub end: BytePos,
}

/// A value([T]) wrapper for [Position].
/// Implements:
///  - value of [T]
///  - value of [Position].
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Positioned<T> {
    pub value: T,
    pub position: Position,
}

impl Position {
    // [?] why.
    pub unsafe fn new_unchecked(start: u32, end: u32) -> Self {
        Position {
            start: BytePos(start),
            end: BytePos(end),
        }
    }

    /// Defines a [Position] with zero positions.
    pub const fn empty() -> Self {
        Position {
            start: BytePos(0),
            end: BytePos(0),
        }
    }

    /// Generates a [Position] with max values of a[Position] and b[Position].
    pub fn union_position(a: Self, b: Self) -> Self {
        use std::cmp;
        Position {
            start: cmp::min(a.start, b.start),
            end: cmp::max(a.end, b.end),
        }
    }

    /// Generates a [Position] with max values of a[Positioned] and b[Positioned].
    pub fn union<A, B>(a: &Positioned<A>, b: &Positioned<B>) -> Self {
        Self::union_position(a.into(), b.into())
    }
}

impl<T> Positioned<T> {
    pub const fn new(value: T, position: Position) -> Self {
        Positioned { value, position }
    }

    /// Creates a [Positioned] instance with empty [Position].
    pub const fn empty(value: T) -> Self {
        Self {
            value,
            position: Position {
                start: BytePos(0),
                end: BytePos(0),
            },
        }
    }

    /// [?] • creates a [Positioned] instance by unchecked type value.
    pub const unsafe fn new_unchecked(value: T, start: u32, end: u32) -> Self {
        Self {
            value,
            position: Position {
                start: BytePos(start),
                end: BytePos(end),
            },
        }
    }
}

impl<T> AsRef<Positioned<T>> for Positioned<T> {
    fn as_ref(&self) -> &Positioned<T> {
        self
    }
}

impl<T> From<Positioned<T>> for Position {
    /// Takes [Position] from [Positioned].
    fn from(this: Positioned<T>) -> Position {
        this.position
    }
}

impl<T> From<&Positioned<T>> for Position {
    /// Takes [Position] from [&Positioned].
    fn from(this: &Positioned<T>) -> Position {
        this.position
    }
}
