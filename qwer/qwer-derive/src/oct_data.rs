use heck::ToPascalCase;
use proc_macro2::{Literal, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::Parse, spanned::Spanned, Attribute, Data, DeriveInput, Error, Fields, Ident, LitInt,
    Meta, MetaNameValue, Result, Token, Type,
};

// Define a struct to hold the parsed data
struct PropertyObject {
    ty: Ident,
    value: LitInt,
}

impl Parse for PropertyObject {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ty = input.parse()?;
        input.parse::<Token![,]>()?;
        let value = input.parse()?;
        Ok(Self { ty, value })
    }
}

pub fn imp(item: &DeriveInput) -> Result<TokenStream> {
    let item_name = &item.ident;

    let (marshal_to, unmarshal_from, lifetimes, format) = match &item.data {
        Data::Struct(data) => {
            let num_fields = data.fields.len() as u16;
            let name = item.ident.to_string();
            let mut marshal_to = quote!();
            let mut unmarshal_from = quote!();
            let mut is_property_object = false;
            let is_root_object = find_attr(&item.attrs, "root").is_some();
            let is_packet = item_name.to_string().starts_with("Rpc")
                || item_name.to_string().starts_with("Ptc");

            let mut lifetimes = quote!();

            for lifetime in item.generics.lifetimes() {
                lifetimes.extend(quote! {#lifetime,});
            }

            let lifetimes = if lifetimes.is_empty() {
                quote!()
            } else {
                quote! {<#lifetimes>}
            };

            // check for #[property_object] attribute]
            if let Some(attr) = find_attr(&item.attrs, "property_object") {
                is_property_object = true;

                // check that all fields are optional
                for field in &data.fields {
                    let ty = &field.ty;
                    let strfy = ty.to_token_stream().to_string();
                    if !strfy.contains("Option") {
                        return Err(Error::new(
                            field.span(),
                            "All fields in a property object must be Option<T>",
                        ));
                    }
                }

                // extract type and value of the property_object attribute
                // e.g. #[property_object(u16, 0x01)]
                if let Ok(PropertyObject { ty, value }) = attr.parse_args() {
                    marshal_to.extend(quote! {
                        bt_property_tag = #value as u16;
                    });

                    unmarshal_from.extend(quote! {
                        bt_property_tag = #value as u16;
                    });

                    marshal_to.extend(match ty.to_string().as_str() {
                        "u8" | "u16" | "u32" | "u64" => {
                            quote! {
                                #ty::marshal_to(&#value, w, bt_property_tag)?;
                            }
                        }
                        _ => Err(Error::new(
                            ty.span(),
                            "Only u8, u16, u32, and u64 are supported for #[property_object]",
                        ))?,
                    });

                    unmarshal_from.extend(match ty.to_string().as_str() {
                        "u8" | "u16" | "u32" | "u64" => {
                            quote! {
                                assert!(#ty::unmarshal_from(r, bt_property_tag)? == #value, "Failed for {}", #name);
                            }
                        }
                        _ => Err(Error::new(
                            ty.span(),
                            "Only u8, u16, u32, and u64 are supported for #[property_object]",
                        ))?,
                    });
                } else {
                    marshal_to.extend(quote! {
                        if bt_property_tag == 0 {
                            #num_fields.marshal_to(w, bt_property_tag)?;
                        }
                    });

                    unmarshal_from.extend(quote! {
                        if bt_property_tag == 0 {
                            assert!(u16::unmarshal_from(r, bt_property_tag)? == #num_fields, "Failed for {}", #name);
                        }
                    });
                }

                let generated_ident = generate_ident(0, Span::call_site());

                let mut loop_count_fields = quote! {
                    let mut #generated_ident = 0;
                };

                for field in &data.fields {
                    let ident = &field.ident;
                    loop_count_fields.extend(quote! {
                        if self.#ident.is_some() {
                            #generated_ident += 1;
                        }
                    });
                }

                // extend with # of fields that are NOT none
                marshal_to.extend(quote! {
                    #loop_count_fields
                    (#generated_ident as u16).marshal_to(w, bt_property_tag)?;
                });

                let num_fields = data.fields.len() as u16;
                unmarshal_from.extend(quote! {
                    let num_fields: u16 = ::qwer::OctData::unmarshal_from(r, bt_property_tag)?;
                    if num_fields > #num_fields {
                        return Err(::std::io::Error::new(
                            ::std::io::ErrorKind::InvalidData,
                            format!("Unexpected number of fields, expected at most {}, got {}", #num_fields, num_fields),
                        ));
                    }
                });
            } else if !is_packet {
                marshal_to.extend(quote! {
                    if bt_property_tag == 0 {
                        #num_fields.marshal_to(w, bt_property_tag)?;
                    }
                });
                unmarshal_from.extend(quote! {
                    if bt_property_tag == 0 {
                        assert!(u16::unmarshal_from(r, bt_property_tag)? == #num_fields, "Failed for {}", #name);
                    }
                });
            }

            marshal_to.extend(write_fields(
                &data.fields,
                |ident| quote!(self.#ident),
                |i, _| {
                    let i = Literal::usize_unsuffixed(i);
                    quote!(self.#i)
                },
                if is_property_object {
                    write_property_field
                } else {
                    write_field
                },
                is_root_object,
                None,
            ));

            let unmarshal_body = if is_property_object {
                // we need to wrap the reads in a for loop, otherwise just assign by default for
                // every field type
                let mut fields = quote!();
                let mut nones = quote!();
                let mut cases = quote!();

                for field in &data.fields {
                    let ident = &field.ident;

                    fields.extend(quote! {
                        mut #ident,
                    });

                    nones.extend(quote! {
                        None,
                    });

                    let tag_attr = find_attr(&field.attrs, "tag").ok_or_else(|| {
                        Error::new(field.span(), "Property fields must have a #[tag]")
                    })?;
                    let Meta::NameValue(MetaNameValue { ref value, .. }) = tag_attr.meta else {
                        return Err(Error::new(
                            tag_attr.span(),
                            format!("Expected #[tag = <value>] for field {ident:?}"),
                        ));
                    };

                    let case = value
                        .into_token_stream()
                        .to_string()
                        .parse::<u16>()
                        .unwrap();

                    if find_attr(&field.attrs, "skip_property").is_some() {
                        if is_root_object {
                            cases.extend(quote! {
                                #case => { u32::unmarshal_from(r, bt_property_tag)?; #ident = None }
                            });
                        } else {
                            cases.extend(quote! {
                                #case => { #ident = None }
                            });
                        }
                    } else {
                        let read_case =
                            read_property_field(&field.ty, &field.attrs, is_root_object, None);

                        cases.extend(quote! {
                            #case => { #ident = Some(#read_case) }
                        });
                    }
                }

                let field_assign = quote! {
                    let ( #fields ) = ( #nones );
                };

                unmarshal_from.extend(field_assign);

                let r#for = quote!(
                    for _ in 0..num_fields {
                        let tag: u16 = ::qwer::OctData::unmarshal_from(r, bt_property_tag)?;
                        match tag {
                            #cases
                            libnignhaccjpkd_so => {
                                return Err(::std::io::Error::new(
                                    ::std::io::ErrorKind::InvalidData,
                                    format!("Unexpected tag {}", libnignhaccjpkd_so),
                                ));
                            }
                        }
                    }
                );

                let mut result_fields = quote!();
                for field in &data.fields {
                    let ident = &field.ident;
                    result_fields.extend(quote! {
                        #ident,
                    });
                }

                quote! {
                    #r#for
                    Ok(Self {
                        #result_fields
                    })
                }
            } else {
                let reads = read_fields(
                    &data.fields,
                    if is_property_object {
                        read_property_field
                    } else {
                        read_field
                    },
                    is_root_object,
                    None,
                );

                quote! {
                    Ok(Self #reads)
                }
            };

            unmarshal_from.extend(unmarshal_body);

            let mut format = quote!();
            for field in &data.fields {
                let ident = &field.ident;
                let ty = &field.ty;
                if ident.as_ref().unwrap().to_string().starts_with("unk") {
                    let strfy = quote!(#ty).to_string();
                    if strfy.starts_with("Option") && strfy.ends_with('>') && strfy.contains('<') {
                        let inner = strfy[7..strfy.len() - 1].to_string();
                        format.extend(quote! {
                            writeln!(f, "({}) {:?}", #inner, self.#ident)?;
                        });
                    } else {
                        format.extend(quote! {
                            writeln!(f, "({}) {}", #strfy, self.#ident)?;
                        });
                    }
                } else {
                    let strfy = ident.as_ref().unwrap().to_string().to_pascal_case();
                    let ty_name = quote!(#ty).to_string();
                    if (ty_name.contains("Option")
                        || ty_name.contains("Vec")
                        || ty_name.contains("HashSet")
                        || ty_name.contains("HashMap"))
                        && ty_name.ends_with('>')
                        && ty_name.contains('<')
                    {
                        format.extend(quote! {
                            writeln!(f, "({}) {:?}", #strfy, self.#ident)?;
                        });
                    } else {
                        format.extend(quote! {
                            writeln!(f, "({}) {}", #strfy, self.#ident)?;
                        });
                    }
                }
            }

            (marshal_to, unmarshal_from, lifetimes, format)
        }
        Data::Enum(data) => {
            let repr_attr = find_attr(&item.attrs, "repr")
                .ok_or_else(|| Error::new(item.span(), "Enum packets must declare #[repr]"))?;
            let repr_ty = repr_attr.parse_args::<Ident>()?;

            let mut marshal_vars = Vec::with_capacity(data.variants.len());
            let mut unmarshal_vars = Vec::with_capacity(data.variants.len());
            let mut print_vars = Vec::with_capacity(data.variants.len());

            let base = find_attr(&item.attrs, "base")
                .map(|attr| {
                    let Meta::NameValue(MetaNameValue { ref value, .. }) = attr.meta else {
                        panic!("Failed to get base field count")
                    };
                    value
                        .into_token_stream()
                        .to_string()
                        .parse::<i16>()
                        .map_err(|_| {
                            Error::new(
                                value.span(),
                                format!(
                                    "Failed to parse base field count ({}) as a valid u16",
                                    value.to_token_stream()
                                ),
                            )
                        })
                })
                .transpose()?;

            for variant in &data.variants {
                let var_name = &variant.ident;
                let (_, discrim) = variant.discriminant.as_ref().ok_or_else(|| {
                    Error::new(
                        variant.span(),
                        "All enum packet variants must have discriminants",
                    )
                })?;
                let polymorphic_none = find_attr(&variant.attrs, "polymorphic_none");

                let fields_pat = pat_fields(&variant.fields);

                let fields_write = write_fields(
                    &variant.fields,
                    |ident| quote!(#ident),
                    |id, span| {
                        let ident = generate_ident(id, span);
                        quote!(#ident)
                    },
                    write_field,
                    false,
                    base,
                );
                let fields_read = read_fields(&variant.fields, read_field, false, base);
                let fields_print = print_fields(&variant.fields);

                let marshal_header = quote! {
                    #repr_ty::marshal_to(&#discrim, w, bt_property_tag)?;
                };

                let unmarshal_header = quote!();
                if polymorphic_none.is_none() {
                    marshal_vars.push(quote!(#item_name::#var_name #fields_pat => {
                        #marshal_header
                        #fields_write
                    }));
                    unmarshal_vars.push(quote! {#discrim => {
                        #unmarshal_header
                        Ok(#item_name::#var_name #fields_read)
                    }});
                } else {
                    marshal_vars.push(quote!(#item_name::#var_name #fields_pat => {
                        #marshal_header
                    }));
                    unmarshal_vars.push(quote! {#discrim => {
                        Ok(#item_name::#var_name {})
                    }});
                }

                print_vars.push(quote!(#item_name::#var_name #fields_pat => {
                    #fields_print
                }));
            }

            let marshal_to = quote! {
                match self {
                    #(#marshal_vars),*
                }
            };

            let read_repr = quote!(#repr_ty);

            let unmarshal_from = if base == Some(0) {
                quote! {{
                    let id = #read_repr::unmarshal_from(r, bt_property_tag)?;
                    if id != 0xFFFF {
                        assert!(u16::unmarshal_from(r, bt_property_tag)? == 0);
                    }
                    match id {
                        #(#unmarshal_vars,)*
                        _ => Err(::std::io::Error::new(
                            ::std::io::ErrorKind::InvalidData,
                            format!("Unexpected discriminant {}", id),
                        ))?,
                    }
                }}
            } else {
                quote! {{
                    let id = #read_repr::unmarshal_from(r, bt_property_tag)?;
                    match id {
                        #(#unmarshal_vars,)*
                        _ => Err(::std::io::Error::new(
                            ::std::io::ErrorKind::InvalidData,
                            format!("Unexpected discriminant {}", id),
                        ))?,
                    }
                }}
            };
            let format = quote! {
                match self {
                    #(#print_vars),*
                }
            };
            (marshal_to, unmarshal_from, quote!(), format)
        }
        Data::Union(_) => Err(Error::new(
            item.span(),
            "Unions cannot be derived as Packet",
        ))?,
    };

    Ok(quote! {
        #[automatically_derived]
        impl #lifetimes ::qwer::OctData for #item_name #lifetimes {
            fn marshal_to<W: ::std::io::Write>(&self, w: &mut W, mut bt_property_tag: u16) -> ::std::io::Result<()> {
                use ::qwer::OctData;
                #marshal_to
                Ok(())
            }

            fn unmarshal_from<R: ::std::io::Read>(r: &mut R, mut bt_property_tag: u16) -> ::std::io::Result<Self> {
                use ::qwer::OctData;
                #unmarshal_from
            }
        }

        #[automatically_derived]
        impl #lifetimes ::std::fmt::Display for #item_name #lifetimes {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                #format
                Ok(())
            }
        }
    })
}

fn find_attr<'a, I, S>(attr: I, name: S) -> Option<&'a Attribute>
where
    I: IntoIterator<Item = &'a Attribute>,
    S: AsRef<str>,
{
    attr.into_iter().find(|attr| attr.path().is_ident(&name))
}

fn write_fields<F, G, H>(
    fields: &Fields,
    access_named: F,
    access_unnamed: G,
    write: H,
    is_root: bool,
    base: Option<i16>,
) -> TokenStream
where
    F: Fn(&Ident) -> TokenStream,
    G: Fn(usize, Span) -> TokenStream,
    H: Fn(&Type, &[Attribute], &TokenStream, bool, Option<i16>) -> TokenStream,
{
    let mut operations = if let Some(base) = base {
        if base == 0 {
            quote! {
                if bt_property_tag == 0 {
                    0u16.marshal_to(w, bt_property_tag)?;
                }
            }
        } else {
            quote!()
        }
    } else {
        quote!()
    };

    let fields_operations = match fields {
        Fields::Named(fields) => {
            let fields = fields
                .named
                .iter()
                .enumerate()
                .map(|(i, field)| {
                    let ident = field.ident.as_ref().unwrap();
                    let accessor = access_named(ident);
                    if let Some(base) = base {
                        if i == 0 && base != 0 {
                            write(&field.ty, &field.attrs, &accessor, is_root, Some(base))
                        } else if i == base as usize {
                            write(
                                &field.ty,
                                &field.attrs,
                                &accessor,
                                is_root,
                                Some(fields.named.len() as i16 - base),
                            )
                        } else if i == fields.named.len() - 1 && i < base as usize {
                            let mut write_op =
                                write(&field.ty, &field.attrs, &accessor, is_root, None);
                            let num = base - fields.named.len() as i16;
                            write_op.extend(quote! {
                                if bt_property_tag == 0 {
                                    (#num as u16).marshal_to(w, bt_property_tag)?;
                                }
                            });
                            write_op
                        } else {
                            write(&field.ty, &field.attrs, &accessor, is_root, None)
                        }
                    } else {
                        write(&field.ty, &field.attrs, &accessor, is_root, None)
                    }
                })
                .collect::<Vec<_>>();
            quote!(#(#fields)*)
        }
        Fields::Unnamed(fields) => {
            let fields = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, field)| {
                    let accessor = access_unnamed(i, field.span());
                    if let Some(base) = base {
                        if i == 0 && base != 0 {
                            write(&field.ty, &field.attrs, &accessor, is_root, Some(base))
                        } else if i == base as usize {
                            write(
                                &field.ty,
                                &field.attrs,
                                &accessor,
                                is_root,
                                Some(fields.unnamed.len() as i16 - base),
                            )
                        } else if i == fields.unnamed.len() - 1 && i < base as usize {
                            write(
                                &field.ty,
                                &field.attrs,
                                &accessor,
                                is_root,
                                Some(base - fields.unnamed.len() as i16),
                            )
                        } else {
                            write(&field.ty, &field.attrs, &accessor, is_root, None)
                        }
                    } else {
                        write(&field.ty, &field.attrs, &accessor, is_root, None)
                    }
                })
                .collect::<Vec<_>>();
            quote!(#(#fields)*)
        }
        Fields::Unit => quote!(),
    };
    operations.extend(fields_operations);

    if let Some(base) = base {
        let field_num = match fields {
            Fields::Named(fields) => fields.named.len(),
            Fields::Unnamed(fields) => fields.unnamed.len(),
            Fields::Unit => 0,
        };

        if field_num == base as usize && base == 1 {
            operations.extend(quote! {
                if bt_property_tag == 0 {
                    0u16.marshal_to(w, bt_property_tag)?;
                }
            });
        }
    }

    operations
}

fn read_fields<R>(fields: &Fields, read: R, is_root: bool, base: Option<i16>) -> TokenStream
where
    R: Fn(&Type, &[Attribute], bool, Option<i16>) -> TokenStream,
{
    match fields {
        Fields::Named(fields) => {
            let fields = fields
                .named
                .iter()
                .enumerate()
                .map(|(i, field)| {
                    let field_name = &field.ident;
                    let read_expr = if let Some(base) = base {
                        if i == 0 && base != 0 {
                            let read_op = read(&field.ty, &field.attrs, is_root, Some(base));
                            if base == 1 && fields.named.len() == base as usize {
                                quote! {{
                                    let ret = #read_op;
                                    if bt_property_tag == 0 {
                                        assert!(u16::unmarshal_from(r, bt_property_tag)? == 0);
                                    }
                                    ret
                                }}
                            } else {
                                read_op
                            }
                        } else if i == base as usize {
                            read(
                                &field.ty,
                                &field.attrs,
                                is_root,
                                Some(fields.named.len() as i16 - base),
                            )
                        } else if i == fields.named.len() - 1 && i < base as usize {
                            read(
                                &field.ty,
                                &field.attrs,
                                is_root,
                                Some(base - fields.named.len() as i16),
                            )
                        } else {
                            read(&field.ty, &field.attrs, is_root, None)
                        }
                    } else {
                        read(&field.ty, &field.attrs, is_root, None)
                    };
                    quote!(#field_name: #read_expr)
                })
                .collect::<Vec<_>>();
            quote!({ #(#fields),* })
        }
        Fields::Unnamed(fields) => {
            let fields = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, field)| {
                    let read_expr = if let Some(base) = base {
                        if i == 0 {
                            read(&field.ty, &field.attrs, is_root, Some(base))
                        } else if i == base as usize {
                            read(
                                &field.ty,
                                &field.attrs,
                                is_root,
                                Some(fields.unnamed.len() as i16 - base),
                            )
                        } else if i == fields.unnamed.len() - 1 && i < base as usize {
                            read(
                                &field.ty,
                                &field.attrs,
                                is_root,
                                Some(base - fields.unnamed.len() as i16),
                            )
                        } else {
                            read(&field.ty, &field.attrs, is_root, None)
                        }
                    } else {
                        read(&field.ty, &field.attrs, is_root, None)
                    };
                    quote!(#read_expr)
                })
                .collect::<Vec<_>>();
            quote!(( #(#fields),* ))
        }
        Fields::Unit => quote!(),
    }
}

fn print_fields(fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(fields) => {
            let fields = fields
                .named
                .iter()
                .map(|field| {
                    let ident = field.ident.as_ref().unwrap();
                    // let accessor = access_named(ident);
                    if ident.to_string().starts_with("unk") {
                        let strfy = quote!(#field.ty).to_string();
                        // check if field.ty is an option<t>
                        if (strfy.starts_with("Option")
                            || strfy.starts_with("Vec")
                            || strfy.starts_with("HashSet")
                            || strfy.starts_with("HashMap"))
                            && strfy.ends_with('>')
                            && strfy.contains('<')
                        {
                            let inner = strfy[7..strfy.len() - 1].to_string();
                            quote! {writeln!(f, "({}) {:?}", #inner, #ident)?;}
                        } else {
                            quote! {writeln!(f, "({}) {}", #strfy, #ident)?;}
                        }
                    } else {
                        let strfy = ident.to_string().to_pascal_case();
                        let ty = quote!(#field.ty).to_string();
                        if (ty.contains("Option")
                            || ty.contains("Vec")
                            || ty.contains("HashSet")
                            || ty.contains("HashMap"))
                            && ty.contains('>')
                            && ty.contains('<')
                        {
                            let inner = match ty.find('<') {
                                Some(i) => ty[i + 1..ty.len() - 1].to_string(),
                                None => ty,
                            };
                            quote! {writeln!(f, "({}) {:?}", #inner, #ident)?;}
                        } else {
                            quote! {writeln!(f, "({}) {}", #strfy, #ident)?;}
                        }
                    }
                })
                .collect::<Vec<_>>();
            quote!(#(#fields)*)
        }
        Fields::Unnamed(fields) => {
            let fields = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(_, field)| {
                    // let accessor = access_unnamed(i, field.span());
                    // let ident = field.ident.as_ref().unwrap();
                    // if ident.to_string().starts_with("unk") {
                    //     let strfy = quote!(#field.ty).to_string();
                    //     quote! {writeln!(f, "({}) {}", #strfy, #ident)?;}
                    // } else {
                    //     let strfy = ident.to_string().to_pascal_case();
                    //     quote! { writeln!(f, "({}) {}", #strfy, #ident)?;}
                    // }
                    let strfy = quote!(#field.ty).to_string();
                    quote! {writeln!(f, "({}) {}", #strfy, #field)?;}
                })
                .collect::<Vec<_>>();
            quote!(#(#fields)*)
        }
        Fields::Unit => quote!(),
    }
}

fn pat_fields(fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(fields) => {
            let fields = fields
                .named
                .iter()
                .map(|field| field.ident.as_ref().unwrap());
            quote!({ #(#fields),* })
        }
        Fields::Unnamed(fields) => {
            let fields = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, field)| generate_ident(i, field.span()));
            quote!(( #(#fields),* ))
        }
        Fields::Unit => quote!(),
    }
}

fn write_field(
    _ty: &Type,
    attrs: &[Attribute],
    expr: &TokenStream,
    _is_root: bool,
    base_to_write: Option<i16>,
) -> TokenStream {
    let mut property_quote = quote!();

    if let Some(property_attr) = find_attr(attrs, "property_object") {
        let PropertyObject { ty, value } = property_attr.parse_args().unwrap();
        match ty.to_string().as_str() {
            "u8" | "u16" | "u32" | "u64" => {
                let scratch = quote! {w};
                property_quote = quote! {
                        #ty::marshal_to(&#value, #scratch, bt_property_tag)?;
                };
            }
            _ => panic!("Only u8, u16, u32, and u64 are supported for #[property_object]"),
        }
    }

    let mut negative = false;
    if let Some(base_to_write) = base_to_write {
        negative = base_to_write < 0;
        let base_to_write = base_to_write.unsigned_abs();
        property_quote = quote! {
            if bt_property_tag == 0 {
                #base_to_write.marshal_to(w, bt_property_tag)?;
            }
        };
    }

    if find_attr(attrs, "property_blob").is_some() {
        // scratch and encode length then extend
        quote! {{
            #property_quote
            let mut scratch = ::std::io::Cursor::new(Vec::new());
            #expr.marshal_to(&mut scratch, bt_property_tag)?;
            let len = scratch.get_ref().len() as u32;
            len.marshal_to(w, bt_property_tag)?;
            w.write_all(scratch.get_ref())?;
        }}
    } else if negative {
        quote! {
            #expr.marshal_to(w, bt_property_tag)?;
            #property_quote
        }
    } else {
        quote! {
            #property_quote
            #expr.marshal_to(w, bt_property_tag)?;
        }
    }
}

// Tags encode the id, length, and then the actual field
fn write_property_field(
    _ty: &Type,
    attrs: &[Attribute],
    expr: &TokenStream,
    is_root: bool,
    _base_to_write: Option<i16>,
) -> TokenStream {
    // If we have a `property_object` attribute then write a PropertyObject beforehand of the
    // specified size like with the top-level struct

    let mut property_quote = quote!();

    if let Some(property_attr) = find_attr(attrs, "property_object") {
        let PropertyObject { ty, value } = property_attr.parse_args().unwrap();
        match ty.to_string().as_str() {
            "u8" | "u16" | "u32" | "u64" => {
                let scratch = if is_root {
                    quote! {scratch}
                } else {
                    quote! {w}
                };
                property_quote = if is_root {
                    quote! {
                        #ty::marshal_to(&#value, &mut #scratch, bt_property_tag)?;
                    }
                } else {
                    quote! {
                        #ty::marshal_to(&#value, #scratch, bt_property_tag)?;
                    }
                };
            }
            _ => panic!("Only u8, u16, u32, and u64 are supported for #[property_object]"),
        }
    }

    let Some(tag_attr) = find_attr(attrs, "tag") else {
        panic!("tag attribute missing!")
    };

    let Meta::NameValue(MetaNameValue { ref value, .. }) = tag_attr.meta else {
        panic!("Failed to get tag id")
    };

    if find_attr(attrs, "skip_property").is_none() {
        if is_root {
            quote! {{
                if #expr.is_some() {
                    let mut scratch = ::std::io::Cursor::new(Vec::new());
                    #property_quote
                    #expr.marshal_to(&mut scratch, bt_property_tag)?;
                    (#value as u16).marshal_to(w, bt_property_tag)?;
                    let len = scratch.get_ref().len() as u32;
                    len.marshal_to(w, bt_property_tag)?;
                    w.write_all(scratch.get_ref())?;
                }
            }}
        } else {
            quote! {
                if #expr.is_some() {
                    (#value as u16).marshal_to(w, bt_property_tag)?;
                    #property_quote
                    #expr.marshal_to(w, bt_property_tag)?;
                }
            }
        }
    } else if is_root {
        quote! {
            if #expr.is_some() {
                (#value as u16).marshal_to(w, bt_property_tag)?;
                #property_quote
                0u32.marshal_to(w, bt_property_tag)?;
            }
        }
    } else {
        quote! {
            if #expr.is_some() {
                (#value as u16).marshal_to(w, bt_property_tag)?;
                #property_quote
            }
        }
    }
}

fn read_field(
    _ty: &Type,
    attrs: &[Attribute],
    _is_root: bool,
    base_to_read: Option<i16>,
) -> TokenStream {
    if find_attr(attrs, "property_blob").is_some() {
        // read length, then read the field
        quote! {{
            let len = u32::unmarshal_from(r, bt_property_tag)?;
            let mut buf = vec![0; len as usize];
            r.read_exact(&mut buf)?;
            let mut scratch = ::std::io::Cursor::new(buf);
            ::qwer::OctData::unmarshal_from(&mut scratch, bt_property_tag)?
        }}
    } else if let Some(base_to_read) = base_to_read {
        if base_to_read > 0 {
            let base_to_read = base_to_read.unsigned_abs();
            quote! {{
                if bt_property_tag == 0 {
                    assert!(u16::unmarshal_from(r, bt_property_tag)? == #base_to_read);
                }
                ::qwer::OctData::unmarshal_from(r, bt_property_tag)?
            }}
        } else {
            let base_to_read = base_to_read.unsigned_abs();
            let generated_ident = generate_ident(1, Span::call_site());
            quote! {{
                let #generated_ident = ::qwer::OctData::unmarshal_from(r, bt_property_tag)?;
                if bt_property_tag == 0 {
                    assert!(u16::unmarshal_from(r, bt_property_tag)? == #base_to_read);
                }
                #generated_ident
            }}
        }
    } else {
        quote! {{
            ::qwer::OctData::unmarshal_from(r, bt_property_tag)?
        }}
    }
}

fn read_property_field(
    _ty: &Type,
    attrs: &[Attribute],
    is_root: bool,
    _base_to_read: Option<i16>,
) -> TokenStream {
    let mut header = if is_root {
        quote! {
            let len = u32::unmarshal_from(r, bt_property_tag)?;
        }
    } else {
        quote!()
    };

    if let Some(attr) = find_attr(attrs, "property_object") {
        let PropertyObject { ty, value } = attr.parse_args().unwrap();
        match ty.to_string().as_str() {
            "u8" | "u16" | "u32" | "u64" => {
                let pre_length = match ty.to_string().as_str() {
                    "u8" => 1,
                    "u16" => 2,
                    "u32" => 4,
                    "u64" => 8,
                    _ => panic!("Not allowed type"),
                } as u32;

                if is_root {
                    header = quote! {
                        let len = u32::unmarshal_from(r, bt_property_tag)? - #pre_length;
                    };
                }
                header = quote! {
                    #header
                    assert!(#ty::unmarshal_from(r, bt_property_tag)? == #value);
                };
            }
            _ => panic!("Only u8, u16, u32, and u64 are supported for #[property_object]"),
        }
    }

    if is_root {
        quote! {{
            #header
            let mut buf = vec![0; len as usize];
            r.read_exact(&mut buf)?;
            let mut scratch = ::std::io::Cursor::new(buf);
            ::qwer::OctData::unmarshal_from(&mut scratch, bt_property_tag)?
        }}
    } else {
        quote! {{
            #header
            ::qwer::OctData::unmarshal_from(r, bt_property_tag)?
        }}
    }
}

fn generate_ident(i: usize, span: Span) -> Ident {
    Ident::new(&format!("generated_ident_{i}"), span)
}
