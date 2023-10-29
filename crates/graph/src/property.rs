use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Inherited;

macro_rules! decl_properties {
    ($(($name: literal, $prop: ident: [$($t: ty)|+])),+) => {
        paste::paste!{
            #[derive(Debug, Clone, PartialEq)]
            pub enum Property {
                $($prop([<$prop Property>])),
                +
            }

            impl Property {
                pub fn kind(&self) -> PropertyKind {
                    match self {
                        $(Property::$prop(_) => PropertyKind::$prop),
                        +
                    }
                }

                pub fn name(&self) -> &'static str {
                    self.kind().name()
                }
            }

            $(impl From<[<$prop Property>]> for Property {
                fn from(property: [<$prop Property>]) -> Self {
                    Property::[<$prop>](property)
                }
            })+
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(u16)]
        pub enum PropertyKind {
            $($prop),
            +
        }

        impl PropertyKind {
            pub fn name(&self) -> &'static str {
                [
                    $($name),+
                ][*self as u16 as usize]
            }
        }

        static PROPERTY_KINDS: phf::Map<&'static str, PropertyKind> = phf::phf_map! {
            $($name => PropertyKind::$prop),
            +
        };

        $(paste::paste!{
            #[derive(Debug, Clone, PartialEq)]
            pub enum [<$prop Property>] {
                $([<$t:camel>]($t)),
                +
            }

            impl [<$prop Property>] {
                pub const NAME: &'static str = $name;
                pub const KIND: PropertyKind = PropertyKind::$prop;

                $(pub fn [<new_ $t:snake:lower>](value: $t) -> Self {
                    [<$prop Property>]::[<$t:camel>](value)
                })+
            }

            $(impl From<$t> for [<$prop Property>] {
                fn from(value: $t) -> Self {
                    [<$prop Property>]::[<$t:camel>](value)
                }
            })+

            impl Properties {
                pub fn [<get_ $prop:snake:lower>](&self) -> Option<&[<$prop Property>]> {
                    match self.properties.get(&PropertyKind::$prop) {
                        Some(Property::$prop(it)) => Some(it),
                        _ => None
                    }
                }

                pub fn [<get_ $prop:snake:lower _mut>](&mut self) -> Option<&mut [<$prop Property>]> {
                    match self.properties.get_mut(&PropertyKind::$prop) {
                        Some(Property::$prop(it)) => Some(it),
                        _ => None
                    }
                }

                pub fn [<set_ $prop:snake:lower>](&mut self, value: impl Into<[<$prop Property>]>) {
                    self.properties.insert(PropertyKind::$prop, Property::$prop(value.into()));
                }
            }
        })*
    };
}

decl_properties![
    ("background", Background: [Inherited | Color]),
    ("foreground", Foreground: [Inherited | Color]),
    ("font-size", FontSize: [Inherited | f32]),
    ("label", Label: [String])
];

impl<'a> TryFrom<&'a str> for PropertyKind {
    type Error = ();

    #[inline]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        PROPERTY_KINDS.get(value).cloned().ok_or(())
    }
}

impl TryFrom<String> for PropertyKind {
    type Error = ();

    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        PROPERTY_KINDS.get(&value).cloned().ok_or(())
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Properties {
    properties: HashMap<PropertyKind, Property>,
}

impl Properties {
    pub fn new() -> Self {
        Properties {
            properties: HashMap::new(),
        }
    }

    pub fn get(&self, property: impl TryInto<PropertyKind>) -> Option<&Property> {
        self.properties.get(&property.try_into().ok()?)
    }

    pub fn set(&mut self, property: Property) {
        self.properties.insert(property.kind(), property);
    }

    // get_mut would allow invalid states
}

/*
Issues:
- properties are not defined per-kind
  - makes it seem as if unsupported properties are supported
  - would probably require proc-macro to handle
- no support for custom properties
  - requirement for layout engines
- Inherited is not most ergonomic
*/