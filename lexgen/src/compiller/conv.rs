use crate::lexicon::{
    Array, ArrayItem, Object, ObjectProperty, ParameterProperty, Primitive, PrimitveArray,
    XrpcParameters,
};

pub fn params_as_object(p: XrpcParameters) -> Object {
    Object {
        required: p.required,
        description: p.description,
        nullable: Vec::new(),

        properties: p
            .properties
            .into_iter()
            .map(|(name, prop)| (name, param_prop_to_obj_prop(prop)))
            .collect(),

        _type: String::new(),
    }
}

fn param_prop_to_obj_prop(p: ParameterProperty) -> ObjectProperty {
    match p {
        ParameterProperty::Boolean(b) => ObjectProperty::Boolean(b),
        ParameterProperty::Integer(i) => ObjectProperty::Integer(i),
        ParameterProperty::String(s) => ObjectProperty::String(s),
        ParameterProperty::Unknown(u) => ObjectProperty::Unknown(u),
        ParameterProperty::Array(a) => ObjectProperty::Array(primitive_array_to_array(a)),
    }
}

fn primitive_array_to_array(p: PrimitveArray) -> Array {
    Array {
        description: p.description,
        min_lenght: p.min_lenght,
        max_length: p.max_length,
        items: primitive_to_array_item(p.items),
    }
}

fn primitive_to_array_item(p: Primitive) -> ArrayItem {
    match p {
        Primitive::Boolean(b) => ArrayItem::Boolean(b),
        Primitive::Integer(i) => ArrayItem::Integer(i),
        Primitive::String(s) => ArrayItem::String(s),
        Primitive::Unknown(u) => ArrayItem::Unknown(u),
    }
}
