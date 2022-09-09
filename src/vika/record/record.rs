use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

#[derive(Deserialize, Serialize, Debug)]
pub struct Record {
    #[serde(rename = "recordId")]
    pub record_id: String,

    pub fields: Value,

    #[serde(rename = "createdAt")]
    pub created_at: u64,

    #[serde(rename = "updatedAt")]
    pub updated_at: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetRecordsResp {
    #[serde(rename = "pageNum")]
    pub page_num: u32,

    pub records: Vec<Record>,

    #[serde(rename = "pageSize")]
    pub page_size: u32,

    pub total: u32,
}

#[derive(Deserialize, Serialize)]
pub struct GetRecordsReq {
    field: Map<String, Value>,
}

impl GetRecordsReq {
    pub fn builder() -> GetRecordsReqBuilder {
        GetRecordsReqBuilder { field: Map::new() }
    }

    pub fn to_parameter_vec(self) -> Vec<(String, String)> {
        let mut result: Vec<(String, String)> = Vec::new();
        for (key, value) in self.field {
            if key.eq("recordIds") {
                let record_ids = value.as_array().unwrap();
                for record_id in record_ids {
                    result.push((key.clone(), record_id.as_str().unwrap().to_string()))
                }
            } else if key.eq("fields") {
                let fields = value.as_array().unwrap();
                for field in fields {
                    result.push((
                        String::from("fields[]"),
                        field.as_str().unwrap().to_string(),
                    ))
                }
            } else if key.eq("sort") {
                let sorts = value.as_array().unwrap();
                for sort in sorts {
                    result.push((
                        format!("sort[][field]",),
                        sort[0].as_str().unwrap().to_string(),
                    ));
                    result.push((
                        format!("sort[][order]",),
                        sort[1].as_str().unwrap().to_string(),
                    ));
                }
            } else if key.eq("pageSize") || key.eq("maxRecords") || key.eq("pageNum") {
                result.push((key, value.as_u64().unwrap().to_string()));
            } else {
                result.push((key, String::from(value.as_str().unwrap())));
            }
        }
        result
    }
}

pub struct GetRecordsReqBuilder {
    field: Map<String, Value>,
}

impl GetRecordsReqBuilder {
    pub fn page_size(mut self, page_size: u64) -> GetRecordsReqBuilder {
        self.field
            .insert(String::from("pageSize"), json!(page_size));
        self
    }

    pub fn max_records(mut self, max_records: u64) -> GetRecordsReqBuilder {
        self.field
            .insert(String::from("maxRecords"), json!(max_records));
        self
    }

    pub fn page_num(mut self, page_num: u64) -> GetRecordsReqBuilder {
        self.field.insert(String::from("pageNum"), json!(page_num));
        self
    }

    pub fn sort(mut self, sort: Vec<Sort>) -> GetRecordsReqBuilder {
        self.field.insert(String::from("sort"), json!(sort));
        self
    }

    pub fn record_ids(mut self, record_ids: Vec<String>) -> GetRecordsReqBuilder {
        self.field
            .insert(String::from("recordIds"), json!(record_ids));
        self
    }

    pub fn view_id(mut self, view_id: String) -> GetRecordsReqBuilder {
        self.field.insert(String::from("viewId"), json!(view_id));
        self
    }

    pub fn fields(mut self, fields: Vec<String>) -> GetRecordsReqBuilder {
        self.field.insert(String::from("fields"), json!(fields));
        self
    }

    pub fn filter_by_formula(mut self, filter_by_formula: String) -> GetRecordsReqBuilder {
        self.field
            .insert(String::from("filterByFormula"), json!(filter_by_formula));
        self
    }

    pub fn cell_format(mut self, cell_format: String) -> GetRecordsReqBuilder {
        self.field
            .insert(String::from("cellFormat"), json!(cell_format));
        self
    }

    pub fn field_key(mut self, field_key: String) -> GetRecordsReqBuilder {
        self.field
            .insert(String::from("fieldKey"), json!(field_key));
        self
    }

