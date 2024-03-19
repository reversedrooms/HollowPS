use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    hash::Hash,
    io::{Read, Result, Write},
};

use byteorder::{ReadBytesExt, WriteBytesExt};

pub use qwer_derive::OctData;

use crate::DoubleKeyHashMap;

// LE encoded data
pub trait OctData: Sized + Send + Sync {
    fn marshal_to<W: Write>(&self, w: &mut W, bt_property_tag: u16) -> Result<()>;

    fn unmarshal_from<R: Read>(r: &mut R, bt_property_tag: u16) -> Result<Self>;
}

impl OctData for bool {
    fn marshal_to<W: Write>(&self, w: &mut W, _: u16) -> Result<()> {
        w.write_u8(u8::from(*self))
    }

    fn unmarshal_from<R: Read>(r: &mut R, _: u16) -> Result<Self> {
        Ok(r.read_u8()? != 0)
    }
}

impl OctData for u8 {
    fn marshal_to<W: Write>(&self, w: &mut W, _: u16) -> Result<()> {
        w.write_u8(*self)
    }

    fn unmarshal_from<R: Read>(r: &mut R, _: u16) -> Result<Self> {
        r.read_u8()
    }
}

impl OctData for i8 {
    fn marshal_to<W: Write>(&self, w: &mut W, _: u16) -> Result<()> {
        w.write_i8(*self)
    }

    fn unmarshal_from<R: Read>(r: &mut R, _: u16) -> Result<Self> {
        r.read_i8()
    }
}

macro_rules! impl_primitive {
    ($($t:ty, $write:ident, $read:ident,)*) => {
        $(
            impl OctData for $t {
                fn marshal_to<W: Write>(&self, w: &mut W, _: u16) -> Result<()> {
                    w.$write::<byteorder::LittleEndian>(*self)
                }

                fn unmarshal_from<R: Read>(r: &mut R, _: u16) -> Result<Self> {
                    r.$read::<byteorder::LittleEndian>()
                }
            }
        )*
    };
}

impl_primitive! {
    u16, write_u16, read_u16,
    i16, write_i16, read_i16,
    u32, write_u32, read_u32,
    i32, write_i32, read_i32,
    u64, write_u64, read_u64,
    i64, write_i64, read_i64,
}

// floats are a bit special, the bits are casted to an integer and then treated as an integer

impl OctData for f32 {
    fn marshal_to<W: Write>(&self, w: &mut W, _: u16) -> Result<()> {
        w.write_u32::<byteorder::LittleEndian>(self.to_bits())
    }

    fn unmarshal_from<R: Read>(r: &mut R, _: u16) -> Result<Self> {
        Ok(Self::from_bits(r.read_u32::<byteorder::LittleEndian>()?))
    }
}

impl OctData for f64 {
    fn marshal_to<W: Write>(&self, w: &mut W, _: u16) -> Result<()> {
        w.write_u64::<byteorder::LittleEndian>(self.to_bits())
    }

    fn unmarshal_from<R: Read>(r: &mut R, _: u16) -> Result<Self> {
        Ok(Self::from_bits(r.read_u64::<byteorder::LittleEndian>()?))
    }
}

impl<T> OctData for Vec<T>
where
    T: OctData,
{
    fn marshal_to<W: Write>(&self, w: &mut W, bt_property_tag: u16) -> Result<()> {
        if self.is_empty() {
            (0i32).marshal_to(w, bt_property_tag)?;
            return Ok(());
        }
        (self.len() as i32).marshal_to(w, bt_property_tag)?;
        for item in self {
            item.marshal_to(w, bt_property_tag)?;
        }
        Ok(())
    }

    fn unmarshal_from<R: Read>(r: &mut R, bt_property_tag: u16) -> Result<Self> {
        let len = i32::unmarshal_from(r, bt_property_tag)?;
        if len < 0 {
            let real_len = -len;
            let mut vec = Self::with_capacity(real_len as usize);
            for _ in 0..real_len {
                bool::unmarshal_from(r, bt_property_tag)?;
                vec.push(T::unmarshal_from(r, bt_property_tag)?);
            }
            Ok(vec)
        } else {
            let mut vec = Self::with_capacity(len as usize);
            for _ in 0..len {
                vec.push(T::unmarshal_from(r, bt_property_tag)?);
            }
            Ok(vec)
        }
    }
}

