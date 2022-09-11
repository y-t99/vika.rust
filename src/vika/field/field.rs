use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
pub struct PostFieldReq {
    field: Map<String, Value>,
}

impl PostFieldReq {
    pub fn builder() -> PostFieldReqBuilder {
        PostFieldReqBuilder { field: Map::new() }
    }

    pub fn to_json_string(self) -> String {
        serde_json::to_string(&self.field).unwrap()
    }
}

pub struct PostFieldReqBuilder {
    field: Map<String, Value>,
}

impl PostFieldReqBuilder {
    pub fn name(mut self, name: String) -> PostFieldReqBuilder {
        self.field.insert(String::from("name"), json!(name));
        self
    }

    pub fn field_type(mut self, field_type: String) -> PostFieldReqBuilder {
        self.field.insert(String::from("type"), json!(field_type));
        self
    }

    pub fn build(self) -> PostFieldReq {
        PostFieldReq { field: self.field }
    }

    pub fn as_single_text_req(mut self) -> PostSingleTextFieldReq {
        self.field.insert(String::from("type"), json!("SingleText"));
        PostSingleTextFieldReq {
            field: self.field,
            property: Map::new(),
        }
    }

    pub fn as_text_req(mut self) -> PostTextFieldReq {
        self.field.insert(String::from("type"), json!("Text"));
        PostTextFieldReq { field: self.field }
    }

    pub fn as_single_select_req(mut self) -> PostSingleSelectFieldReq {
        self.field
            .insert(String::from("type"), json!("SingleSelect"));
        PostSingleSelectFieldReq {
            field: self.field,
            property: Map::new(),
        }
    }

    pub fn as_multi_select_req(mut self) -> PostMultiSelectFieldReq {
        self.field
            .insert(String::from("type"), json!("MultiSelect"));
        PostMultiSelectFieldReq {
            field: self.field,
            property: Map::new(),
        }
    }

    pub fn as_number_req(mut self) -> PostNumberFieldReq {
        self.field.insert(String::from("type"), json!("Number"));
        PostNumberFieldReq {
            field: self.field,
            property: Map::new(),
        }
    }

    pub fn as_currency_req(mut self) -> PostCurrencyFieldReq {
        self.field.insert(String::from("type"), json!("Currency"));
        PostCurrencyFieldReq {
            field: self.field,
            property: Map::new(),
        }
    }

    pub fn as_percent_req(mut self) -> PostPercentFieldReq {
        self.field.insert(String::from("type"), json!("Percent"));
        PostPercentFieldReq {
            field: self.field,
            property: Map::new(),
        }
    }

    pub fn as_date_time_req(mut self) -> PostDateTimeFieldReq {
        self.field.insert(String::from("type"), json!("DateTime"));
        PostDateTimeFieldReq {
            field: self.field,
            property: Map::new(),
        }
    }

    pub fn as_attachment_req(mut self) -> PostAttachmentFieldReq {
        self.field.insert(String::from("type"), json!("Attachment"));
        PostAttachmentFieldReq {
            field: self.field,
            property: Map::new(),
        }
    }

    pub fn as_member_req(mut self) -> PostMemberFieldReq {
        self.field.insert(String::from("type"), json!("Member"));
        PostMemberFieldReq {
            field: self.field,
            property: Map::new(),
        }
    }

    pub fn as_checkbox_req(mut self) -> PostCheckboxFieldReq {
        self.field.insert(String::from("type"), json!("Checkbox"));
        PostCheckboxFieldReq {
            field: self.field,
            property: Map::new(),
        }
    }

    pub fn as_rating_req(mut self) -> PostRatingFieldReq {
        self.field.insert(String::from("type"), json!("Rating"));
        PostRatingFieldReq {
            field: self.field,
            property: Map::new(),
        }
    }

    pub fn as_url_req(mut self) -> PostURLFieldReq {
        self.field.insert(String::from("type"), json!("URL"));
        PostURLFieldReq {
            field: self.field,
            property: Map::new(),
        }
    }

    pub fn as_phone_req(mut self) -> PostPhoneFieldReq {
        self.field.insert(String::from("type"), json!("Phone"));
        PostPhoneFieldReq {
            field: self.field,
            property: Map::new(),
        }
    }

    pub fn as_email_req(mut self) -> PostEmailFieldReq {
        self.field.insert(String::from("type"), json!("Email"));
        PostEmailFieldReq {
            field: self.field,
            property: Map::new(),
        }
    }

