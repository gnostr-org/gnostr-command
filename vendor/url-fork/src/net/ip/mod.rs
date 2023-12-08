// Effectively all the code in this repo is copied with permission from Rust's std library.
// They hold the copyright (http://rust-lang.org/COPYRIGHT) and whatever other rights, but this
// crate is MIT licensed also, so it's all good.

// Tests for this module
#[cfg(all(test, not(target_os = "emscripten")))]
mod tests;

use core::cmp::Ordering;
use core::fmt::{self, Write};
use core::hash;

use super::helper::WriteHelper;

/// An IP address, either IPv4 or IPv6.
///
/// This enum can contain either an [`Ipv4Addr`] or an [`Ipv6Addr`], see their
/// respective documentation for more details.
#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum IpAddr {
    /// An IPv4 address.
    V4(Ipv4Addr),
    /// An IPv6 address.
    V6(Ipv6Addr),
}

/// An IPv4 address.
///
/// IPv4 addresses are defined as 32-bit integers in [IETF RFC 791].
/// They are usually represented as four octets.
///
/// See [`IpAddr`] for a type encompassing both IPv4 and IPv6 addresses.
///
/// [IETF RFC 791]: https://tools.ietf.org/html/rfc791
///
/// # Textual representation
///
/// `Ipv4Addr` provides a [`FromStr`] implementation. The four octets are in decimal
/// notation, divided by `.` (this is called "dot-decimal notation").
/// Notably, octal numbers and hexadecimal numbers are not allowed per [IETF RFC 6943].
///
/// [IETF RFC 6943]: https://tools.ietf.org/html/rfc6943#section-3.1.1
/// [`FromStr`]: core::str::FromStr
#[derive(Copy)]
pub struct Ipv4Addr {
    // Octets stored in transmit order.
    inner: [u8; 4],
}

/// An IPv6 address.
///
/// IPv6 addresses are defined as 128-bit integers in [IETF RFC 4291].
/// They are usually represented as eight 16-bit segments.
///
/// See [`IpAddr`] for a type encompassing both IPv4 and IPv6 addresses.
///
/// [IETF RFC 4291]: https://tools.ietf.org/html/rfc4291
///
/// # Textual representation
///
/// `Ipv6Addr` provides a [`FromStr`] implementation. There are many ways to represent
/// an IPv6 address in text, but in general, each segments is written in hexadecimal
/// notation, and segments are separated by `:`. For more information, see
/// [IETF RFC 5952].
///
/// [`FromStr`]: core::str::FromStr
/// [IETF RFC 5952]: https://tools.ietf.org/html/rfc5952
#[derive(Copy)]
pub struct Ipv6Addr {
    // Octets stored in transmit order.
    inner: [u8; 16],
}

impl IpAddr {
    /// Returns [`true`] for the special 'unspecified' address.
    ///
    /// See the documentation for [`Ipv4Addr::is_unspecified()`] and
    /// [`Ipv6Addr::is_unspecified()`] for more details.
    #[inline]
    pub const fn is_unspecified(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_unspecified(),
            IpAddr::V6(ip) => ip.is_unspecified(),
        }
    }

    /// Returns [`true`] if this is a loopback address.
    ///
    /// See the documentation for [`Ipv4Addr::is_loopback()`] and
    /// [`Ipv6Addr::is_loopback()`] for more details.
    #[inline]
    pub const fn is_loopback(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_loopback(),
            IpAddr::V6(ip) => ip.is_loopback(),
        }
    }

    /// Returns [`true`] if this is a multicast address.
    ///
    /// See the documentation for [`Ipv4Addr::is_multicast()`] and
    /// [`Ipv6Addr::is_multicast()`] for more details.
    #[inline]
    pub const fn is_multicast(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_multicast(),
            IpAddr::V6(ip) => ip.is_multicast(),
        }
    }

    /// Returns [`true`] if this address is an [`IPv4` address], and [`false`]
    /// otherwise.
    ///
    /// [`IPv4` address]: IpAddr::V4
    #[inline]
    pub const fn is_ipv4(&self) -> bool {
        matches!(self, IpAddr::V4(_))
    }

    /// Returns [`true`] if this address is an [`IPv6` address], and [`false`]
    /// otherwise.
    ///
    /// [`IPv6` address]: IpAddr::V6
    #[inline]
    pub const fn is_ipv6(&self) -> bool {
        matches!(self, IpAddr::V6(_))
    }
}

