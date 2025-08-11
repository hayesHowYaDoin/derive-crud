#[macro_export]
macro_rules! parse_struct_fields {
    ($input:expr) => {
        match &$input.data {
            syn::Data::Struct(data_struct) => &data_struct.fields,
            _ => {
                // If it's not a struct, emit a compile-time error.
                return syn::Error::new_spanned(
                    $input.ident.clone(),
                    "MyCreate can only be derived for structs",
                )
                .to_compile_error()
                .into();
            }
        }
    };
}

#[macro_export]
macro_rules! parse_table_attribute {
    ($input:expr) => {{
        let mut table_name = None;
        for attr in $input.attrs.iter() {
            if attr.path().is_ident("crud_table") {
                if let syn::Meta::List(value) = &attr.meta {
                    table_name = match table_name {
                        None => Some(value.tokens.to_string()),
                        Some(_) => {
                            return syn::Error::new_spanned(
                                value,
                                "Only one `#[crud_table]` attribute is allowed",
                            )
                            .to_compile_error()
                            .into();
                        }
                    }
                }
            }
        }

        match table_name {
            Some(name) => name,
            None => {
                return syn::Error::new_spanned(
                    $input,
                    "A struct must be annotated with `#[crud_table(\"table_name\")]` to specify the database table",
                )
                .to_compile_error()
                .into();
            }
        }
    }};
}

#[macro_export]
macro_rules! parse_id_attribute {
    ($fields:expr) => {{
        let mut id_field_ident: Option<&syn::Ident> = None;

        for field in $fields.iter() {
            let field_ident = match field.ident.as_ref() {
                Some(ident) => ident,
                None => {
                    return syn::Error::new_spanned(
                        field,
                        "All fields in the struct must have names",
                    )
                    .to_compile_error()
                    .into();
                }
            };

            let is_id_field = field
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("crud_id"));
            if is_id_field {
                if id_field_ident.is_some() {
                    return syn::Error::new_spanned(
                        field,
                        "Only one field can be annotated with `#[crud_id]`",
                    )
                    .to_compile_error()
                    .into();
                }

                id_field_ident = Some(field_ident);
            }
        }

        match id_field_ident {
            Some(ident) => ident,
            None => {
                return syn::Error::new_spanned(
                    $fields,
                    "A struct field must be annotated with `#[crud_id]` to specify the database table",
                )
                .to_compile_error()
                .into();
            }
        }
    }};
}

#[macro_export]
macro_rules! parse_column_fields {
    ($fields:expr) => {{
        let mut attribute_idents = Vec::new();
        let mut attribute_types = Vec::new();

        for field in $fields.iter() {
            let field_ident = match field.ident.as_ref() {
                Some(ident) => ident,
                None => {
                    // If a field does not have a name, emit a compile-time error.
                    return syn::Error::new_spanned(
                        field,
                        "All fields in the struct must have names",
                    )
                    .to_compile_error()
                    .into();
                }
            };
            let field_type = &field.ty;

            let is_id_field = field
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("crud_id"));
            if !is_id_field {
                attribute_idents.push(field_ident);
                attribute_types.push(field_type);
            }
        }

        (attribute_idents, attribute_types)
    }};
}
