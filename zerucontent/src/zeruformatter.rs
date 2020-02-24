use serde::Serialize;
use serde_json::error::{Result};
use serde_json::ser::{Formatter, Serializer};
use std::io;

#[derive(Clone, Debug)]
pub struct ZeruFormatter;

pub fn to_writer_pretty<W, T: ?Sized>(writer: W, value: &T) -> Result<()>
where
	W: io::Write,
	T: Serialize,
{
	let mut ser = Serializer::with_formatter(writer, ZeruFormatter);
	value.serialize(&mut ser)
}

pub fn to_vec_pretty<T: ?Sized>(value: &T) -> Result<Vec<u8>>
where
	T: Serialize,
{
	let mut writer = Vec::with_capacity(128);
	to_writer_pretty(&mut writer, value)?;
	Ok(writer)
}

pub fn to_string_zero<T: ?Sized>(value: &T) -> Result<String>
where
	T: Serialize,
{
	let vec = to_vec_pretty(value)?;
	let string = unsafe {
		// We do not emit invalid UTF-8.
		String::from_utf8_unchecked(vec)
	};
	Ok(string)
}

impl Formatter for ZeruFormatter {
	#[inline]
	fn begin_array<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
	where
		W: io::Write,
	{
		writer.write_all(b"[")
	}

	#[inline]
	fn end_array<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
	where
		W: io::Write,
	{
		writer.write_all(b"]")
	}

	#[inline]
	fn begin_array_value<W: ?Sized>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
	where
		W: io::Write,
	{
		if !first {
			return writer.write_all(b", ");
		}
		Ok(())
	}

	#[inline]
	fn end_array_value<W: ?Sized>(&mut self, _writer: &mut W) -> io::Result<()>
	where
		W: io::Write,
	{
		Ok(())
	}

	#[inline]
	fn begin_object<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
	where
		W: io::Write,
	{
		writer.write_all(b"{")
	}

	#[inline]
	fn end_object<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
	where
		W: io::Write,
	{
		writer.write_all(b"}")
	}

	#[inline]
	fn begin_object_key<W: ?Sized>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
	where
		W: io::Write,
	{
		if first {
			writer.write_all(b"")
		} else {
			writer.write_all(b", ")
		}
	}

	#[inline]
	fn begin_object_value<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
	where
		W: io::Write,
	{
		writer.write_all(b": ")
	}

	#[inline]
	fn end_object_value<W: ?Sized>(&mut self, _writer: &mut W) -> io::Result<()>
	where
		W: io::Write,
	{
		Ok(())
	}
}