impl Ipv4Addr {
    /// Creates a new IPv4 address from four eight-bit octets.
    ///
    /// The result will represent the IP address `a`.`b`.`c`.`d`.
    #[inline]
    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Ipv4Addr {
        Ipv4Addr {
            inner: [a, b, c, d],
        }
    }

    /// An IPv4 address with the address pointing to localhost: `127.0.0.1`
    pub const LOCALHOST: Self = Ipv4Addr::new(127, 0, 0, 1);

    /// An IPv4 address representing an unspecified address: `0.0.0.0`
    ///
    /// This corresponds to the constant `INADDR_ANY` in other languages.
    #[doc(alias = "INADDR_ANY")]
    pub const UNSPECIFIED: Self = Ipv4Addr::new(0, 0, 0, 0);

    /// An IPv4 address representing the broadcast address: `255.255.255.255`
    pub const BROADCAST: Self = Ipv4Addr::new(255, 255, 255, 255);

    /// Returns the four eight-bit integers that make up this address.
    #[inline]
    pub const fn octets(&self) -> [u8; 4] {
        self.inner
    }

    /// Returns [`true`] for the special 'unspecified' address (`0.0.0.0`).
    ///
    /// This property is defined in _UNIX Network Programming, Second Edition_,
    /// W. Richard Stevens, p. 891; see also [ip7].
    ///
    /// [ip7]: http://man7.org/linux/man-pages/man7/ip.7.html
    #[inline]
    pub const fn is_unspecified(&self) -> bool {
        self.inner[0] == 0 && self.inner[1] == 0 && self.inner[2] == 0 && self.inner[3] == 0
    }

    /// Returns [`true`] if this is a loopback address (`127.0.0.0/8`).
    ///
    /// This property is defined by [IETF RFC 1122].
    ///
    /// [IETF RFC 1122]: https://tools.ietf.org/html/rfc1122
    #[inline]
    pub const fn is_loopback(&self) -> bool {
        self.octets()[0] == 127
    }

    /// Returns [`true`] if this is a private address.
    ///
    /// The private address ranges are defined in [IETF RFC 1918] and include:
    ///
    ///  - `10.0.0.0/8`
    ///  - `172.16.0.0/12`
    ///  - `192.168.0.0/16`
    ///
    /// [IETF RFC 1918]: https://tools.ietf.org/html/rfc1918
    #[inline]
    pub const fn is_private(&self) -> bool {
        match self.octets() {
            [10, ..] => true,
            [172, b, ..] if b >= 16 && b <= 31 => true,
            [192, 168, ..] => true,
            _ => false,
        }
    }

    /// Returns [`true`] if the address is link-local (`169.254.0.0/16`).
    ///
    /// This property is defined by [IETF RFC 3927].
    ///
    /// [IETF RFC 3927]: https://tools.ietf.org/html/rfc3927
    #[inline]
    pub const fn is_link_local(&self) -> bool {
        matches!(self.octets(), [169, 254, ..])
    }

    /// Returns [`true`] if this is a multicast address (`224.0.0.0/4`).
    ///
    /// Multicast addresses have a most significant octet between `224` and `239`,
    /// and is defined by [IETF RFC 5771].
    ///
    /// [IETF RFC 5771]: https://tools.ietf.org/html/rfc5771
    #[inline]
    pub const fn is_multicast(&self) -> bool {
        self.octets()[0] >= 224 && self.octets()[0] <= 239
    }

    /// Returns [`true`] if this is a broadcast address (`255.255.255.255`).
    ///
    /// A broadcast address has all octets set to `255` as defined in [IETF RFC 919].
    ///
    /// [IETF RFC 919]: https://tools.ietf.org/html/rfc919
    #[inline]
    pub const fn is_broadcast(&self) -> bool {
        u32::from_be_bytes(self.octets()) == u32::from_be_bytes(Self::BROADCAST.octets())
    }