impl<K, V> OctData for HashMap<K, V>
where
    K: OctData + Eq + Hash,
    V: OctData,
{
    default fn marshal_to<W: Write>(&self, w: &mut W, bt_property_tag: u16) -> Result<()> {
        (self.len() as i32).marshal_to(w, bt_property_tag)?;
        for (key, value) in self {
            key.marshal_to(w, bt_property_tag)?;
            value.marshal_to(w, bt_property_tag)?;
        }
        Ok(())
    }

    default fn unmarshal_from<R: Read>(r: &mut R, bt_property_tag: u16) -> Result<Self> {
        let len = i32::unmarshal_from(r, bt_property_tag)?;
        if len == -1 {
            return Ok(Self::new());
        }
        let mut map = Self::with_capacity(len as usize);
        for _ in 0..len {
            map.insert(
                K::unmarshal_from(r, bt_property_tag)?,
                V::unmarshal_from(r, bt_property_tag)?,
            );
        }
        Ok(map)
    }
}

#[cfg(feature = "collection")]
impl<K1, K2, V> OctData for DoubleKeyHashMap<K1, K2, V>
where
    K1: OctData + Eq + Hash,
    K2: OctData + Eq + Hash,
    V: OctData,
{
    fn marshal_to<W: Write>(&self, w: &mut W, bt_property_tag: u16) -> Result<()> {
        self.iter()
            .map(|(_, inner_map)| inner_map.len() as i32)
            .sum::<i32>()
            .marshal_to(w, bt_property_tag)?;
        for (key1, inner_map) in self {
            for (key2, value) in inner_map {
                key1.marshal_to(w, bt_property_tag)?;
                key2.marshal_to(w, bt_property_tag)?;
                value.marshal_to(w, bt_property_tag)?;
            }
        }
        Ok(())
    }

    fn unmarshal_from<R: Read>(r: &mut R, bt_property_tag: u16) -> Result<Self> {
        let len = i32::unmarshal_from(r, bt_property_tag)?;
        if len == -1 {
            return Ok(Self::new());
        }
        let mut map = Self::new();
        for _ in 0..len {
            let key1 = K1::unmarshal_from(r, bt_property_tag)?;
            let key2 = K2::unmarshal_from(r, bt_property_tag)?;
            let value = V::unmarshal_from(r, bt_property_tag)?;
            map.entry(key1)
                .or_insert_with(HashMap::new)
                .insert(key2, value);
        }
        Ok(map)
    }
}

impl<T> OctData for HashSet<T>
where
    T: OctData + Eq + Hash,
{
    fn marshal_to<W: Write>(&self, w: &mut W, bt_property_tag: u16) -> Result<()> {
        (self.len() as i32).marshal_to(w, bt_property_tag)?;
        for item in self {
            item.marshal_to(w, bt_property_tag)?;
        }
        Ok(())
    }

    fn unmarshal_from<R: Read>(r: &mut R, bt_property_tag: u16) -> Result<Self> {
        let len = i32::unmarshal_from(r, bt_property_tag)?;
        if len == -1 {
            return Ok(Self::new());
        }
        let mut set = Self::with_capacity(len as usize);
        for _ in 0..len {
            set.insert(T::unmarshal_from(r, bt_property_tag)?);
        }
        Ok(set)
    }
}

impl<T> OctData for Option<T>
where
    T: OctData,
{
    fn marshal_to<W: Write>(&self, w: &mut W, bt_property_tag: u16) -> Result<()> {
        if let Some(item) = self {
            item.marshal_to(w, bt_property_tag)?;
        }

        Ok(())
    }

    fn unmarshal_from<R: Read>(r: &mut R, bt_property_tag: u16) -> Result<Self> {
        Ok(Some(T::unmarshal_from(r, bt_property_tag)?))
    }
}

impl OctData for String {
    fn marshal_to<W: Write>(&self, w: &mut W, bt_property_tag: u16) -> Result<()> {
        if self.is_empty() {
            (-1i32).marshal_to(w, bt_property_tag)?;
            return Ok(());
        }
        (self.len() as i32).marshal_to(w, bt_property_tag)?;
        w.write_all(self.as_bytes())
    }

    fn unmarshal_from<R: Read>(r: &mut R, bt_property_tag: u16) -> Result<Self> {
        let len = i32::unmarshal_from(r, bt_property_tag)?;
        if len == -1 {
            return Ok(Self::new());
        }
        let mut buf = vec![0; len as usize];
        r.read_exact(&mut buf)?;
        Ok(Self::from_utf8(buf).unwrap())
    }
}

impl<'a, T> OctData for Cow<'a, T>
where
    T: OctData + Clone,
{
    fn marshal_to<W: Write>(&self, w: &mut W, bt_property_tag: u16) -> Result<()> {
        self.as_ref().marshal_to(w, bt_property_tag)
    }

    fn unmarshal_from<R: Read>(r: &mut R, bt_property_tag: u16) -> Result<Self> {
        Ok(Cow::Owned(T::unmarshal_from(r, bt_property_tag)?))
    }
}
