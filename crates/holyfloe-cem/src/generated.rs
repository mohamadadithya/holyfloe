// Generated once by typify, checked in (D-031). Do not edit by hand.
// Regenerate from crates/holyfloe-cem: cargo typify --no-builder schema/schema-2.1.0.json -o src/generated.rs && cargo fmt
// cargo-typify 0.8.0, schema from npm custom-elements-manifest@2.1.0.

#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = "`Attribute`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Attribute {
    #[doc = "The default value of the attribute, if any.\n\nAs attributes are always strings, this is the actual value, not a human\nreadable description."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<::std::string::String>,
    #[doc = "Whether the attribute is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub deprecated: ::std::option::Option<AttributeDeprecated>,
    #[doc = "A markdown description."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the field this attribute is associated with, if any."]
    #[serde(
        rename = "fieldName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub field_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "inheritedFrom",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub inherited_from: ::std::option::Option<Reference>,
    pub name: ::std::string::String,
    #[doc = "A markdown summary suitable for display in a listing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[doc = "The type that the attribute will be serialized/deserialized as."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<Type>,
}
#[doc = "Whether the attribute is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum AttributeDeprecated {
    Boolean(bool),
    String(::std::string::String),
}
impl ::std::fmt::Display for AttributeDeprecated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for AttributeDeprecated {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
#[doc = "`ClassDeclaration`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ClassDeclaration {
    #[doc = "Whether the class or mixin is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub deprecated: ::std::option::Option<ClassDeclarationDeprecated>,
    #[doc = "A markdown description of the class."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub kind: ClassDeclarationKind,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub members: ::std::vec::Vec<ClassDeclarationMembersItem>,
    #[doc = "Any class mixins applied in the extends clause of this class.\n\nIf mixins are applied in the class definition, then the true superclass\nof this class is the result of applying mixins in order to the superclass.\n\nMixins must be listed in order of their application to the superclass or\nprevious mixin application. This means that the innermost mixin is listed\nfirst. This may read backwards from the common order in JavaScript, but\nmatches the order of language used to describe mixin application, like\n\"S with A, B\"."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub mixins: ::std::vec::Vec<Reference>,
    pub name: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<SourceReference>,
    #[doc = "A markdown summary suitable for display in a listing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[doc = "The superclass of this class.\n\nIf this class is defined with mixin applications, the prototype chain\nincludes the mixin applications and the true superclass is computed\nfrom them."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub superclass: ::std::option::Option<Reference>,
}
#[doc = "Whether the class or mixin is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ClassDeclarationDeprecated {
    Boolean(bool),
    String(::std::string::String),
}
impl ::std::fmt::Display for ClassDeclarationDeprecated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for ClassDeclarationDeprecated {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
#[doc = "`ClassDeclarationKind`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ClassDeclarationKind {
    #[serde(rename = "class")]
    Class,
}
impl ::std::fmt::Display for ClassDeclarationKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Class => f.write_str("class"),
        }
    }
}
impl ::std::str::FromStr for ClassDeclarationKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "class" => Ok(Self::Class),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ClassDeclarationKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ClassDeclarationKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ClassDeclarationMembersItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ClassDeclarationMembersItem {
    Field(ClassField),
    Method(ClassMethod),
}
impl ::std::convert::From<ClassField> for ClassDeclarationMembersItem {
    fn from(value: ClassField) -> Self {
        Self::Field(value)
    }
}
impl ::std::convert::From<ClassMethod> for ClassDeclarationMembersItem {
    fn from(value: ClassMethod) -> Self {
        Self::Method(value)
    }
}
#[doc = "`ClassField`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ClassField {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<::std::string::String>,
    #[doc = "Whether the property is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub deprecated: ::std::option::Option<ClassFieldDeprecated>,
    #[doc = "A markdown description of the field."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "inheritedFrom",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub inherited_from: ::std::option::Option<Reference>,
    pub kind: ClassFieldKind,
    pub name: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub privacy: ::std::option::Option<Privacy>,
    #[doc = "Whether the property is read-only."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub readonly: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<SourceReference>,
    #[serde(
        rename = "static",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub static_: ::std::option::Option<bool>,
    #[doc = "A markdown summary suitable for display in a listing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<Type>,
}
#[doc = "Whether the property is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ClassFieldDeprecated {
    Boolean(bool),
    String(::std::string::String),
}
impl ::std::fmt::Display for ClassFieldDeprecated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for ClassFieldDeprecated {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
#[doc = "`ClassFieldKind`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ClassFieldKind {
    #[serde(rename = "field")]
    Field,
}
impl ::std::fmt::Display for ClassFieldKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Field => f.write_str("field"),
        }
    }
}
impl ::std::str::FromStr for ClassFieldKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "field" => Ok(Self::Field),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ClassFieldKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ClassFieldKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ClassMethod`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ClassMethod {
    #[doc = "Whether the function is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub deprecated: ::std::option::Option<ClassMethodDeprecated>,
    #[doc = "A markdown description."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "inheritedFrom",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub inherited_from: ::std::option::Option<Reference>,
    pub kind: ClassMethodKind,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub parameters: ::std::vec::Vec<Parameter>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub privacy: ::std::option::Option<Privacy>,
    #[serde(
        rename = "return",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub return_: ::std::option::Option<ClassMethodReturn>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<SourceReference>,
    #[serde(
        rename = "static",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub static_: ::std::option::Option<bool>,
    #[doc = "A markdown summary suitable for display in a listing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
}
#[doc = "Whether the function is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ClassMethodDeprecated {
    Boolean(bool),
    String(::std::string::String),
}
impl ::std::fmt::Display for ClassMethodDeprecated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for ClassMethodDeprecated {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
#[doc = "`ClassMethodKind`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ClassMethodKind {
    #[serde(rename = "method")]
    Method,
}
impl ::std::fmt::Display for ClassMethodKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Method => f.write_str("method"),
        }
    }
}
impl ::std::str::FromStr for ClassMethodKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "method" => Ok(Self::Method),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ClassMethodKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ClassMethodKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ClassMethodReturn`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ClassMethodReturn {
    #[doc = "A markdown description."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "A markdown summary suitable for display in a listing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<Type>,
}
#[doc = "`CssCustomProperty`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CssCustomProperty {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<::std::string::String>,
    #[doc = "Whether the CSS custom property is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub deprecated: ::std::option::Option<CssCustomPropertyDeprecated>,
    #[doc = "A markdown description."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the property, including leading `--`."]
    pub name: ::std::string::String,
    #[doc = "A markdown summary suitable for display in a listing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[doc = "The expected syntax of the defined property. Defaults to \"*\".\n\nThe syntax must be a valid CSS [syntax string](https://developer.mozilla.org/en-US/docs/Web/CSS/@property/syntax)\nas defined in the CSS Properties and Values API.\n\nExamples:\n\n\"<color>\": accepts a color\n\"<length> | <percentage>\": accepts lengths or percentages but not calc expressions with a combination of the two\n\"small | medium | large\": accepts one of these values set as custom idents.\n\"*\": any valid token"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub syntax: ::std::option::Option<::std::string::String>,
}
#[doc = "Whether the CSS custom property is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CssCustomPropertyDeprecated {
    Boolean(bool),
    String(::std::string::String),
}
impl ::std::fmt::Display for CssCustomPropertyDeprecated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for CssCustomPropertyDeprecated {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
#[doc = "The description of a CSS Custom State\nhttps://developer.mozilla.org/en-US/docs/Web/API/CustomStateSet"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CssCustomState {
    #[doc = "Whether the CSS custom state is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub deprecated: ::std::option::Option<CssCustomStateDeprecated>,
    #[doc = "A markdown description."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the state. Note: Unlike CSS custom properties, custom states\ndo not have a leading `--`."]
    pub name: ::std::string::String,
    #[doc = "A markdown summary suitable for display in a listing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
}
#[doc = "Whether the CSS custom state is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CssCustomStateDeprecated {
    Boolean(bool),
    String(::std::string::String),
}
impl ::std::fmt::Display for CssCustomStateDeprecated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for CssCustomStateDeprecated {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
#[doc = "The description of a CSS Part"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CssPart {
    #[doc = "Whether the CSS shadow part is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub deprecated: ::std::option::Option<CssPartDeprecated>,
    #[doc = "A markdown description."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    #[doc = "A markdown summary suitable for display in a listing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
}
#[doc = "Whether the CSS shadow part is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CssPartDeprecated {
    Boolean(bool),
    String(::std::string::String),
}
impl ::std::fmt::Display for CssPartDeprecated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for CssPartDeprecated {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
#[doc = "A description of a custom element class.\n\nCustom elements are JavaScript classes, so this extends from\n`ClassDeclaration` and adds custom-element-specific features like\nattributes, events, and slots.\n\nNote that `tagName` in this interface is optional. Tag names are not\nneccessarily part of a custom element class, but belong to the definition\n(often called the \"registration\") or the `customElements.define()` call.\n\nBecause classes and tag names can only be registered once, there's a\none-to-one relationship between classes and tag names. For ease of use,\nwe allow the tag name here.\n\nSome packages define and register custom elements in separate modules. In\nthese cases one `Module` should contain the `CustomElement` without a\ntagName, and another `Module` should contain the\n`CustomElementExport`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CustomElementDeclaration {
    #[doc = "The attributes that this element is known to understand."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub attributes: ::std::vec::Vec<Attribute>,
    #[serde(
        rename = "cssParts",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub css_parts: ::std::vec::Vec<CssPart>,
    #[serde(
        rename = "cssProperties",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub css_properties: ::std::vec::Vec<CssCustomProperty>,
    #[serde(
        rename = "cssStates",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub css_states: ::std::vec::Vec<CssCustomState>,
    #[doc = "Distinguishes a regular JavaScript class from a\ncustom element class"]
    #[serde(rename = "customElement")]
    pub custom_element: bool,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub demos: ::std::vec::Vec<Demo>,
    #[doc = "Whether the class or mixin is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub deprecated: ::std::option::Option<CustomElementDeclarationDeprecated>,
    #[doc = "A markdown description of the class."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The events that this element fires."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub events: ::std::vec::Vec<Event>,
    pub kind: CustomElementDeclarationKind,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub members: ::std::vec::Vec<CustomElementDeclarationMembersItem>,
    #[doc = "Any class mixins applied in the extends clause of this class.\n\nIf mixins are applied in the class definition, then the true superclass\nof this class is the result of applying mixins in order to the superclass.\n\nMixins must be listed in order of their application to the superclass or\nprevious mixin application. This means that the innermost mixin is listed\nfirst. This may read backwards from the common order in JavaScript, but\nmatches the order of language used to describe mixin application, like\n\"S with A, B\"."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub mixins: ::std::vec::Vec<Reference>,
    pub name: ::std::string::String,
    #[doc = "The shadow dom content slots that this element accepts."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub slots: ::std::vec::Vec<Slot>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<SourceReference>,
    #[doc = "A markdown summary suitable for display in a listing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[doc = "The superclass of this class.\n\nIf this class is defined with mixin applications, the prototype chain\nincludes the mixin applications and the true superclass is computed\nfrom them."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub superclass: ::std::option::Option<Reference>,
    #[doc = "An optional tag name that should be specified if this is a\nself-registering element.\n\nSelf-registering elements must also include a CustomElementExport\nin the module's exports."]
    #[serde(
        rename = "tagName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub tag_name: ::std::option::Option<::std::string::String>,
}
#[doc = "Whether the class or mixin is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CustomElementDeclarationDeprecated {
    Boolean(bool),
    String(::std::string::String),
}
impl ::std::fmt::Display for CustomElementDeclarationDeprecated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for CustomElementDeclarationDeprecated {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
#[doc = "`CustomElementDeclarationKind`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CustomElementDeclarationKind {
    #[serde(rename = "class")]
    Class,
}
impl ::std::fmt::Display for CustomElementDeclarationKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Class => f.write_str("class"),
        }
    }
}
impl ::std::str::FromStr for CustomElementDeclarationKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "class" => Ok(Self::Class),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CustomElementDeclarationKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomElementDeclarationKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CustomElementDeclarationMembersItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CustomElementDeclarationMembersItem {
    Field(ClassField),
    Method(ClassMethod),
}
impl ::std::convert::From<ClassField> for CustomElementDeclarationMembersItem {
    fn from(value: ClassField) -> Self {
        Self::Field(value)
    }
}
impl ::std::convert::From<ClassMethod> for CustomElementDeclarationMembersItem {
    fn from(value: ClassMethod) -> Self {
        Self::Method(value)
    }
}
#[doc = "A global custom element defintion, ie the result of a\n`customElements.define()` call.\n\nThis is represented as an export because a definition makes the element\navailable outside of the module it's defined it."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CustomElementExport {
    #[doc = "A reference to the class or other declaration that implements the\ncustom element."]
    pub declaration: Reference,
    #[doc = "Whether the custom-element export is deprecated.\nFor example, a future version will not register the custom element in this file.\nIf the value is a string, it's the reason for the deprecation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub deprecated: ::std::option::Option<CustomElementExportDeprecated>,
    pub kind: CustomElementExportKind,
    #[doc = "The tag name of the custom element."]
    pub name: ::std::string::String,
}
#[doc = "Whether the custom-element export is deprecated.\nFor example, a future version will not register the custom element in this file.\nIf the value is a string, it's the reason for the deprecation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CustomElementExportDeprecated {
    Boolean(bool),
    String(::std::string::String),
}
impl ::std::fmt::Display for CustomElementExportDeprecated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for CustomElementExportDeprecated {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
#[doc = "`CustomElementExportKind`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CustomElementExportKind {
    #[serde(rename = "custom-element-definition")]
    CustomElementDefinition,
}
impl ::std::fmt::Display for CustomElementExportKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CustomElementDefinition => f.write_str("custom-element-definition"),
        }
    }
}
impl ::std::str::FromStr for CustomElementExportKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "custom-element-definition" => Ok(Self::CustomElementDefinition),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CustomElementExportKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomElementExportKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A class mixin that also adds custom element related properties."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CustomElementMixinDeclaration {
    #[doc = "The attributes that this element is known to understand."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub attributes: ::std::vec::Vec<Attribute>,
    #[serde(
        rename = "cssParts",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub css_parts: ::std::vec::Vec<CssPart>,
    #[serde(
        rename = "cssProperties",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub css_properties: ::std::vec::Vec<CssCustomProperty>,
    #[serde(
        rename = "cssStates",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub css_states: ::std::vec::Vec<CssCustomState>,
    #[doc = "Distinguishes a regular JavaScript class from a\ncustom element class"]
    #[serde(rename = "customElement")]
    pub custom_element: bool,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub demos: ::std::vec::Vec<Demo>,
    #[doc = "Whether the class or mixin is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub deprecated: ::std::option::Option<CustomElementMixinDeclarationDeprecated>,
    #[doc = "A markdown description of the class."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The events that this element fires."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub events: ::std::vec::Vec<Event>,
    pub kind: CustomElementMixinDeclarationKind,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub members: ::std::vec::Vec<CustomElementMixinDeclarationMembersItem>,
    #[doc = "Any class mixins applied in the extends clause of this class.\n\nIf mixins are applied in the class definition, then the true superclass\nof this class is the result of applying mixins in order to the superclass.\n\nMixins must be listed in order of their application to the superclass or\nprevious mixin application. This means that the innermost mixin is listed\nfirst. This may read backwards from the common order in JavaScript, but\nmatches the order of language used to describe mixin application, like\n\"S with A, B\"."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub mixins: ::std::vec::Vec<Reference>,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub parameters: ::std::vec::Vec<Parameter>,
    #[serde(
        rename = "return",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub return_: ::std::option::Option<CustomElementMixinDeclarationReturn>,
    #[doc = "The shadow dom content slots that this element accepts."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub slots: ::std::vec::Vec<Slot>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<SourceReference>,
    #[doc = "A markdown summary suitable for display in a listing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[doc = "The superclass of this class.\n\nIf this class is defined with mixin applications, the prototype chain\nincludes the mixin applications and the true superclass is computed\nfrom them."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub superclass: ::std::option::Option<Reference>,
    #[doc = "An optional tag name that should be specified if this is a\nself-registering element.\n\nSelf-registering elements must also include a CustomElementExport\nin the module's exports."]
    #[serde(
        rename = "tagName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub tag_name: ::std::option::Option<::std::string::String>,
}
#[doc = "Whether the class or mixin is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CustomElementMixinDeclarationDeprecated {
    Boolean(bool),
    String(::std::string::String),
}
impl ::std::fmt::Display for CustomElementMixinDeclarationDeprecated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for CustomElementMixinDeclarationDeprecated {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
#[doc = "`CustomElementMixinDeclarationKind`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CustomElementMixinDeclarationKind {
    #[serde(rename = "mixin")]
    Mixin,
}
impl ::std::fmt::Display for CustomElementMixinDeclarationKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mixin => f.write_str("mixin"),
        }
    }
}
impl ::std::str::FromStr for CustomElementMixinDeclarationKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mixin" => Ok(Self::Mixin),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CustomElementMixinDeclarationKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomElementMixinDeclarationKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CustomElementMixinDeclarationMembersItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CustomElementMixinDeclarationMembersItem {
    Field(ClassField),
    Method(ClassMethod),
}
impl ::std::convert::From<ClassField> for CustomElementMixinDeclarationMembersItem {
    fn from(value: ClassField) -> Self {
        Self::Field(value)
    }
}
impl ::std::convert::From<ClassMethod> for CustomElementMixinDeclarationMembersItem {
    fn from(value: ClassMethod) -> Self {
        Self::Method(value)
    }
}
#[doc = "`CustomElementMixinDeclarationReturn`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct CustomElementMixinDeclarationReturn {
    #[doc = "A markdown description."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "A markdown summary suitable for display in a listing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<Type>,
}
#[doc = "`Demo`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Demo {
    #[doc = "A markdown description of the demo."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<SourceReference>,
    #[doc = "Relative URL of the demo if it's published with the package. Absolute URL\nif it's hosted."]
    pub url: ::std::string::String,
}
#[doc = "`Event`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Event {
    #[doc = "Whether the event is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub deprecated: ::std::option::Option<EventDeprecated>,
    #[doc = "A markdown description."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "inheritedFrom",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub inherited_from: ::std::option::Option<Reference>,
    pub name: ::std::string::String,
    #[doc = "A markdown summary suitable for display in a listing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[doc = "The type of the event object that's fired."]
    #[serde(rename = "type")]
    pub type_: Type,
}
#[doc = "Whether the event is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum EventDeprecated {
    Boolean(bool),
    String(::std::string::String),
}
impl ::std::fmt::Display for EventDeprecated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for EventDeprecated {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
#[doc = "`FunctionDeclaration`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FunctionDeclaration {
    #[doc = "Whether the function is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub deprecated: ::std::option::Option<FunctionDeclarationDeprecated>,
    #[doc = "A markdown description."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub kind: FunctionDeclarationKind,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub parameters: ::std::vec::Vec<Parameter>,
    #[serde(
        rename = "return",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub return_: ::std::option::Option<FunctionDeclarationReturn>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<SourceReference>,
    #[doc = "A markdown summary suitable for display in a listing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
}
#[doc = "Whether the function is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum FunctionDeclarationDeprecated {
    Boolean(bool),
    String(::std::string::String),
}
impl ::std::fmt::Display for FunctionDeclarationDeprecated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for FunctionDeclarationDeprecated {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
#[doc = "`FunctionDeclarationKind`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionDeclarationKind {
    #[serde(rename = "function")]
    Function,
}
impl ::std::fmt::Display for FunctionDeclarationKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Function => f.write_str("function"),
        }
    }
}
impl ::std::str::FromStr for FunctionDeclarationKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function" => Ok(Self::Function),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionDeclarationKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionDeclarationKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`FunctionDeclarationReturn`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct FunctionDeclarationReturn {
    #[doc = "A markdown description."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "A markdown summary suitable for display in a listing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<Type>,
}
#[doc = "`JavaScriptExport`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JavaScriptExport {
    #[doc = "A reference to the exported declaration.\n\nIn the case of aggregating exports, the reference's `module` field must be\ndefined and the `name` field must be `\"*\"`."]
    pub declaration: Reference,
    #[doc = "Whether the export is deprecated. For example, the name of the export was changed.\nIf the value is a string, it's the reason for the deprecation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub deprecated: ::std::option::Option<JavaScriptExportDeprecated>,
    pub kind: JavaScriptExportKind,
    #[doc = "The name of the exported symbol.\n\nJavaScript has a number of ways to export objects which determine the\ncorrect name to use.\n\n- Default exports must use the name \"default\".\n- Named exports use the name that is exported. If the export is renamed\n  with the \"as\" clause, use the exported name.\n- Aggregating exports (`* from`) should use the name `*`"]
    pub name: ::std::string::String,
}
#[doc = "Whether the export is deprecated. For example, the name of the export was changed.\nIf the value is a string, it's the reason for the deprecation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JavaScriptExportDeprecated {
    Boolean(bool),
    String(::std::string::String),
}
impl ::std::fmt::Display for JavaScriptExportDeprecated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for JavaScriptExportDeprecated {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
#[doc = "`JavaScriptExportKind`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JavaScriptExportKind {
    #[serde(rename = "js")]
    Js,
}
impl ::std::fmt::Display for JavaScriptExportKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Js => f.write_str("js"),
        }
    }
}
impl ::std::str::FromStr for JavaScriptExportKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "js" => Ok(Self::Js),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JavaScriptExportKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JavaScriptExportKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`JavaScriptModule`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JavaScriptModule {
    #[doc = "The declarations of a module.\n\nFor documentation purposes, all declarations that are reachable from\nexports should be described here. Ie, functions and objects that may be\nproperties of exported objects, or passed as arguments to functions."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub declarations: ::std::vec::Vec<JavaScriptModuleDeclarationsItem>,
    #[doc = "Whether the module is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub deprecated: ::std::option::Option<JavaScriptModuleDeprecated>,
    #[doc = "A markdown description of the module."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The exports of a module. This includes JavaScript exports and\ncustom element definitions."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub exports: ::std::vec::Vec<JavaScriptModuleExportsItem>,
    pub kind: JavaScriptModuleKind,
    #[doc = "Path to the javascript file needed to be imported.\n(not the path for example to a typescript file.)"]
    pub path: ::std::string::String,
    #[doc = "A markdown summary suitable for display in a listing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
}
#[doc = "`JavaScriptModuleDeclarationsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JavaScriptModuleDeclarationsItem {
    ClassDeclaration(ClassDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    MixinDeclaration(MixinDeclaration),
    VariableDeclaration(VariableDeclaration),
    CustomElementDeclaration(CustomElementDeclaration),
    CustomElementMixinDeclaration(CustomElementMixinDeclaration),
}
impl ::std::convert::From<ClassDeclaration> for JavaScriptModuleDeclarationsItem {
    fn from(value: ClassDeclaration) -> Self {
        Self::ClassDeclaration(value)
    }
}
impl ::std::convert::From<FunctionDeclaration> for JavaScriptModuleDeclarationsItem {
    fn from(value: FunctionDeclaration) -> Self {
        Self::FunctionDeclaration(value)
    }
}
impl ::std::convert::From<MixinDeclaration> for JavaScriptModuleDeclarationsItem {
    fn from(value: MixinDeclaration) -> Self {
        Self::MixinDeclaration(value)
    }
}
impl ::std::convert::From<VariableDeclaration> for JavaScriptModuleDeclarationsItem {
    fn from(value: VariableDeclaration) -> Self {
        Self::VariableDeclaration(value)
    }
}
impl ::std::convert::From<CustomElementDeclaration> for JavaScriptModuleDeclarationsItem {
    fn from(value: CustomElementDeclaration) -> Self {
        Self::CustomElementDeclaration(value)
    }
}
impl ::std::convert::From<CustomElementMixinDeclaration> for JavaScriptModuleDeclarationsItem {
    fn from(value: CustomElementMixinDeclaration) -> Self {
        Self::CustomElementMixinDeclaration(value)
    }
}
#[doc = "Whether the module is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JavaScriptModuleDeprecated {
    Boolean(bool),
    String(::std::string::String),
}
impl ::std::fmt::Display for JavaScriptModuleDeprecated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for JavaScriptModuleDeprecated {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
#[doc = "`JavaScriptModuleExportsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JavaScriptModuleExportsItem {
    JavaScriptExport(JavaScriptExport),
    CustomElementExport(CustomElementExport),
}
impl ::std::convert::From<JavaScriptExport> for JavaScriptModuleExportsItem {
    fn from(value: JavaScriptExport) -> Self {
        Self::JavaScriptExport(value)
    }
}
impl ::std::convert::From<CustomElementExport> for JavaScriptModuleExportsItem {
    fn from(value: CustomElementExport) -> Self {
        Self::CustomElementExport(value)
    }
}
#[doc = "`JavaScriptModuleKind`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JavaScriptModuleKind {
    #[serde(rename = "javascript-module")]
    JavascriptModule,
}
impl ::std::fmt::Display for JavaScriptModuleKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::JavascriptModule => f.write_str("javascript-module"),
        }
    }
}
impl ::std::str::FromStr for JavaScriptModuleKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "javascript-module" => Ok(Self::JavascriptModule),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JavaScriptModuleKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JavaScriptModuleKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A description of a class mixin.\n\nMixins are functions which generate a new subclass of a given superclass.\nThis interfaces describes the class and custom element features that\nare added by the mixin. As such, it extends the CustomElement interface and\nClassLike interface.\n\nSince mixins are functions, it also extends the FunctionLike interface. This\nmeans a mixin is callable, and has parameters and a return type.\n\nThe return type is often hard or impossible to accurately describe in type\nsystems like TypeScript. It requires generics and an `extends` operator\nthat TypeScript lacks. Therefore it's recommended that the return type is\nleft empty. The most common form of a mixin function takes a single\nargument, so consumers of this interface should assume that the return type\nis the single argument subclassed by this declaration.\n\nA mixin should not have a superclass. If a mixins composes other mixins,\nthey should be listed in the `mixins` field.\n\nSee [this article]{@link https://justinfagnani.com/2015/12/21/real-mixins-with-javascript-classes/}\nfor more information on the classmixin pattern in JavaScript."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MixinDeclaration {
    #[doc = "Whether the class or mixin is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub deprecated: ::std::option::Option<MixinDeclarationDeprecated>,
    #[doc = "A markdown description of the class."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub kind: MixinDeclarationKind,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub members: ::std::vec::Vec<MixinDeclarationMembersItem>,
    #[doc = "Any class mixins applied in the extends clause of this class.\n\nIf mixins are applied in the class definition, then the true superclass\nof this class is the result of applying mixins in order to the superclass.\n\nMixins must be listed in order of their application to the superclass or\nprevious mixin application. This means that the innermost mixin is listed\nfirst. This may read backwards from the common order in JavaScript, but\nmatches the order of language used to describe mixin application, like\n\"S with A, B\"."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub mixins: ::std::vec::Vec<Reference>,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub parameters: ::std::vec::Vec<Parameter>,
    #[serde(
        rename = "return",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub return_: ::std::option::Option<MixinDeclarationReturn>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<SourceReference>,
    #[doc = "A markdown summary suitable for display in a listing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[doc = "The superclass of this class.\n\nIf this class is defined with mixin applications, the prototype chain\nincludes the mixin applications and the true superclass is computed\nfrom them."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub superclass: ::std::option::Option<Reference>,
}
#[doc = "Whether the class or mixin is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum MixinDeclarationDeprecated {
    Boolean(bool),
    String(::std::string::String),
}
impl ::std::fmt::Display for MixinDeclarationDeprecated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for MixinDeclarationDeprecated {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
#[doc = "`MixinDeclarationKind`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MixinDeclarationKind {
    #[serde(rename = "mixin")]
    Mixin,
}
impl ::std::fmt::Display for MixinDeclarationKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mixin => f.write_str("mixin"),
        }
    }
}
impl ::std::str::FromStr for MixinDeclarationKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mixin" => Ok(Self::Mixin),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MixinDeclarationKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MixinDeclarationKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`MixinDeclarationMembersItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum MixinDeclarationMembersItem {
    Field(ClassField),
    Method(ClassMethod),
}
impl ::std::convert::From<ClassField> for MixinDeclarationMembersItem {
    fn from(value: ClassField) -> Self {
        Self::Field(value)
    }
}
impl ::std::convert::From<ClassMethod> for MixinDeclarationMembersItem {
    fn from(value: ClassMethod) -> Self {
        Self::Method(value)
    }
}
#[doc = "`MixinDeclarationReturn`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct MixinDeclarationReturn {
    #[doc = "A markdown description."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "A markdown summary suitable for display in a listing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<Type>,
}
#[doc = "`Parameter`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Parameter {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<::std::string::String>,
    #[doc = "Whether the property is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub deprecated: ::std::option::Option<ParameterDeprecated>,
    #[doc = "A markdown description of the field."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    #[doc = "Whether the parameter is optional. Undefined implies non-optional."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub optional: ::std::option::Option<bool>,
    #[doc = "Whether the property is read-only."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub readonly: ::std::option::Option<bool>,
    #[doc = "Whether the parameter is a rest parameter. Only the last parameter may be a rest parameter.\nUndefined implies single parameter."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub rest: ::std::option::Option<bool>,
    #[doc = "A markdown summary suitable for display in a listing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<Type>,
}
#[doc = "Whether the property is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ParameterDeprecated {
    Boolean(bool),
    String(::std::string::String),
}
impl ::std::fmt::Display for ParameterDeprecated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for ParameterDeprecated {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
#[doc = "`Privacy`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum Privacy {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "protected")]
    Protected,
    #[serde(rename = "public")]
    Public,
}
impl ::std::fmt::Display for Privacy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Private => f.write_str("private"),
            Self::Protected => f.write_str("protected"),
            Self::Public => f.write_str("public"),
        }
    }
}
impl ::std::str::FromStr for Privacy {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "private" => Ok(Self::Private),
            "protected" => Ok(Self::Protected),
            "public" => Ok(Self::Public),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Privacy {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Privacy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A reference to an export of a module.\n\nAll references are required to be publically accessible, so the canonical\nrepresentation of a reference is the export it's available from.\n\n`package` should generally refer to an npm package name. If `package` is\nundefined then the reference is local to this package. If `module` is\nundefined the reference is local to the containing module.\n\nReferences to global symbols like `Array`, `HTMLElement`, or `Event` should\nuse a `package` name of `\"global:\"`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Reference {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub module: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub package: ::std::option::Option<::std::string::String>,
}
#[doc = "`Slot`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Slot {
    #[doc = "Whether the slot is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub deprecated: ::std::option::Option<SlotDeprecated>,
    #[doc = "A markdown description."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The slot name, or the empty string for an unnamed slot."]
    pub name: ::std::string::String,
    #[doc = "A markdown summary suitable for display in a listing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
}
#[doc = "Whether the slot is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SlotDeprecated {
    Boolean(bool),
    String(::std::string::String),
}
impl ::std::fmt::Display for SlotDeprecated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for SlotDeprecated {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
#[doc = "A reference to the source of a declaration or member."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SourceReference {
    #[doc = "An absolute URL to the source (ie. a GitHub URL)."]
    pub href: ::std::string::String,
}
#[doc = "`Type`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Type {
    #[doc = "An array of references to the types in the type string.\n\nThese references have optional indices into the type string so that tools\ncan understand the references in the type string independently of the type\nsystem and syntax. For example, a documentation viewer could display the\ntype `Array<FooElement | BarElement>` with cross-references to `FooElement`\nand `BarElement` without understanding arrays, generics, or union types."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub references: ::std::vec::Vec<TypeReference>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<SourceReference>,
    #[doc = "The full string representation of the type, in whatever type syntax is\nused, such as JSDoc, Closure, or TypeScript."]
    pub text: ::std::string::String,
}
#[doc = "A reference that is associated with a type string and optionally a range\nwithin the string.\n\nStart and end must both be present or not present. If they're present, they\nare indices into the associated type string. If they are missing, the entire\ntype string is the symbol referenced and the name should match the type\nstring."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TypeReference {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub module: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub package: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start: ::std::option::Option<f64>,
}
#[doc = "`VariableDeclaration`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct VariableDeclaration {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<::std::string::String>,
    #[doc = "Whether the property is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub deprecated: ::std::option::Option<VariableDeclarationDeprecated>,
    #[doc = "A markdown description of the field."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub kind: VariableDeclarationKind,
    pub name: ::std::string::String,
    #[doc = "Whether the property is read-only."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub readonly: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<SourceReference>,
    #[doc = "A markdown summary suitable for display in a listing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<Type>,
}
#[doc = "Whether the property is deprecated.\nIf the value is a string, it's the reason for the deprecation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum VariableDeclarationDeprecated {
    Boolean(bool),
    String(::std::string::String),
}
impl ::std::fmt::Display for VariableDeclarationDeprecated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for VariableDeclarationDeprecated {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
#[doc = "`VariableDeclarationKind`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum VariableDeclarationKind {
    #[serde(rename = "variable")]
    Variable,
}
impl ::std::fmt::Display for VariableDeclarationKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Variable => f.write_str("variable"),
        }
    }
}
impl ::std::str::FromStr for VariableDeclarationKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "variable" => Ok(Self::Variable),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VariableDeclarationKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VariableDeclarationKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = " Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