    /// Returns [`true`] if this address is in a range designated for documentation.
    ///
    /// This is defined in [IETF RFC 5737]:
    ///
    /// - `192.0.2.0/24` (TEST-NET-1)
    /// - `198.51.100.0/24` (TEST-NET-2)
    /// - `203.0.113.0/24` (TEST-NET-3)
    ///
    /// [IETF RFC 5737]: https://tools.ietf.org/html/rfc5737
    #[inline]
    pub const fn is_documentation(&self) -> bool {
        matches!(
            self.octets(),
            [192, 0, 2, _] | [198, 51, 100, _] | [203, 0, 113, _]
        )
    }

    /// Converts this address to an IPv4-compatible [`IPv6` address].
    ///
    /// `a.b.c.d` becomes `::a.b.c.d`
    ///
    /// This isn't typically the method you want; these addresses don't typically
    /// function on modern systems. Use `to_ipv6_mapped` instead.
    ///
    /// [`IPv6` address]: Ipv6Addr
    #[inline]
    pub const fn to_ipv6_compatible(self) -> Ipv6Addr {
        let [a, b, c, d] = self.octets();
        Ipv6Addr::new(
            0,
            0,
            0,
            0,
            0,
            0,
            ((a as u16) << 8) | b as u16,
            ((c as u16) << 8) | d as u16,
        )
    }

    /// Converts this address to an IPv4-mapped [`IPv6` address].
    ///
    /// `a.b.c.d` becomes `::ffff:a.b.c.d`
    ///
    /// [`IPv6` address]: Ipv6Addr
    #[inline]
    pub const fn to_ipv6_mapped(self) -> Ipv6Addr {
        let [a, b, c, d] = self.octets();
        Ipv6Addr::new(
            0,
            0,
            0,
            0,
            0,
            0xffff,
            ((a as u16) << 8) | b as u16,
            ((c as u16) << 8) | d as u16,
        )
    }
}

impl fmt::Display for IpAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpAddr::V4(ip) => ip.fmt(fmt),
            IpAddr::V6(ip) => ip.fmt(fmt),
        }
    }
}

impl fmt::Debug for IpAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}

impl From<Ipv4Addr> for IpAddr {
    /// Copies this address to a new `IpAddr::V4`.
    #[inline]
    fn from(ipv4: Ipv4Addr) -> IpAddr {
        IpAddr::V4(ipv4)
    }
}

impl From<Ipv6Addr> for IpAddr {
    /// Copies this address to a new `IpAddr::V6`.
    #[inline]
    fn from(ipv6: Ipv6Addr) -> IpAddr {
        IpAddr::V6(ipv6)
    }
}

impl fmt::Display for Ipv4Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let octets = self.octets();
        // Fast Path: if there's no alignment stuff, write directly to the buffer
        if fmt.precision().is_none() && fmt.width().is_none() {
            write!(
                fmt,
                "{}.{}.{}.{}",
                octets[0], octets[1], octets[2], octets[3]
            )
        } else {
            const IPV4_BUF_LEN: usize = 15; // Long enough for the longest possible IPv4 address
            let mut buf = [0u8; IPV4_BUF_LEN];
            let mut buf_slice = WriteHelper::new(&mut buf[..]);

            // Note: The call to write should never fail, hence the unwrap
            write!(
                buf_slice,
                "{}.{}.{}.{}",
                octets[0], octets[1], octets[2], octets[3]
            )
            .unwrap();
            let len = IPV4_BUF_LEN - buf_slice.into_raw().len();

            // This unsafe is OK because we know what is being written to the buffer
            let buf = unsafe { core::str::from_utf8_unchecked(&buf[..len]) };
            fmt.pad(buf)
        }
    }
}

impl fmt::Debug for Ipv4Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}

impl Clone for Ipv4Addr {
    #[inline]
    fn clone(&self) -> Ipv4Addr {
        *self
    }
}

impl PartialEq for Ipv4Addr {
    #[inline]
    fn eq(&self, other: &Ipv4Addr) -> bool {
        self.inner == other.inner
    }
}

impl PartialEq<Ipv4Addr> for IpAddr {
    #[inline]
    fn eq(&self, other: &Ipv4Addr) -> bool {
        match self {
            IpAddr::V4(v4) => v4 == other,
            IpAddr::V6(_) => false,
        }
    }
}

