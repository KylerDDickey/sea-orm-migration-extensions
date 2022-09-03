use darling::FromMeta;

macro_rules! define_data_type {
    ($name:ident) => {
        #[cfg_attr(test, derive(Debug, PartialEq))]
        #[derive(Default, FromMeta)]
        pub struct $name;
    };
    ($name:ident { $($fieldname:ident : $type:ty),* }) => {
        #[cfg_attr(test, derive(Debug, PartialEq))]
        #[derive(Default, FromMeta)]
        pub struct $name {
            $(
                pub $fieldname: $type,
            )*
        }
    };
}

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(FromMeta)]
pub enum DataType {
    Char(Char),
    String(String),
    Text(Text),
    TinyInteger(TinyInteger),
    SmallInteger(SmallInteger),
    Integer(Integer),
    BigInteger(BigInteger),
    TinyUnsigned(TinyUnsigned),
    SmallUnsigned(SmallUnsigned),
    Unsigned(Unsigned),
    BigUnsigned(BigUnsigned),
    Float(Float),
    Double(Double),

    Timestamp(Timestamp),
    TimestampWithTimeZone(TimestampWithTimeZone),
    Time(Time),
    Date(Date),


    VarBinary(VarBinary),
    Boolean(Boolean),

    Json(Json),
    JsonBinary(JsonBinary),

    Uuid(Uuid),

    Array(Array),
    Cidr(Cidr),
    Inet(Inet),
    MacAddr(MacAddr),
}

define_data_type!(Char { len: Option<u32> });
define_data_type!(String { len: Option<u32> });
define_data_type!(Text);
define_data_type!(TinyInteger { len: Option<u32> });
define_data_type!(SmallInteger { len: Option<u32> });
define_data_type!(Integer { len: Option<u32> });
define_data_type!(BigInteger { len: Option<u32> });
define_data_type!(TinyUnsigned { len: Option<u32> });
define_data_type!(SmallUnsigned { len: Option<u32> });
define_data_type!(Unsigned { len: Option<u32> });
define_data_type!(BigUnsigned { len: Option<u32> });
define_data_type!(Float { len: Option<u32> });
define_data_type!(Double { len: Option<u32> });
// define_data_type!(Decimal { len: Option<SizePrecision> });
define_data_type!(Timestamp { len: Option<u32> });
define_data_type!(TimestampWithTimeZone { len: Option<u32> });
define_data_type!(Time { len: Option<u32> });
define_data_type!(Date);
// define_data_type!(Interval { fields: Option<PgInterval>, precision: Option<u32> });
// define_data_type!(Binary { size: BlobSize });
define_data_type!(VarBinary { len: u32 });
define_data_type!(Boolean);
// define_data_type!(Money { len: Option<SizePrecision> });
define_data_type!(Json);
define_data_type!(JsonBinary);
// define_data_type!(Custom { typing: Arc<dyn Iden + 'static> });
define_data_type!(Uuid);
// define_data_type!(Enum { name: String, variants: Vec<String> });
define_data_type!(Array { elem_type: Option<std::string::String> });
define_data_type!(Cidr);
define_data_type!(Inet);
define_data_type!(MacAddr);