    pub fn build(self) -> GetRecordsReq {
        GetRecordsReq { field: self.field }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Sort(pub String, pub String);

pub type PatchRecordsReq = PostRecordsReq;

#[derive(Deserialize, Serialize, Debug)]
pub struct PostRecordsReq {
    #[serde(rename = "fieldKey")]
    pub field_key: String,

    pub records: Vec<RecordMap>,
}

impl PostRecordsReq {
    pub fn to_json_string(self) -> String {
        let mut value: Map<String, Value> = Map::new();
        value.insert("fieldKey".to_string(), Value::String(self.field_key));
        let mut record_values: Vec<Value> = Vec::new();
        for record in self.records {
            let record_value_string = serde_json::to_string(&record).unwrap();
            let record_value: Value = serde_json::from_str(&record_value_string).unwrap();
            record_values.push(record_value);
        }
        value.insert("records".to_string(), Value::Array(record_values));
        serde_json::to_string(&value).unwrap()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RecordMap {
    #[serde(rename = "recordId")]
    record_id: String,

    fields: Map<String, Value>,
}

impl RecordMap {
    pub fn builder() -> RecordMapBuilder {
        RecordMapBuilder {
            record_id: "".to_string(),
            fields: Map::new(),
        }
    }
}

pub struct RecordMapBuilder {
    pub record_id: String,

    pub fields: Map<String, Value>,
}

impl RecordMapBuilder {
    pub fn record_id(mut self, record_id: String) -> RecordMapBuilder {
        self.record_id = record_id;
        self
    }

    pub fn put_string(mut self, field_key: String, field_value: String) -> RecordMapBuilder {
        self.fields.insert(field_key, Value::String(field_value));
        self
    }

    pub fn put_strings(mut self, field_key: String, field_values: Vec<String>) -> RecordMapBuilder {
        let mut values: Vec<Value> = Vec::new();
        for field_value in field_values {
            values.push(Value::String(field_value));
        }
        self.fields.insert(field_key, Value::Array(values));
        self
    }

    pub fn build(self) -> RecordMap {
        RecordMap {
            record_id: self.record_id,
            fields: self.fields,
        }
    }
}

pub type PatchRecordsResp = PostRecordsResp;

#[derive(Deserialize, Serialize, Debug)]
pub struct PostRecordsResp {
    pub records: Vec<Record>,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_records_req() {
        let get_records_req = GetRecordsReq::builder()
            .page_size(1)
            .max_records(1)
            .page_num(1)
            .sort(vec![
                Sort("1".to_string(), "asc".to_string()),
                Sort("2".to_string(), "desc".to_string()),
            ])
            .record_ids(vec!["1".to_string(), "2".to_string()])
            .view_id("1".to_string())
            .fields(vec!["1".to_string(), "2".to_string()])
            .filter_by_formula("1".to_string())
            .cell_format("1".to_string())
            .field_key("1".to_string())
            .build();
        println!("{:?}", get_records_req.to_parameter_vec())
    }

    #[test]
    fn post_records_req() {
        let record1 = RecordMap::builder()
            .put_string("field_key1".to_string(), "field_value1".to_string())
            .put_string("field_key2".to_string(), "field_value2".to_string())
            .build();
        let record2 = RecordMap::builder()
            .put_string("field_key1".to_string(), "field_value1".to_string())
            .put_string("field_key2".to_string(), "field_value2".to_string())
            .build();
        let req = PostRecordsReq {
            field_key: "name".to_string(),
            records: vec![record1, record2],
        };

        assert_eq!(
            r#"{"fieldKey":"name","records":[{"fields":{"field_key1":"field_value1","field_key2":"field_value2"},"recordId":""},{"fields":{"field_key1":"field_value1","field_key2":"field_value2"},"recordId":""}]}"#,
            req.to_json_string()
        )
    }

    #[test]
    fn patch_records_req() {
        let record1 = RecordMap::builder()
            .record_id("rec1".to_string())
            .put_string("field_key1".to_string(), "field_value1".to_string())
            .put_string("field_key2".to_string(), "field_value2".to_string())
            .build();
        let record2 = RecordMap::builder()
            .record_id("rec1".to_string())
            .put_string("field_key1".to_string(), "field_value1".to_string())
            .put_string("field_key2".to_string(), "field_value2".to_string())
            .build();
        let req = PostRecordsReq {
            field_key: "name".to_string(),
            records: vec![record1, record2],
        };
        assert_eq!(
            r#"{"fieldKey":"name","records":[{"fields":{"field_key1":"field_value1","field_key2":"field_value2"},"recordId":"rec1"},{"fields":{"field_key1":"field_value1","field_key2":"field_value2"},"recordId":"rec1"}]}"#,
            req.to_json_string()
        )
    }
}
