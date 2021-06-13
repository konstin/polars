use crate::prelude::Arc;
use arrow::array::{ArrayRef, UInt32Array, Utf8Array};
use arrow::buffer::Buffer;
use arrow::datatypes::DataType;

pub(crate) fn string_lengths(array: &Utf8Array<i64>) -> ArrayRef {
    let values = array
        .offsets()
        .iter()
        .windows(2)
        .map(|x| (x[1] - x[0]) as u32);

    let values = Buffer::from_trusted_len_iter(values);

    UInt32Array::from_data(DataType::UInt32, values, array.validity().clone());
    Arc::new(array)
}