    pub fn as_magic_look_up_req(mut self) -> PostMagicLookUpFieldReq {
        self.field
            .insert(String::from("type"), json!("MagicLookUp"));
        PostMagicLookUpFieldReq {
            field: self.field,
            property: Map::new(),
        }
    }

    pub fn as_magic_link_req(mut self) -> PostMagicLinkFieldReq {
        self.field.insert(String::from("type"), json!("MagicLink"));
        PostMagicLinkFieldReq {
            field: self.field,
            property: Map::new(),
        }
    }

    pub fn as_formula_req(mut self) -> PostFormulaFieldReq {
        self.field.insert(String::from("type"), json!("Formula"));
        PostFormulaFieldReq {
            field: self.field,
            property: Map::new(),
        }
    }
    pub fn as_auto_number_req(mut self) -> PostAutoNumberFieldReq {
        self.field.insert(String::from("type"), json!("AutoNumber"));
        PostAutoNumberFieldReq {
            field: self.field,
            property: Map::new(),
        }
    }

    pub fn as_created_time_req(mut self) -> PostCreatedTimeFieldReq {
        self.field
            .insert(String::from("type"), json!("CreatedTime"));
        PostCreatedTimeFieldReq {
            field: self.field,
            property: Map::new(),
        }
    }

    pub fn as_last_modified_time_req(mut self) -> PostLastModifiedTimeFieldReq {
        self.field
            .insert(String::from("type"), json!("LastModifiedTime"));
        PostLastModifiedTimeFieldReq {
            field: self.field,
            property: Map::new(),
        }
    }

    pub fn as_created_by_req(mut self) -> PostCreatedByFieldReq {
        self.field.insert(String::from("type"), json!("CreatedBy"));
        PostCreatedByFieldReq {
            field: self.field,
            property: Map::new(),
        }
    }

    pub fn as_last_modified_by_req(mut self) -> PostLastModifiedByFieldReq {
        self.field
            .insert(String::from("type"), json!("LastModified"));
        PostLastModifiedByFieldReq {
            field: self.field,
            property: Map::new(),
        }
    }
}

pub struct PostSingleTextFieldReq {
    field: Map<String, Value>,
    property: Map<String, Value>,
}

pub struct PostTextFieldReq {
    field: Map<String, Value>,
}

impl PostSingleTextFieldReq {
    pub fn default_value(mut self, default_value: String) -> PostSingleTextFieldReq {
        self.property
            .insert(String::from("defaultValue"), json!(default_value));
        self
    }

    pub fn build(mut self) -> PostFieldReq {
        self.field
            .insert(String::from("property"), json!(self.property));
        PostFieldReq { field: self.field }
    }
}

impl PostTextFieldReq {
    pub fn build(self) -> PostFieldReq {
        PostFieldReq { field: self.field }
    }
}

pub struct PostSingleSelectFieldReq {
    field: Map<String, Value>,
    property: Map<String, Value>,
}

impl PostSingleSelectFieldReq {
    pub fn build(mut self) -> PostFieldReq {
        self.field
            .insert(String::from("property"), json!(self.property));
        PostFieldReq { field: self.field }
    }

    pub fn default_value(mut self, default_value: String) -> PostSingleSelectFieldReq {
        self.property
            .insert(String::from("defaultValue"), json!(default_value));
        self
    }

    pub fn options(mut self, options: Vec<SelectPropertyOption>) -> PostSingleSelectFieldReq {
        let mut options_list: Vec<Value> = Vec::new();
        for option in options {
            let value = option.to_json_string();
            let struct_value = serde_json::from_str(&value).unwrap();
            options_list.push(struct_value);
        }
        self.property
            .insert(String::from("options"), json!(options_list));
        self
    }
}

#[derive(Deserialize, Serialize)]
pub struct SelectPropertyOption {
    pub name: String,

    #[serde(default)]
    pub color: String,
}

