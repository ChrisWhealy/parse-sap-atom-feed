## Parsing Decimal Values


### `Edm.Decimal` Properties in the Metadata XML

When parsing the XML metadata of an SAP V2 OData service, the code in crate `parse-sap-odata` will probably encounter `<Property>` declarations of type `Edm.Decimal`.
For example:

```xml
<Property Name="Depth" Type="Edm.Decimal" Precision="13" Scale="3" sap:unicode="false" sap:unit="DimUnit" sap:label="Dimensions"/>
```

This declaration says that a field called `Depth` exists that stores a decimal value in 13 digits (`Precision="13"`), that last three of which are to the right of the decimal separator (`Scale="3"`).

### Transforming `Edm.Decimal` to `rust_decimal::Decimal`

In a `rust_decimal::Decimal` value however, the maximum number of digits is hard-coded to 64.  Consequently, when transforming an `Edm.Decimal` value, we can make use of the `Scale` attribute, but the `Precision` attribute has no meaning.

### `Edm.Decimal` Custom Deserializer

The above XML declaration will generate the following field declaration in the appropriate entity type `struct`:

```rust
#[serde(
    deserialize_with = "parse_sap_atom_feed::deserializers::edm_decimal::to_rust_decimal_3dp_opt"
)]
pub depth: Option<rust_decimal::Decimal>,
```

There are several points to notice here:

1. Since the `<Property>` element does not contain the attribute `Nullable="false"`, the decimal value might be absent; therefore it must be declared inside an `Option`.
1. The scale of a `rust_decimal::Decimal` value is defined at the time a new value is created.
    Consequently, the field declaration refers to a generic decimal value, without any reference to scale.
1. Since the `Scale` attribute is part of an `Edm.Decimal` type declaration, this value can only be preserved by implementing a different serializer function for each of the possible scale values.
    Hence the serializer function name contains both the number of decimal places it will use (`3dp` in this case) and whether the value is wrapped in an `Option`.

### The Custom Deserializer Can Fail and Might Even Panic!

#### Scale Too Large

If the value of `Scale` is greater than 28, then function `rust_decimal::Decimal::try_new()` used by the custom deserializer will fail.

#### Too Many Fractional Digits

Consider the XML declaration of a `Property` called `Balance`:

```xml
<Property Name="Balance" Type="Edm.Decimal" Precision="16" Scale="5" sap:unicode="false" sap:unit="CurrencyCode" sap:label="Account Balance"/>
```

Knowing that the `Precision` value has no meaning and that `Scale` equals `5`, the deserializer will expect a decimal string value containing up to 5 fractional digits.
For example, all of these string values will be parsed successfully:

| XML String | Becomes the Rust Declaration     | Is Printed As |
|------------|----------------------------------|---------------:|
| `1234`     | `Decimal::try_new(123400000, 5)` |   `1234.00000` |
| `123.4`    | `Decimal::try_new(12340000, 5)`  |    `123.40000` |
| `12.34`    | `Decimal::try_new(1234000, 5)`   |     `12.34000` |
| `1.234`    | `Decimal::try_new(123400, 5)`    |      `1.23400` |
| `.1234`    | `Decimal::try_new(12340, 5)`     |      `0.12340` |
| `.01234`   | `Decimal::try_new(1234, 5)`      |      `0.01234` |

But attempting to parse a string containing 6 decimal places when `scale = 5` (E.G. `123.456789`) will cause data loss and the deserializer will panic with the message:

`Data loss: Edm.Decimal value 123.456789 contains too many fractional digits. Expected 5 but got 6`

However, if teh fraction contains trailing zeroes, these will first be trimmed before deciding whether to panic.

E.G. With `scale = 5`, attempting to deserialize `123.4567800` will succeed because trimming the trailing zeroes does not lead to data loss, but attempting to deserialize `123.456789` will cause a panic.
