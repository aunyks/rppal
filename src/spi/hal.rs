// Copyright (c) 2017-2021 Rene van der Meer
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

#![allow(clippy::needless_lifetimes)]

use embedded_hal::spi::blocking::{Transfer, Write};
use embedded_hal::spi::nb::FullDuplex;

use super::{Error, Spi};

/// `Transfer<u8>` trait implementation for `embedded-hal` v1.0.0-alpha.5.
impl Transfer<u8> for Spi {
    type Error = Error;

    fn transfer<'a>(&mut self, buffer: &'a mut [u8]) -> Result<(), Self::Error> {
        let write_buffer = buffer.to_vec();
        Spi::transfer(self, buffer, &write_buffer)?;

        Ok(())
    }
}

/// `Transfer<u8>` trait implementation for `embedded-hal` v0.2.6.
impl embedded_hal_0::blocking::spi::Transfer<u8> for Spi {
    type Error = Error;

    fn transfer<'a>(&mut self, buffer: &'a mut [u8]) -> Result<&'a [u8], Self::Error> {
        Transfer::transfer(self, buffer)?;
        Ok(buffer)
    }
}

/// `Write<u8>` trait implementation for `embedded-hal` v1.0.0-alpha.5.
impl Write<u8> for Spi {
    type Error = Error;

    fn write(&mut self, buffer: &[u8]) -> Result<(), Self::Error> {
        Spi::write(self, buffer)?;

        Ok(())
    }
}

/// `Write<u8>` trait implementation for `embedded-hal` v0.2.6.
impl embedded_hal_0::blocking::spi::Write<u8> for Spi {
    type Error = Error;

    fn write(&mut self, buffer: &[u8]) -> Result<(), Self::Error> {
        Write::write(self, buffer)
    }
}

/// `FullDuplex<u8>` trait implementation for `embedded-hal` v1.0.0-alpha.5.
impl FullDuplex<u8> for Spi {
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        if let Some(last_read) = self.last_read.take() {
            Ok(last_read)
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        let mut read_buffer: [u8; 1] = [0];

        Spi::transfer(self, &mut read_buffer, &[byte])?;
        self.last_read = Some(read_buffer[0]);

        Ok(())
    }
}

/// `FullDuplex<u8>` trait implementation for `embedded-hal` v0.2.6.
impl embedded_hal_0::spi::FullDuplex<u8> for Spi {
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        FullDuplex::read(self)
    }

    fn send(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        FullDuplex::write(self, byte)
    }
}