impl SelectPropertyOption {
    pub fn to_json_string(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

pub struct PostMultiSelectFieldReq {
    field: Map<String, Value>,
    property: Map<String, Value>,
}

impl PostMultiSelectFieldReq {
    pub fn build(mut self) -> PostFieldReq {
        self.field
            .insert(String::from("property"), json!(self.property));
        PostFieldReq { field: self.field }
    }

    pub fn default_value(mut self, default_value: String) -> PostMultiSelectFieldReq {
        self.property
            .insert(String::from("defaultValue"), json!(default_value));
        self
    }

    pub fn options(mut self, options: Vec<SelectPropertyOption>) -> PostMultiSelectFieldReq {
        let mut options_list: Vec<Value> = Vec::new();
        for option in options {
            let value = option.to_json_string();
            let struct_value = serde_json::from_str(&value).unwrap();
            options_list.push(struct_value);
        }
        self.property
            .insert(String::from("options"), json!(options_list));
        self
    }
}

pub struct PostNumberFieldReq {
    field: Map<String, Value>,
    property: Map<String, Value>,
}

impl PostNumberFieldReq {
    pub fn build(mut self) -> PostFieldReq {
        self.field
            .insert(String::from("property"), json!(self.property));
        PostFieldReq { field: self.field }
    }

    pub fn default_value(mut self, default_value: String) -> PostNumberFieldReq {
        self.property
            .insert(String::from("defaultValue"), json!(default_value));
        self
    }

    pub fn precision(mut self, precision: i8) -> PostNumberFieldReq {
        self.property
            .insert(String::from("precision"), json!(precision));
        self
    }

    pub fn symbol(mut self, symbol: String) -> PostNumberFieldReq {
        self.property.insert(String::from("symbol"), json!(symbol));
        self
    }
}

pub struct PostCurrencyFieldReq {
    field: Map<String, Value>,
    property: Map<String, Value>,
}

impl PostCurrencyFieldReq {
    pub fn build(mut self) -> PostFieldReq {
        self.field
            .insert(String::from("property"), json!(self.property));
        PostFieldReq { field: self.field }
    }

    pub fn default_value(mut self, default_value: String) -> PostCurrencyFieldReq {
        self.property
            .insert(String::from("defaultValue"), json!(default_value));
        self
    }

    pub fn precision(mut self, precision: i8) -> PostCurrencyFieldReq {
        self.property
            .insert(String::from("precision"), json!(precision));
        self
    }

    pub fn symbol(mut self, symbol: String) -> PostCurrencyFieldReq {
        self.property.insert(String::from("symbol"), json!(symbol));
        self
    }

    pub fn symbol_align(mut self, symbol_align: String) -> PostCurrencyFieldReq {
        self.property
            .insert(String::from("symbolAlign"), json!(symbol_align));
        self
    }
}

pub struct PostPercentFieldReq {
    field: Map<String, Value>,
    property: Map<String, Value>,
}

impl PostPercentFieldReq {
    pub fn build(mut self) -> PostFieldReq {
        self.field
            .insert(String::from("property"), json!(self.property));
        PostFieldReq { field: self.field }
    }

    pub fn default_value(mut self, default_value: String) -> PostPercentFieldReq {
        self.property
            .insert(String::from("defaultValue"), json!(default_value));
        self
    }

    pub fn precision(mut self, precision: i8) -> PostPercentFieldReq {
        self.property
            .insert(String::from("precision"), json!(precision));
        self
    }
}

pub struct PostDateTimeFieldReq {
    field: Map<String, Value>,
    property: Map<String, Value>,
}

impl PostDateTimeFieldReq {
    pub fn build(mut self) -> PostFieldReq {
        self.field
            .insert(String::from("property"), json!(self.property));
        PostFieldReq { field: self.field }
    }

    pub fn date_format(mut self, date_format: String) -> PostDateTimeFieldReq {
        self.property
            .insert(String::from("dateFormat"), json!(date_format));
        self
    }

    pub fn time_format(mut self, time_format: String) -> PostDateTimeFieldReq {
        self.property
            .insert(String::from("timeFormat"), json!(time_format));
        self
    }

    pub fn auto_fill(mut self, auto_fill: bool) -> PostDateTimeFieldReq {
        self.property
            .insert(String::from("autoFill"), json!(auto_fill));
        self
    }

    pub fn include_time(mut self, include_time: bool) -> PostDateTimeFieldReq {
        self.property
            .insert(String::from("includeTime"), json!(include_time));
        self
    }
}

pub struct PostAttachmentFieldReq {
    field: Map<String, Value>,
    property: Map<String, Value>,
}

impl PostAttachmentFieldReq {
    pub fn build(mut self) -> PostFieldReq {
        self.field
            .insert(String::from("property"), json!(self.property));
        PostFieldReq { field: self.field }
    }
}

pub struct PostMemberFieldReq {
    field: Map<String, Value>,
    property: Map<String, Value>,
}

impl PostMemberFieldReq {
    pub fn build(mut self) -> PostFieldReq {
        self.field
            .insert(String::from("property"), json!(self.property));
        PostFieldReq { field: self.field }
    }

    pub fn is_multi(mut self, is_multi: bool) -> PostMemberFieldReq {
        self.property
            .insert(String::from("isMulti"), json!(is_multi));
        self
    }

    pub fn should_send_msg(mut self, should_send_msg: bool) -> PostMemberFieldReq {
        self.property
            .insert(String::from("shouldSendMsg"), json!(should_send_msg));
        self
    }
}

pub struct PostCheckboxFieldReq {
    field: Map<String, Value>,
    property: Map<String, Value>,
}

impl PostCheckboxFieldReq {
    pub fn build(mut self) -> PostFieldReq {
        self.field
            .insert(String::from("property"), json!(self.property));
        PostFieldReq { field: self.field }
    }

    pub fn icon(mut self, icon: String) -> PostCheckboxFieldReq {
        self.property.insert(String::from("icon"), json!(icon));
        self
    }
}

pub struct PostRatingFieldReq {
    field: Map<String, Value>,
    property: Map<String, Value>,
}

impl PostRatingFieldReq {
    pub fn build(mut self) -> PostFieldReq {
        self.field
            .insert(String::from("property"), json!(self.property));
        PostFieldReq { field: self.field }
    }

    pub fn icon(mut self, icon: String) -> PostRatingFieldReq {
        self.property.insert(String::from("icon"), json!(icon));
        self
    }

    pub fn max(mut self, max: String) -> PostRatingFieldReq {
        self.property.insert(String::from("max"), json!(max));
        self
    }
}

pub struct PostURLFieldReq {
    field: Map<String, Value>,
    property: Map<String, Value>,
}

impl PostURLFieldReq {
    pub fn build(mut self) -> PostFieldReq {
        self.field
            .insert(String::from("property"), json!(self.property));
        PostFieldReq { field: self.field }
    }
}

pub struct PostPhoneFieldReq {
    field: Map<String, Value>,
    property: Map<String, Value>,
}

impl PostPhoneFieldReq {
    pub fn build(mut self) -> PostFieldReq {
        self.field
            .insert(String::from("property"), json!(self.property));
        PostFieldReq { field: self.field }
    }
}

pub struct PostEmailFieldReq {
    field: Map<String, Value>,
    property: Map<String, Value>,
}

impl PostEmailFieldReq {
    pub fn build(mut self) -> PostFieldReq {
        self.field
            .insert(String::from("property"), json!(self.property));
        PostFieldReq { field: self.field }
    }
}

pub struct PostMagicLinkFieldReq {
    field: Map<String, Value>,
    property: Map<String, Value>,
}

impl PostMagicLinkFieldReq {
    pub fn build(mut self) -> PostFieldReq {
        self.field
            .insert(String::from("property"), json!(self.property));
        PostFieldReq { field: self.field }
    }

    pub fn foreign_datasheet_id(mut self, foreign_datasheet_id: String) -> PostMagicLinkFieldReq {
        self.property.insert(
            String::from("foreignDatasheetId"),
            json!(foreign_datasheet_id),
        );
        self
    }

    pub fn limit_single_record(mut self, limit_single_record: bool) -> PostMagicLinkFieldReq {
        self.property.insert(
            String::from("limitSingleRecord"),
            json!(limit_single_record),
        );
        self
    }
}

pub struct PostMagicLookUpFieldReq {
    field: Map<String, Value>,
    property: Map<String, Value>,
}

#[derive(Deserialize, Serialize)]
pub struct Format {
    #[serde(default, rename = "type")]
    pub format_type: String,

    #[serde(default)]
    pub format: String,
}

impl Format {
    pub fn to_json_string(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl PostMagicLookUpFieldReq {
    pub fn build(mut self) -> PostFieldReq {
        self.field
            .insert(String::from("property"), json!(self.property));
        PostFieldReq { field: self.field }
    }

    pub fn related_link_field_id(
        mut self,
        related_link_field_id: String,
    ) -> PostMagicLookUpFieldReq {
        self.property.insert(
            String::from("relatedLinkFieldId"),
            json!(related_link_field_id),
        );
        self
    }

    pub fn target_field_id(mut self, target_field_id: String) -> PostMagicLookUpFieldReq {
        self.property
            .insert(String::from("targetFieldId"), json!(target_field_id));
        self
    }

    pub fn rollup_function(mut self, rollup_function: String) -> PostMagicLookUpFieldReq {
        self.property
            .insert(String::from("rollupFunction"), json!(rollup_function));
        self
    }

    pub fn format(mut self, format: Format) -> PostMagicLookUpFieldReq {
        self.property.insert(String::from("format"), json!(format));
        self
    }
}

pub struct PostFormulaFieldReq {
    field: Map<String, Value>,
    property: Map<String, Value>,
}

impl PostFormulaFieldReq {
    pub fn build(mut self) -> PostFieldReq {
        self.field
            .insert(String::from("property"), json!(self.property));
        PostFieldReq { field: self.field }
    }

    pub fn expression(mut self, expression: String) -> PostFormulaFieldReq {
        self.property
            .insert(String::from("expression"), json!(expression));
        self
    }

    pub fn format(mut self, format: Format) -> PostFormulaFieldReq {
        self.property.insert(String::from("format"), json!(format));
        self
    }
}

pub struct PostAutoNumberFieldReq {
    field: Map<String, Value>,
    property: Map<String, Value>,
}

impl PostAutoNumberFieldReq {
    pub fn build(mut self) -> PostFieldReq {
        self.field
            .insert(String::from("property"), json!(self.property));
        PostFieldReq { field: self.field }
    }
}

pub struct PostCreatedTimeFieldReq {
    field: Map<String, Value>,
    property: Map<String, Value>,
}

impl PostCreatedTimeFieldReq {
    pub fn build(mut self) -> PostFieldReq {
        self.field
            .insert(String::from("property"), json!(self.property));
        PostFieldReq { field: self.field }
    }

    pub fn date_format(mut self, date_format: String) -> PostCreatedTimeFieldReq {
        self.property
            .insert(String::from("dateFormat"), json!(date_format));
        self
    }

    pub fn time_format(mut self, time_format: String) -> PostCreatedTimeFieldReq {
        self.property
            .insert(String::from("timeFormat"), json!(time_format));
        self
    }

    pub fn include_time(mut self, include_time: bool) -> PostCreatedTimeFieldReq {
        self.property
            .insert(String::from("includeTime"), json!(include_time));
        self
    }
}

pub struct PostLastModifiedTimeFieldReq {
    field: Map<String, Value>,
    property: Map<String, Value>,
}

impl PostLastModifiedTimeFieldReq {
    pub fn build(mut self) -> PostFieldReq {
        self.field
            .insert(String::from("property"), json!(self.property));
        PostFieldReq { field: self.field }
    }

    pub fn date_format(mut self, date_format: String) -> PostLastModifiedTimeFieldReq {
        self.property
            .insert(String::from("dateFormat"), json!(date_format));
        self
    }

    pub fn time_format(mut self, time_format: String) -> PostLastModifiedTimeFieldReq {
        self.property
            .insert(String::from("timeFormat"), json!(time_format));
        self
    }

    pub fn include_time(mut self, include_time: bool) -> PostLastModifiedTimeFieldReq {
        self.property
            .insert(String::from("includeTime"), json!(include_time));
        self
    }

    pub fn collect_type(mut self, collect_type: u8) -> PostLastModifiedTimeFieldReq {
        self.property
            .insert(String::from("collectType"), json!(collect_type));
        self
    }

    pub fn field_id_collection(
        mut self,
        field_id_collection: Vec<String>,
    ) -> PostLastModifiedTimeFieldReq {
        self.property.insert(
            String::from("fieldIdCollection"),
            json!(field_id_collection),
        );
        self
    }
}

pub struct PostCreatedByFieldReq {
    field: Map<String, Value>,
    property: Map<String, Value>,
}

impl PostCreatedByFieldReq {
    pub fn build(mut self) -> PostFieldReq {
        self.field
            .insert(String::from("property"), json!(self.property));
        PostFieldReq { field: self.field }
    }
}

pub struct PostLastModifiedByFieldReq {
    field: Map<String, Value>,
    property: Map<String, Value>,
}

impl PostLastModifiedByFieldReq {
    pub fn build(mut self) -> PostFieldReq {
        self.field
            .insert(String::from("property"), json!(self.property));
        PostFieldReq { field: self.field }
    }

    pub fn collect_type(mut self, collect_type: u8) -> PostLastModifiedByFieldReq {
        self.property
            .insert(String::from("collectType"), json!(collect_type));
        self
    }

    pub fn field_id_collection(
        mut self,
        field_id_collection: Vec<String>,
    ) -> PostLastModifiedByFieldReq {
        self.property.insert(
            String::from("fieldIdCollection"),
            json!(field_id_collection),
        );
        self
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_single_field_req() {
        let req = PostFieldReq::builder()
            .name(String::from("name"))
            .as_single_text_req()
            .default_value(String::from("value"))
            .build();
        assert_eq!(
            req.to_json_string().as_str(),
            r#"{"name":"name","property":{"default_value":"value"},"type":"SingleText"}"#
        )
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PostFieldResp {
    #[serde(default)]
    pub id: String,

    #[serde(default)]
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetFieldsResp {
    #[serde(default)]
    pub fields: Vec<Value>,
}
