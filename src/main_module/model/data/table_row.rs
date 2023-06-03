/*
 * MIT License
 *
 * Copyright (c) 2023 Andrew Kushyk
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use crate::main_module::model::data::data_object::DataObject;

pub struct TableRow {
	incorrect_field_name: String,
	fields: Vec<DataObject>,
}

impl TableRow {
	pub fn new(fields: Vec<DataObject>) -> Self {
		TableRow {
			incorrect_field_name: String::from("[ERROR] [Incorrect field name]"),
			fields,
		}
	}

	pub fn add_field(data_object: DataObject) {
		todo!()
	}

	pub fn insert_field(data_object: DataObject) {
		todo!()
	}

	pub fn remove_field(field: String) {
		todo!()
	}

	pub fn remove_field_value(field: String) {
		todo!()
	}

	pub fn get_field_data_obj(field: String) {
		todo!()
	}

	pub fn replace_existing_obj_value(current: DataObject, new: DataObject) {
		todo!()
	}

	fn find_data_obj(field: String) {
		todo!()
	}
}