impl PartialEq<IpAddr> for Ipv4Addr {
    #[inline]
    fn eq(&self, other: &IpAddr) -> bool {
        match other {
            IpAddr::V4(v4) => self == v4,
            IpAddr::V6(_) => false,
        }
    }
}

impl Eq for Ipv4Addr {}

impl hash::Hash for Ipv4Addr {
    #[inline]
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        self.inner.hash(s)
    }
}

impl PartialOrd for Ipv4Addr {
    #[inline]
    fn partial_cmp(&self, other: &Ipv4Addr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<Ipv4Addr> for IpAddr {
    #[inline]
    fn partial_cmp(&self, other: &Ipv4Addr) -> Option<Ordering> {
        match self {
            IpAddr::V4(v4) => v4.partial_cmp(other),
            IpAddr::V6(_) => Some(Ordering::Greater),
        }
    }
}

impl PartialOrd<IpAddr> for Ipv4Addr {
    #[inline]
    fn partial_cmp(&self, other: &IpAddr) -> Option<Ordering> {
        match other {
            IpAddr::V4(v4) => self.partial_cmp(v4),
            IpAddr::V6(_) => Some(Ordering::Less),
        }
    }
}

impl Ord for Ipv4Addr {
    #[inline]
    fn cmp(&self, other: &Ipv4Addr) -> Ordering {
        // Compare as native endian
        u32::from_be_bytes(self.inner).cmp(&u32::from_be_bytes(other.inner))
    }
}

impl From<Ipv4Addr> for u32 {
    /// Converts an `Ipv4Addr` into a host byte order `u32`.
    #[inline]
    fn from(ip: Ipv4Addr) -> u32 {
        let ip = ip.octets();
        u32::from_be_bytes(ip)
    }
}

impl From<u32> for Ipv4Addr {
    /// Converts a host byte order `u32` into an `Ipv4Addr`.
    #[inline]
    fn from(ip: u32) -> Ipv4Addr {
        Ipv4Addr::from(ip.to_be_bytes())
    }
}

impl From<[u8; 4]> for Ipv4Addr {
    /// Creates an `Ipv4Addr` from a four element byte array.
    #[inline]
    fn from(octets: [u8; 4]) -> Ipv4Addr {
        Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3])
    }
}

impl From<[u8; 4]> for IpAddr {
    /// Creates an `IpAddr::V4` from a four element byte array.
    #[inline]
    fn from(octets: [u8; 4]) -> IpAddr {
        IpAddr::V4(Ipv4Addr::from(octets))
    }
}

impl Ipv6Addr {
    /// Creates a new IPv6 address from eight 16-bit segments.
    ///
    /// The result will represent the IP address `a:b:c:d:e:f:g:h`.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::many_single_char_names)]
    pub const fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Ipv6Addr {
        Ipv6Addr {
            inner: [
                (a >> 8) as u8,
                a as u8,
                (b >> 8) as u8,
                b as u8,
                (c >> 8) as u8,
                c as u8,
                (d >> 8) as u8,
                d as u8,
                (e >> 8) as u8,
                e as u8,
                (f >> 8) as u8,
                f as u8,
                (g >> 8) as u8,
                g as u8,
                (h >> 8) as u8,
                h as u8,
            ],
        }
    }

