//! DynamoDB helper methods that make Rusoto's automatically generated DynamoDBClient easier for humans to use
//!
//!

#![allow(unused_variables, unused_mut, non_snake_case)]
use std::str;

use rusoto::credentials::AWSCredentialsProvider;
use rusoto::regions::Region;
use rusoto::dynamodb::*;

// Make getting values out of nexted Option objects less verbose
macro_rules! try_opt {
    ($expr:expr) => (match $expr {
        ::std::option::Option::Some(ref val) => val,
        ::std::option::Option::None => return None
    })
}

pub struct DynamoDBHelper<'a> {
	client: DynamoDBClient<'a>
}

impl<'a> DynamoDBHelper<'a> {
	pub fn new<P: AWSCredentialsProvider + 'a>(credentials: P, region:&'a Region) -> DynamoDBHelper<'a> {
		DynamoDBHelper { client: DynamoDBClient::new(credentials, region) }
	}

	pub fn list_tables(&mut self) -> Result<ListTablesOutput> {
		let mut req = ListTablesInput::default();
		self.client.list_tables(&req)
	}

	pub fn create_table(&mut self, input: &CreateTableInput) -> Result<CreateTableOutput> {
		self.client.create_table(input)
	}

	pub fn describe_table(&mut self, name: &str) -> Result<DescribeTableOutput> {
		let mut input = DescribeTableInput::default();
		input.TableName = String::from(name);
		self.client.describe_table(&input)
	}

	pub fn delete_table(&mut self, name: &str) -> Result<DeleteTableOutput> {
		let mut input = DeleteTableInput::default();
		input.TableName = String::from(name);
		self.client.delete_table(&input)
	}

	pub fn put_item(&mut self, input: &PutItemInput) -> Result<PutItemOutput> {
		self.client.put_item(input)
	}

    pub fn get_item(&mut self, input: &GetItemInput) -> Result<GetItemOutput> {
		self.client.get_item(input)
	}

}

pub trait PutItemInputHelper {
	fn new() -> PutItemInput;
}

impl PutItemInputHelper for PutItemInput {
	 fn new() -> PutItemInput {
		PutItemInput::default()
	}
}

pub trait CreateTableInputHelper {
	fn new() -> CreateTableInput;
	fn with_name(mut self, table_name: &str) -> CreateTableInput;
	fn with_write_capacity(mut self, write_capacity: PositiveLongObject) -> CreateTableInput;
	fn with_read_capacity(mut self, read_capacity: PositiveLongObject) -> CreateTableInput;
	fn with_attributes(mut self, attributes: Vec<AttributeDefinition>) -> CreateTableInput;
	fn with_key_schema(mut self, key_schema: Vec<KeySchemaElement>) -> CreateTableInput;
	fn add_attribute<N: Into<String>, T: Into<String>>(mut self, name: N, attr_type: T) -> CreateTableInput;
}

impl CreateTableInputHelper for CreateTableInput {
	fn new() -> CreateTableInput {
		CreateTableInput::default()
	}

	fn with_name(mut self, table_name: &str) -> CreateTableInput {
		self.TableName = String::from(table_name);
		self
	}

	fn with_write_capacity(mut self, write_capacity: PositiveLongObject) -> CreateTableInput {
		self.ProvisionedThroughput.WriteCapacityUnits = write_capacity;
		self
	}

	fn with_read_capacity(mut self, read_capacity: PositiveLongObject) -> CreateTableInput {
		self.ProvisionedThroughput.ReadCapacityUnits = read_capacity;
		self
	}

	fn with_attributes(mut self, attributes: Vec<AttributeDefinition>) -> CreateTableInput {
		self.AttributeDefinitions = attributes;
		self
	}

	fn with_key_schema(mut self, key_schema: Vec<KeySchemaElement>) -> CreateTableInput {
		self.KeySchema = key_schema;
		self
	}

	fn add_attribute<N: Into<String>, T: Into<String>>(mut self, name: N, attr_type: T) -> CreateTableInput {
		self.AttributeDefinitions.push(AttributeDefinition { AttributeName: name.into(), AttributeType: attr_type.into() });
		self
	}

}

pub trait DescribeTableOutputHelper {
	fn get_status(&self) -> Option<String>;
}

impl DescribeTableOutputHelper for DescribeTableOutput {
	fn get_status(&self) -> Option<String> {
		let table = try_opt!(self.Table);
		Some(try_opt!(table.TableStatus).to_string())
	}
}




#[macro_export]
macro_rules! attributes {
    ($($val:expr => $attr_type:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(AttributeDefinition { AttributeName: String::from($val), AttributeType: String::from($attr_type) });
            )*
            temp_vec
        }
    }
}

#[macro_export]
macro_rules! key_schema {
    ($($name:expr => $key_type:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(KeySchemaElement { KeyType: String::from($key_type), AttributeName: String::from($name) });
            )*
            temp_vec
        }
    }
}

#[macro_export]
macro_rules! val {
	(B => $val:expr) => (
	    {
	    	let mut attr = AttributeValue::default();
	    	attr.B = Some($val);
	    	attr
	    }
	);
	(S => $val:expr) => (
	    {
			let mut attr = AttributeValue::default();
			attr.S = Some($val.to_string());
			attr
		}
	);
	(N => $val:expr) => (
	    {
	    	let mut attr = AttributeValue::default();
	    	attr.N = Some($val.to_string());
	    	attr
	    }
	);
}

// TODO: make a macro from this?
pub fn get_str_from_attribute(attr: &AttributeValue) -> Option<&str> {
    match attr.B {
        None => (),
        Some(ref blob_attribute) => return Some(str::from_utf8(blob_attribute).unwrap()),
    }

    match attr.S {
        None => (),
        Some(ref string_attribute) => return Some(string_attribute),
    }

    match attr.N {
        None => (),
        Some(ref number_attribute) => return Some(number_attribute),
    }

    return None;
}

