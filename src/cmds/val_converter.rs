use mongodb::bson::{Bson, Document};
use nu_protocol::{LabeledError, Record, Span, Value};

pub fn doc_to_value(doc: Document, span: Span) -> Value {
    let mut rec = Record::new();

    for (k, v) in doc {
        let val = match v {
            Bson::Null => Value::nothing(span),
            Bson::Double(n) => Value::float(n, span),
            Bson::String(s) => Value::string(s, span),
            Bson::Boolean(v) => Value::bool(v, span),
            Bson::Int32(i) => Value::int(i.into(), span),
            Bson::Int64(i) => Value::int(i.into(), span),
            Bson::ObjectId(oid) => Value::string(oid.to_string(), span),
            Bson::Document(d) => doc_to_value(d, span),
            Bson::DateTime(dt) => Value::date(dt.to_chrono().into(), span),
            Bson::Binary(b) => Value::binary(b.bytes, span),
            other => Value::string(other.to_string(), span),
        };
        rec.push(k, val);
    }
    Value::record(rec, span)
}

fn to_bson(v: Value) -> Result<Bson, LabeledError> {
    let val_span = v.span();
    let bson_val = match v {
        Value::Record { val, .. } => Bson::Document(value_to_doc(val.into_owned())?),
        Value::Int { val, .. } => Bson::Int64(val),
        Value::Bool { val, .. } => Bson::Boolean(val),
        Value::Float { val, .. } => Bson::Double(val),
        Value::String { val, .. } | Value::Glob { val, .. } => {
            // FIXME: there are some issues on record value interpolation
            // see more: https://github.com/nushell/nushell/issues/13602
            // // may parse as an mongodb ObjectId
            // if val.starts_with("ObjectId(\"") && val.ends_with("\")") {
            //     let object_id = val
            //         .trim_start_matches("ObjectId(\"")
            //         .trim_end_matches("\")");
            //     match bson::oid::ObjectId::parse_str(object_id) {
            //         Err(e) => {
            //             return Err(LabeledError::new(format!(
            //                 "invalid ObjectId, detailed: {e}"
            //             )))
            //         }
            //         Ok(object_id) => Bson::ObjectId(object_id),
            //     let object_id = val.trim_start_matches("ObjectId(\'").trim_end_matches("')");
            //     match bson::oid::ObjectId::parse_str(object_id) {
            //         Err(e) => {
            //             return Err(LabeledError::new(format!(
            //                 "invalid ObjectId, detailed: {e}"
            //             )))
            //         }
            //         Ok(object_id) => Bson::ObjectId(object_id),
            //     }
            // } else {
            //     Bson::String(val)
            // }
            //
            // A workaround
            if val.starts_with("ObjectId") {
                let object_id = val.trim_start_matches("ObjectId");
                match bson::oid::ObjectId::parse_str(object_id) {
                    Err(e) => {
                        return Err(LabeledError::new(format!("invalid ObjectId"))
                            .with_label(format!("{e}"), val_span))
                    }
                    Ok(object_id) => Bson::ObjectId(object_id),
                }
            } else {
                Bson::String(val)
            }
        }
        Value::Date { val, .. } => {
            Bson::DateTime(bson::DateTime::from_chrono::<chrono::Utc>(val.into()))
        }
        Value::Binary { val, .. } => Bson::Binary(bson::Binary {
            subtype: bson::spec::BinarySubtype::Generic,
            bytes: val,
        }),
        Value::List { vals, .. } => {
            let mut array_vals = vec![];
            for v in vals {
                array_vals.push(to_bson(v)?)
            }
            Bson::Array(array_vals)
        }
        other => {
            return Err(
                LabeledError::new(format!("can't convert to mongo doc")).with_label(
                    format!("invalid value type: {}", other.get_type()),
                    val_span,
                ),
            )
        }
    };
    Ok(bson_val)
}

pub fn value_to_doc(val: Record) -> Result<Document, LabeledError> {
    let mut doc = Document::new();
    for (k, v) in Record::clone(&val).into_iter() {
        let bson_val = to_bson(v)?;
        doc.insert(k, bson_val);
    }
    Ok(doc)
}