    /// An IPv6 address representing localhost: `::1`.
    pub const LOCALHOST: Self = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);

    /// An IPv6 address representing the unspecified address: `::`
    pub const UNSPECIFIED: Self = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0);

    /// Returns the eight 16-bit segments that make up this address.
    #[inline]
    pub const fn segments(&self) -> [u16; 8] {
        let arr = self.octets();
        [
            (arr[0] as u16) << 8 | (arr[1] as u16),
            (arr[2] as u16) << 8 | (arr[3] as u16),
            (arr[4] as u16) << 8 | (arr[5] as u16),
            (arr[6] as u16) << 8 | (arr[7] as u16),
            (arr[8] as u16) << 8 | (arr[9] as u16),
            (arr[10] as u16) << 8 | (arr[11] as u16),
            (arr[12] as u16) << 8 | (arr[13] as u16),
            (arr[14] as u16) << 8 | (arr[15] as u16),
        ]
    }

    #[inline]
    pub const fn is_unspecified(&self) -> bool {
        u128::from_be_bytes(self.octets()) == u128::from_be_bytes(Ipv6Addr::UNSPECIFIED.octets())
    }

    #[inline]
    pub const fn is_loopback(&self) -> bool {
        u128::from_be_bytes(self.octets()) == u128::from_be_bytes(Ipv6Addr::LOCALHOST.octets())
    }

    /// Returns [`true`] if this is a multicast address (`ff00::/8`).
    ///
    /// This property is defined by [IETF RFC 4291].
    ///
    /// [IETF RFC 4291]: https://tools.ietf.org/html/rfc4291
    #[inline]
    pub const fn is_multicast(&self) -> bool {
        (self.segments()[0] & 0xff00) == 0xff00
    }

    /// Converts this address to an [`IPv4` address]. Returns [`None`] if this address is
    /// neither IPv4-compatible or IPv4-mapped.
    ///
    /// `::a.b.c.d` and `::ffff:a.b.c.d` become `a.b.c.d`
    ///
    /// [`IPv4` address]: Ipv4Addr
    #[inline]
    pub const fn to_ipv4(self) -> Option<Ipv4Addr> {
        if let [0, 0, 0, 0, 0, 0 | 0xffff, ab, cd] = self.segments() {
            let [a, b] = ab.to_be_bytes();
            let [c, d] = cd.to_be_bytes();
            Some(Ipv4Addr::new(a, b, c, d))
        } else {
            None
        }
    }

    #[inline]
    pub const fn octets(&self) -> [u8; 16] {
        self.inner
    }
}

/// Write an Ipv6Addr, conforming to the canonical style described by
/// [RFC 5952](https://tools.ietf.org/html/rfc5952).
impl fmt::Display for Ipv6Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // If there are no alignment requirements, write out the IP address to
        // f. Otherwise, write it to a local buffer, then use f.pad.
        if f.precision().is_none() && f.width().is_none() {
            let segments = self.segments();

            // Special case for :: and ::1; otherwise they get written with the
            // IPv4 formatter
            if self.is_unspecified() {
                f.write_str("::")
            } else if self.is_loopback() {
                f.write_str("::1")
            } else if let Some(ipv4) = self.to_ipv4() {
                match segments[5] {
                    // IPv4 Compatible address
                    0 => write!(f, "::{}", ipv4),
                    // IPv4 Mapped address
                    0xffff => write!(f, "::ffff:{}", ipv4),
                    _ => unreachable!(),
                }
            } else {
                #[derive(Copy, Clone, Default)]
                struct Span {
                    start: usize,
                    len: usize,
                }

                // Find the inner 0 span
                let zeroes = {
                    let mut longest = Span::default();
                    let mut current = Span::default();

                    for (i, &segment) in segments.iter().enumerate() {
                        if segment == 0 {
                            if current.len == 0 {
                                current.start = i;
                            }

                            current.len += 1;

                            if current.len > longest.len {
                                longest = current;
                            }
                        } else {
                            current = Span::default();
                        }
                    }

                    longest
                };

                /// Write a colon-separated part of the address
                #[inline]
                fn fmt_subslice(f: &mut fmt::Formatter<'_>, chunk: &[u16]) -> fmt::Result {
                    if let Some((first, tail)) = chunk.split_first() {
                        write!(f, "{:x}", first)?;
                        for segment in tail {
                            f.write_char(':')?;
                            write!(f, "{:x}", segment)?;
                        }
                    }
                    Ok(())
                }

                if zeroes.len > 1 {
                    fmt_subslice(f, &segments[..zeroes.start])?;
                    f.write_str("::")?;
                    fmt_subslice(f, &segments[zeroes.start + zeroes.len..])
                } else {
                    fmt_subslice(f, &segments)
                }
            }
        } else {
            // Slow path: write the address to a local buffer, the use f.pad.
            // Defined recursively by using the fast path to write to the
            // buffer.

            // This is the largest possible size of an IPv6 address
            const IPV6_BUF_LEN: usize = (4 * 8) + 7;
            let mut buf = [0u8; IPV6_BUF_LEN];
            let mut buf_slice = WriteHelper::new(&mut buf[..]);

            // Note: This call to write should never fail, so unwrap is okay.
            write!(buf_slice, "{}", self).unwrap();
            let len = IPV6_BUF_LEN - buf_slice.into_raw().len();

            // This is safe because we know exactly what can be in this buffer
            let buf = unsafe { core::str::from_utf8_unchecked(&buf[..len]) };
            f.pad(buf)
        }
    }
}

impl fmt::Debug for Ipv6Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}

impl Clone for Ipv6Addr {
    #[inline]
    fn clone(&self) -> Ipv6Addr {
        *self
    }
}

impl PartialEq for Ipv6Addr {
    #[inline]
    fn eq(&self, other: &Ipv6Addr) -> bool {
        self.inner == other.inner
    }
}

impl PartialEq<IpAddr> for Ipv6Addr {
    #[inline]
    fn eq(&self, other: &IpAddr) -> bool {
        match other {
            IpAddr::V4(_) => false,
            IpAddr::V6(v6) => self == v6,
        }
    }
}

impl PartialEq<Ipv6Addr> for IpAddr {
    #[inline]
    fn eq(&self, other: &Ipv6Addr) -> bool {
        match self {
            IpAddr::V4(_) => false,
            IpAddr::V6(v6) => v6 == other,
        }
    }
}

impl Eq for Ipv6Addr {}

impl hash::Hash for Ipv6Addr {
    #[inline]
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        self.inner.hash(s)
    }
}

impl PartialOrd for Ipv6Addr {
    #[inline]
    fn partial_cmp(&self, other: &Ipv6Addr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<Ipv6Addr> for IpAddr {
    #[inline]
    fn partial_cmp(&self, other: &Ipv6Addr) -> Option<Ordering> {
        match self {
            IpAddr::V4(_) => Some(Ordering::Less),
            IpAddr::V6(v6) => v6.partial_cmp(other),
        }
    }
}

impl PartialOrd<IpAddr> for Ipv6Addr {
    #[inline]
    fn partial_cmp(&self, other: &IpAddr) -> Option<Ordering> {
        match other {
            IpAddr::V4(_) => Some(Ordering::Greater),
            IpAddr::V6(v6) => self.partial_cmp(v6),
        }
    }
}

impl Ord for Ipv6Addr {
    #[inline]
    fn cmp(&self, other: &Ipv6Addr) -> Ordering {
        self.segments().cmp(&other.segments())
    }
}
impl From<Ipv6Addr> for u128 {
    /// Convert an `Ipv6Addr` into a host byte order `u128`.
    #[inline]
    fn from(ip: Ipv6Addr) -> u128 {
        let ip = ip.octets();
        u128::from_be_bytes(ip)
    }
}
impl From<u128> for Ipv6Addr {
    /// Convert a host byte order `u128` into an `Ipv6Addr`.
    #[inline]
    fn from(ip: u128) -> Ipv6Addr {
        Ipv6Addr::from(ip.to_be_bytes())
    }
}

impl From<[u8; 16]> for Ipv6Addr {
    /// Creates an `Ipv6Addr` from a sixteen element byte array.
    #[inline]
    fn from(octets: [u8; 16]) -> Ipv6Addr {
        Ipv6Addr { inner: octets }
    }
}

impl From<[u16; 8]> for Ipv6Addr {
    /// Creates an `Ipv6Addr` from an eight element 16-bit array.
    #[inline]
    #[allow(clippy::many_single_char_names)]
    fn from(segments: [u16; 8]) -> Ipv6Addr {
        let [a, b, c, d, e, f, g, h] = segments;
        Ipv6Addr::new(a, b, c, d, e, f, g, h)
    }
}

impl From<[u8; 16]> for IpAddr {
    /// Creates an `IpAddr::V6` from a sixteen element byte array.
    #[inline]
    fn from(octets: [u8; 16]) -> IpAddr {
        IpAddr::V6(Ipv6Addr::from(octets))
    }
}

impl From<[u16; 8]> for IpAddr {
    /// Creates an `IpAddr::V6` from an eight element 16-bit array.
    #[inline]
    fn from(segments: [u16; 8]) -> IpAddr {
        IpAddr::V6(Ipv6Addr::from(segments))
    }
}
