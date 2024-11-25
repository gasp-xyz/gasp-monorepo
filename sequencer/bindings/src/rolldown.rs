///Module containing a contract's types and functions.
/**

```solidity
library IRolldownPrimitives {
    type ChainId is uint8;
    type Origin is uint8;
    struct Cancel { RequestId requestId; Range range; bytes32 hash; }
    struct CancelResolution { RequestId requestId; uint256 l2RequestId; bool cancelJustified; uint256 timeStamp; }
    struct Deposit { RequestId requestId; address depositRecipient; address tokenAddress; uint256 amount; uint256 timeStamp; uint256 ferryTip; }
    struct FailedDepositResolution { RequestId requestId; uint256 originRequestId; address ferry; }
    struct L1Update { ChainId chain; Deposit[] pendingDeposits; CancelResolution[] pendingCancelResolutions; }
    struct Range { uint256 start; uint256 end; }
    struct RequestId { Origin origin; uint256 id; }
    struct Withdrawal { RequestId requestId; address recipient; address tokenAddress; uint256 amount; uint256 ferryTip; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IRolldownPrimitives {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ChainId(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<ChainId> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::Token<'_>
            {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(self).0
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::abi_encoded_size(
                    self,
                )
            }
        }
        #[automatically_derived]
        impl ChainId {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for ChainId {
            type RustType = u8;
            type Token<'a> =
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> =
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ChainId {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::EventTopic>::encode_topic(
                    rust,
                )
            }
        }
    };
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Origin(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Origin> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::Token<'_>
            {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(self).0
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::abi_encoded_size(
                    self,
                )
            }
        }
        #[automatically_derived]
        impl Origin {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Origin {
            type RustType = u8;
            type Token<'a> =
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> =
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Origin {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::EventTopic>::encode_topic(
                    rust,
                )
            }
        }
    };
    /**```solidity
    struct Cancel { RequestId requestId; Range range; bytes32 hash; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Cancel {
        pub requestId: <RequestId as alloy::sol_types::SolType>::RustType,
        pub range: <Range as alloy::sol_types::SolType>::RustType,
        pub hash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> =
            (RequestId, Range, alloy::sol_types::sol_data::FixedBytes<32>);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <RequestId as alloy::sol_types::SolType>::RustType,
            <Range as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::FixedBytes<32>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Cancel> for UnderlyingRustTuple<'_> {
            fn from(value: Cancel) -> Self {
                (value.requestId, value.range, value.hash)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Cancel {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    requestId: tuple.0,
                    range: tuple.1,
                    hash: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Cancel {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Cancel {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <RequestId as alloy_sol_types::SolType>::tokenize(&self.requestId),
                    <Range as alloy_sol_types::SolType>::tokenize(&self.range),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.hash),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Cancel {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Cancel {
            const NAME: &'static str = "Cancel";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Cancel(RequestId requestId,Range range,bytes32 hash)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components.push(<RequestId as alloy_sol_types::SolStruct>::eip712_root_type());
                components.extend(<RequestId as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<Range as alloy_sol_types::SolStruct>::eip712_root_type());
                components.extend(<Range as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <RequestId as alloy_sol_types::SolType>::eip712_data_word(
                            &self.requestId,
                        )
                        .0,
                    <Range as alloy_sol_types::SolType>::eip712_data_word(&self.range).0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.hash)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Cancel {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <RequestId as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.requestId,
                    )
                    + <Range as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.range,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.hash)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <RequestId as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.requestId,
                    out,
                );
                <Range as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.range, out);
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.hash,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    /**```solidity
    struct CancelResolution { RequestId requestId; uint256 l2RequestId; bool cancelJustified; uint256 timeStamp; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CancelResolution {
        pub requestId: <RequestId as alloy::sol_types::SolType>::RustType,
        pub l2RequestId: alloy::sol_types::private::primitives::aliases::U256,
        pub cancelJustified: bool,
        pub timeStamp: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            RequestId,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Bool,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <RequestId as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::primitives::aliases::U256,
            bool,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<CancelResolution> for UnderlyingRustTuple<'_> {
            fn from(value: CancelResolution) -> Self {
                (
                    value.requestId,
                    value.l2RequestId,
                    value.cancelJustified,
                    value.timeStamp,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CancelResolution {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    requestId: tuple.0,
                    l2RequestId: tuple.1,
                    cancelJustified: tuple.2,
                    timeStamp: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for CancelResolution {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for CancelResolution {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <RequestId as alloy_sol_types::SolType>::tokenize(&self.requestId),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.l2RequestId,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.cancelJustified,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.timeStamp,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for CancelResolution {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for CancelResolution {
            const NAME: &'static str = "CancelResolution";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "CancelResolution(RequestId requestId,uint256 l2RequestId,bool cancelJustified,uint256 timeStamp)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components.push(<RequestId as alloy_sol_types::SolStruct>::eip712_root_type());
                components.extend(<RequestId as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <RequestId as alloy_sol_types::SolType>::eip712_data_word(
                            &self.requestId,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.l2RequestId)
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.cancelJustified,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.timeStamp)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for CancelResolution {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <RequestId as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.requestId,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.l2RequestId,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.cancelJustified,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.timeStamp,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <RequestId as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.requestId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.l2RequestId,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.cancelJustified,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.timeStamp,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    /**```solidity
    struct Deposit { RequestId requestId; address depositRecipient; address tokenAddress; uint256 amount; uint256 timeStamp; uint256 ferryTip; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Deposit {
        pub requestId: <RequestId as alloy::sol_types::SolType>::RustType,
        pub depositRecipient: alloy::sol_types::private::Address,
        pub tokenAddress: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub timeStamp: alloy::sol_types::private::primitives::aliases::U256,
        pub ferryTip: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            RequestId,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <RequestId as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Deposit> for UnderlyingRustTuple<'_> {
            fn from(value: Deposit) -> Self {
                (
                    value.requestId,
                    value.depositRecipient,
                    value.tokenAddress,
                    value.amount,
                    value.timeStamp,
                    value.ferryTip,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Deposit {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    requestId: tuple.0,
                    depositRecipient: tuple.1,
                    tokenAddress: tuple.2,
                    amount: tuple.3,
                    timeStamp: tuple.4,
                    ferryTip: tuple.5,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Deposit {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Deposit {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <RequestId as alloy_sol_types::SolType>::tokenize(&self.requestId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.depositRecipient,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.amount,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.timeStamp,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.ferryTip,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Deposit {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Deposit {
            const NAME: &'static str = "Deposit";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Deposit(RequestId requestId,address depositRecipient,address tokenAddress,uint256 amount,uint256 timeStamp,uint256 ferryTip)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components.push(<RequestId as alloy_sol_types::SolStruct>::eip712_root_type());
                components.extend(<RequestId as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <RequestId as alloy_sol_types::SolType>::eip712_data_word(
                            &self.requestId,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.depositRecipient,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.tokenAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.timeStamp)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.ferryTip)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Deposit {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <RequestId as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.requestId,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.depositRecipient,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tokenAddress,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.timeStamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ferryTip,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <RequestId as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.requestId,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.depositRecipient,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tokenAddress,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.timeStamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ferryTip,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    /**```solidity
    struct FailedDepositResolution { RequestId requestId; uint256 originRequestId; address ferry; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FailedDepositResolution {
        pub requestId: <RequestId as alloy::sol_types::SolType>::RustType,
        pub originRequestId: alloy::sol_types::private::primitives::aliases::U256,
        pub ferry: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            RequestId,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <RequestId as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<FailedDepositResolution> for UnderlyingRustTuple<'_> {
            fn from(value: FailedDepositResolution) -> Self {
                (value.requestId, value.originRequestId, value.ferry)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FailedDepositResolution {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    requestId: tuple.0,
                    originRequestId: tuple.1,
                    ferry: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FailedDepositResolution {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FailedDepositResolution {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <RequestId as alloy_sol_types::SolType>::tokenize(&self.requestId),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.originRequestId,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.ferry,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for FailedDepositResolution {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for FailedDepositResolution {
            const NAME: &'static str = "FailedDepositResolution";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FailedDepositResolution(RequestId requestId,uint256 originRequestId,address ferry)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components.push(<RequestId as alloy_sol_types::SolStruct>::eip712_root_type());
                components.extend(<RequestId as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <RequestId as alloy_sol_types::SolType>::eip712_data_word(
                            &self.requestId,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.originRequestId,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.ferry,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FailedDepositResolution {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <RequestId as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.requestId,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.originRequestId,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ferry,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <RequestId as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.requestId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.originRequestId,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ferry,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    /**```solidity
    struct L1Update { ChainId chain; Deposit[] pendingDeposits; CancelResolution[] pendingCancelResolutions; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct L1Update {
        pub chain: <ChainId as alloy::sol_types::SolType>::RustType,
        pub pendingDeposits:
            alloy::sol_types::private::Vec<<Deposit as alloy::sol_types::SolType>::RustType>,
        pub pendingCancelResolutions: alloy::sol_types::private::Vec<
            <CancelResolution as alloy::sol_types::SolType>::RustType,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            ChainId,
            alloy::sol_types::sol_data::Array<Deposit>,
            alloy::sol_types::sol_data::Array<CancelResolution>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <ChainId as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Vec<<Deposit as alloy::sol_types::SolType>::RustType>,
            alloy::sol_types::private::Vec<
                <CancelResolution as alloy::sol_types::SolType>::RustType,
            >,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<L1Update> for UnderlyingRustTuple<'_> {
            fn from(value: L1Update) -> Self {
                (
                    value.chain,
                    value.pendingDeposits,
                    value.pendingCancelResolutions,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for L1Update {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    chain: tuple.0,
                    pendingDeposits: tuple.1,
                    pendingCancelResolutions: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for L1Update {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for L1Update {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <ChainId as alloy_sol_types::SolType>::tokenize(&self.chain),
                    <alloy::sol_types::sol_data::Array<
                        Deposit,
                    > as alloy_sol_types::SolType>::tokenize(&self.pendingDeposits),
                    <alloy::sol_types::sol_data::Array<
                        CancelResolution,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.pendingCancelResolutions,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for L1Update {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for L1Update {
            const NAME: &'static str = "L1Update";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "L1Update(uint8 chain,Deposit[] pendingDeposits,CancelResolution[] pendingCancelResolutions)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components.push(<Deposit as alloy_sol_types::SolStruct>::eip712_root_type());
                components.extend(<Deposit as alloy_sol_types::SolStruct>::eip712_components());
                components
                    .push(<CancelResolution as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<CancelResolution as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <ChainId as alloy_sol_types::SolType>::eip712_data_word(&self.chain)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        Deposit,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.pendingDeposits,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        CancelResolution,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.pendingCancelResolutions,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for L1Update {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <ChainId as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.chain,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        Deposit,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.pendingDeposits,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        CancelResolution,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.pendingCancelResolutions,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <ChainId as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.chain, out);
                <alloy::sol_types::sol_data::Array<
                    Deposit,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.pendingDeposits,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    CancelResolution,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.pendingCancelResolutions,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    /**```solidity
    struct Range { uint256 start; uint256 end; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Range {
        pub start: alloy::sol_types::private::primitives::aliases::U256,
        pub end: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Range> for UnderlyingRustTuple<'_> {
            fn from(value: Range) -> Self {
                (value.start, value.end)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Range {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    start: tuple.0,
                    end: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Range {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Range {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.start,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.end,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Range {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Range {
            const NAME: &'static str = "Range";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("Range(uint256 start,uint256 end)")
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.start)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.end)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Range {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.start)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.end)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.start,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.end, out);
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    /**```solidity
    struct RequestId { Origin origin; uint256 id; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RequestId {
        pub origin: <Origin as alloy::sol_types::SolType>::RustType,
        pub id: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (Origin, alloy::sol_types::sol_data::Uint<256>);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <Origin as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<RequestId> for UnderlyingRustTuple<'_> {
            fn from(value: RequestId) -> Self {
                (value.origin, value.id)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RequestId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    origin: tuple.0,
                    id: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RequestId {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RequestId {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <Origin as alloy_sol_types::SolType>::tokenize(&self.origin),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.id,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for RequestId {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for RequestId {
            const NAME: &'static str = "RequestId";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("RequestId(uint8 origin,uint256 id)")
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <Origin as alloy_sol_types::SolType>::eip712_data_word(&self.origin)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.id)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RequestId {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <Origin as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.origin,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.id)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <Origin as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.origin, out);
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.id, out);
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    /**```solidity
    struct Withdrawal { RequestId requestId; address recipient; address tokenAddress; uint256 amount; uint256 ferryTip; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Withdrawal {
        pub requestId: <RequestId as alloy::sol_types::SolType>::RustType,
        pub recipient: alloy::sol_types::private::Address,
        pub tokenAddress: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub ferryTip: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            RequestId,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <RequestId as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Withdrawal> for UnderlyingRustTuple<'_> {
            fn from(value: Withdrawal) -> Self {
                (
                    value.requestId,
                    value.recipient,
                    value.tokenAddress,
                    value.amount,
                    value.ferryTip,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Withdrawal {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    requestId: tuple.0,
                    recipient: tuple.1,
                    tokenAddress: tuple.2,
                    amount: tuple.3,
                    ferryTip: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Withdrawal {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Withdrawal {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <RequestId as alloy_sol_types::SolType>::tokenize(&self.requestId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.recipient,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.amount,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.ferryTip,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Withdrawal {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Withdrawal {
            const NAME: &'static str = "Withdrawal";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Withdrawal(RequestId requestId,address recipient,address tokenAddress,uint256 amount,uint256 ferryTip)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components.push(<RequestId as alloy_sol_types::SolStruct>::eip712_root_type());
                components.extend(<RequestId as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <RequestId as alloy_sol_types::SolType>::eip712_data_word(
                            &self.requestId,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.recipient,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.tokenAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.ferryTip)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Withdrawal {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <RequestId as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.requestId,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.recipient,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tokenAddress,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ferryTip,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <RequestId as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.requestId,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.recipient,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tokenAddress,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ferryTip,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IRolldownPrimitives`](self) contract instance.

    See the [wrapper's documentation](`IRolldownPrimitivesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IRolldownPrimitivesInstance<T, P, N> {
        IRolldownPrimitivesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IRolldownPrimitives`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IRolldownPrimitives`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IRolldownPrimitivesInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IRolldownPrimitivesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IRolldownPrimitivesInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > IRolldownPrimitivesInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IRolldownPrimitives`](self) contract instance.

        See the [wrapper's documentation](`IRolldownPrimitivesInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> IRolldownPrimitivesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IRolldownPrimitivesInstance<T, P, N> {
            IRolldownPrimitivesInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > IRolldownPrimitivesInstance<T, P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > IRolldownPrimitivesInstance<T, P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library IRolldownPrimitives {
    type ChainId is uint8;
    type Origin is uint8;
    struct Cancel {
        RequestId requestId;
        Range range;
        bytes32 hash;
    }
    struct CancelResolution {
        RequestId requestId;
        uint256 l2RequestId;
        bool cancelJustified;
        uint256 timeStamp;
    }
    struct Deposit {
        RequestId requestId;
        address depositRecipient;
        address tokenAddress;
        uint256 amount;
        uint256 timeStamp;
        uint256 ferryTip;
    }
    struct FailedDepositResolution {
        RequestId requestId;
        uint256 originRequestId;
        address ferry;
    }
    struct L1Update {
        ChainId chain;
        Deposit[] pendingDeposits;
        CancelResolution[] pendingCancelResolutions;
    }
    struct Range {
        uint256 start;
        uint256 end;
    }
    struct RequestId {
        Origin origin;
        uint256 id;
    }
    struct Withdrawal {
        RequestId requestId;
        address recipient;
        address tokenAddress;
        uint256 amount;
        uint256 ferryTip;
    }
}

interface Rolldown {
    error FerryTipExceedsAmount(uint256 ferryTip, uint256 amount);
    error InvalidFerriedAmount(uint256 actualAmount, uint256 expectedAmount);
    error InvalidRequestId(uint256 requestId);
    error InvalidRequestProof(bytes32 merkleRoot);
    error InvalidRequestRange(uint256 start, uint256 end);
    error InvalidUpdateRange(uint256 start, uint256 end);
    error L2RequestAlreadyProcessed(bytes32 requestHash);
    error PreviousUpdateMissed(uint256 currentStartRange, uint256 lastProcessedUpdate);
    error RequestOutOfRange(uint256 requestId, uint256 start, uint256 end);
    error RequestRangeTooLarge(uint256 count);
    error UnexpectedMerkleRoot();
    error UpdateAlreadyApplied(uint256 currentEndRange, uint256 lastProcessedUpdate);
    error WithdrawalAlreadyFerried(bytes32 withdrawalHash);
    error ZeroAdmin();
    error ZeroAmount();
    error ZeroRecipient();
    error ZeroRootCount();
    error ZeroToken();
    error ZeroTransferAmount();
    error ZeroUpdateRange();
    error ZeroUpdater();

    event DepositAcceptedIntoQueue(uint256 indexed requestId, address indexed depositRecipient, address indexed tokenAddress, uint256 amount, uint256 ferryTip);
    event DisputeResolutionAcceptedIntoQueue(uint256 indexed requestId, bool cancelJustified, bytes32 cancelResolutionHash);
    event ERC20TokensWithdrawn(address indexed sender, address indexed tokenAddress, uint256 amount);
    event FailedDepositResolutionClosed(uint256 indexedrequestId, uint256 originDepositId, bytes32 failedDespotiResolutionHash);
    event FerriedWithdrawalClosed(uint256 indexed requestId, bytes32 withdrawalHash);
    event Initialized(uint8 version);
    event L2UpdateAccepted(bytes32 root, IRolldownPrimitives.Range range);
    event NativeTokensWithdrawn(address indexed sender, uint256 amount);
    event NewUpdaterSet(address indexed updater);
    event Paused(address account);
    event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole);
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
    event Unpaused(address account);
    event WithdrawalClosed(uint256 indexed requestId, bytes32 withdrawalHash);
    event WithdrawalFerried(uint256 indexedrequestId, uint256 amount, address indexed recipient, address indexed ferry, bytes32 withdrawalHash);

    function CLOSED() external view returns (address);
    function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
    function NATIVE_TOKEN_ADDRESS() external view returns (address);
    function UPDATER_ROLE() external view returns (bytes32);
    function cancelResolutions(uint256) external view returns (IRolldownPrimitives.RequestId memory requestId, uint256 l2RequestId, bool cancelJustified, uint256 timeStamp);
    function chain() external view returns (IRolldownPrimitives.ChainId);
    function closeCancel(IRolldownPrimitives.Cancel memory cancel, bytes32 merkleRoot, bytes32[] memory proof) external;
    function closeDepositRefund(IRolldownPrimitives.FailedDepositResolution memory failedDeposit, bytes32 merkleRoot, bytes32[] memory proof) external;
    function closeWithdrawal(IRolldownPrimitives.Withdrawal memory withdrawal, bytes32 merkleRoot, bytes32[] memory proof) external;
    function close_cancel(IRolldownPrimitives.Cancel memory cancel, bytes32 merkleRoot, bytes32[] memory proof) external;
    function close_deposit_refund(IRolldownPrimitives.FailedDepositResolution memory failedDeposit, bytes32 merkleRoot, bytes32[] memory proof) external;
    function close_withdrawal(IRolldownPrimitives.Withdrawal memory withdrawal, bytes32 merkleRoot, bytes32[] memory proof) external;
    function counter() external view returns (uint256);
    function deposit(address tokenAddress, uint256 amount, uint256 ferryTip) external;
    function deposit(address tokenAddress, uint256 amount) external;
    function depositERC20(address tokenAddress, uint256 amount, uint256 ferryTip) external;
    function depositERC20(address tokenAddress, uint256 amount) external;
    function depositNative(uint256 ferryTip) external payable;
    function depositNative() external payable;
    function deposit_erc20(address tokenAddress, uint256 amount, uint256 ferryTip) external;
    function deposit_erc20(address tokenAddress, uint256 amount) external;
    function deposit_native() external payable;
    function deposit_native(uint256 ferryTip) external payable;
    function deposits(uint256) external view returns (IRolldownPrimitives.RequestId memory requestId, address depositRecipient, address tokenAddress, uint256 amount, uint256 timeStamp, uint256 ferryTip);
    function ferryWithdrawal(IRolldownPrimitives.Withdrawal memory withdrawal) external payable;
    function ferry_withdrawal(IRolldownPrimitives.Withdrawal memory withdrawal) external payable;
    function findL2Batch(uint256 requestId) external view returns (bytes32);
    function find_l2_batch(uint256 requestId) external view returns (bytes32);
    function getMerkleRootsLength() external view returns (uint256);
    function getPendingRequests(uint256 start, uint256 end) external view returns (IRolldownPrimitives.L1Update memory);
    function getRoleAdmin(bytes32 role) external view returns (bytes32);
    function getUpdateForL2() external view returns (IRolldownPrimitives.L1Update memory);
    function grantRole(bytes32 role, address account) external;
    function hasRole(bytes32 role, address account) external view returns (bool);
    function hashCancel(IRolldownPrimitives.Cancel memory cancel) external pure returns (bytes32);
    function hashFailedDepositResolution(IRolldownPrimitives.FailedDepositResolution memory failedDeposit) external pure returns (bytes32);
    function hashWithdrawal(IRolldownPrimitives.Withdrawal memory withdrawal) external pure returns (bytes32);
    function initialize(address admin, IRolldownPrimitives.ChainId chainId, address updater) external;
    function lastProcessedUpdate_origin_l1() external view returns (uint256);
    function lastProcessedUpdate_origin_l2() external view returns (uint256);
    function merkleRootRange(bytes32) external view returns (uint256 start, uint256 end);
    function pause() external;
    function paused() external view returns (bool);
    function processedL2Requests(bytes32) external view returns (address);
    function renounceRole(bytes32 role, address account) external;
    function revokeRole(bytes32 role, address account) external;
    function roots(uint256) external view returns (bytes32);
    function setUpdater(address updater) external;
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
    function unpause() external;
    function updateL1FromL2(bytes32 merkleRoot, IRolldownPrimitives.Range memory range) external;
    function update_l1_from_l2(bytes32 merkleRoot, IRolldownPrimitives.Range memory range) external;
    function updaterAccount() external view returns (address);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "CLOSED",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "DEFAULT_ADMIN_ROLE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "NATIVE_TOKEN_ADDRESS",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "UPDATER_ROLE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "cancelResolutions",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "requestId",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.RequestId",
        "components": [
          {
            "name": "origin",
            "type": "uint8",
            "internalType": "enum IRolldownPrimitives.Origin"
          },
          {
            "name": "id",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "l2RequestId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "cancelJustified",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "timeStamp",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "chain",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "enum IRolldownPrimitives.ChainId"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "closeCancel",
    "inputs": [
      {
        "name": "cancel",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.Cancel",
        "components": [
          {
            "name": "requestId",
            "type": "tuple",
            "internalType": "struct IRolldownPrimitives.RequestId",
            "components": [
              {
                "name": "origin",
                "type": "uint8",
                "internalType": "enum IRolldownPrimitives.Origin"
              },
              {
                "name": "id",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "range",
            "type": "tuple",
            "internalType": "struct IRolldownPrimitives.Range",
            "components": [
              {
                "name": "start",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "end",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "hash",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "merkleRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "proof",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "closeDepositRefund",
    "inputs": [
      {
        "name": "failedDeposit",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.FailedDepositResolution",
        "components": [
          {
            "name": "requestId",
            "type": "tuple",
            "internalType": "struct IRolldownPrimitives.RequestId",
            "components": [
              {
                "name": "origin",
                "type": "uint8",
                "internalType": "enum IRolldownPrimitives.Origin"
              },
              {
                "name": "id",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "originRequestId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "ferry",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "merkleRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "proof",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "closeWithdrawal",
    "inputs": [
      {
        "name": "withdrawal",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.Withdrawal",
        "components": [
          {
            "name": "requestId",
            "type": "tuple",
            "internalType": "struct IRolldownPrimitives.RequestId",
            "components": [
              {
                "name": "origin",
                "type": "uint8",
                "internalType": "enum IRolldownPrimitives.Origin"
              },
              {
                "name": "id",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "recipient",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "tokenAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "amount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "ferryTip",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "merkleRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "proof",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "close_cancel",
    "inputs": [
      {
        "name": "cancel",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.Cancel",
        "components": [
          {
            "name": "requestId",
            "type": "tuple",
            "internalType": "struct IRolldownPrimitives.RequestId",
            "components": [
              {
                "name": "origin",
                "type": "uint8",
                "internalType": "enum IRolldownPrimitives.Origin"
              },
              {
                "name": "id",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "range",
            "type": "tuple",
            "internalType": "struct IRolldownPrimitives.Range",
            "components": [
              {
                "name": "start",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "end",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "hash",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "merkleRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "proof",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "close_deposit_refund",
    "inputs": [
      {
        "name": "failedDeposit",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.FailedDepositResolution",
        "components": [
          {
            "name": "requestId",
            "type": "tuple",
            "internalType": "struct IRolldownPrimitives.RequestId",
            "components": [
              {
                "name": "origin",
                "type": "uint8",
                "internalType": "enum IRolldownPrimitives.Origin"
              },
              {
                "name": "id",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "originRequestId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "ferry",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "merkleRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "proof",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "close_withdrawal",
    "inputs": [
      {
        "name": "withdrawal",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.Withdrawal",
        "components": [
          {
            "name": "requestId",
            "type": "tuple",
            "internalType": "struct IRolldownPrimitives.RequestId",
            "components": [
              {
                "name": "origin",
                "type": "uint8",
                "internalType": "enum IRolldownPrimitives.Origin"
              },
              {
                "name": "id",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "recipient",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "tokenAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "amount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "ferryTip",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "merkleRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "proof",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "counter",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "deposit",
    "inputs": [
      {
        "name": "tokenAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "ferryTip",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deposit",
    "inputs": [
      {
        "name": "tokenAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "depositERC20",
    "inputs": [
      {
        "name": "tokenAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "ferryTip",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "depositERC20",
    "inputs": [
      {
        "name": "tokenAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "depositNative",
    "inputs": [
      {
        "name": "ferryTip",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "depositNative",
    "inputs": [],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "deposit_erc20",
    "inputs": [
      {
        "name": "tokenAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "ferryTip",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deposit_erc20",
    "inputs": [
      {
        "name": "tokenAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deposit_native",
    "inputs": [],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "deposit_native",
    "inputs": [
      {
        "name": "ferryTip",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "deposits",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "requestId",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.RequestId",
        "components": [
          {
            "name": "origin",
            "type": "uint8",
            "internalType": "enum IRolldownPrimitives.Origin"
          },
          {
            "name": "id",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "depositRecipient",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "timeStamp",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "ferryTip",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "ferryWithdrawal",
    "inputs": [
      {
        "name": "withdrawal",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.Withdrawal",
        "components": [
          {
            "name": "requestId",
            "type": "tuple",
            "internalType": "struct IRolldownPrimitives.RequestId",
            "components": [
              {
                "name": "origin",
                "type": "uint8",
                "internalType": "enum IRolldownPrimitives.Origin"
              },
              {
                "name": "id",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "recipient",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "tokenAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "amount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "ferryTip",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "ferry_withdrawal",
    "inputs": [
      {
        "name": "withdrawal",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.Withdrawal",
        "components": [
          {
            "name": "requestId",
            "type": "tuple",
            "internalType": "struct IRolldownPrimitives.RequestId",
            "components": [
              {
                "name": "origin",
                "type": "uint8",
                "internalType": "enum IRolldownPrimitives.Origin"
              },
              {
                "name": "id",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "recipient",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "tokenAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "amount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "ferryTip",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "findL2Batch",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "find_l2_batch",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getMerkleRootsLength",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getPendingRequests",
    "inputs": [
      {
        "name": "start",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "end",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.L1Update",
        "components": [
          {
            "name": "chain",
            "type": "uint8",
            "internalType": "enum IRolldownPrimitives.ChainId"
          },
          {
            "name": "pendingDeposits",
            "type": "tuple[]",
            "internalType": "struct IRolldownPrimitives.Deposit[]",
            "components": [
              {
                "name": "requestId",
                "type": "tuple",
                "internalType": "struct IRolldownPrimitives.RequestId",
                "components": [
                  {
                    "name": "origin",
                    "type": "uint8",
                    "internalType": "enum IRolldownPrimitives.Origin"
                  },
                  {
                    "name": "id",
                    "type": "uint256",
                    "internalType": "uint256"
                  }
                ]
              },
              {
                "name": "depositRecipient",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "tokenAddress",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "amount",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "timeStamp",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "ferryTip",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "pendingCancelResolutions",
            "type": "tuple[]",
            "internalType": "struct IRolldownPrimitives.CancelResolution[]",
            "components": [
              {
                "name": "requestId",
                "type": "tuple",
                "internalType": "struct IRolldownPrimitives.RequestId",
                "components": [
                  {
                    "name": "origin",
                    "type": "uint8",
                    "internalType": "enum IRolldownPrimitives.Origin"
                  },
                  {
                    "name": "id",
                    "type": "uint256",
                    "internalType": "uint256"
                  }
                ]
              },
              {
                "name": "l2RequestId",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "cancelJustified",
                "type": "bool",
                "internalType": "bool"
              },
              {
                "name": "timeStamp",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRoleAdmin",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getUpdateForL2",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.L1Update",
        "components": [
          {
            "name": "chain",
            "type": "uint8",
            "internalType": "enum IRolldownPrimitives.ChainId"
          },
          {
            "name": "pendingDeposits",
            "type": "tuple[]",
            "internalType": "struct IRolldownPrimitives.Deposit[]",
            "components": [
              {
                "name": "requestId",
                "type": "tuple",
                "internalType": "struct IRolldownPrimitives.RequestId",
                "components": [
                  {
                    "name": "origin",
                    "type": "uint8",
                    "internalType": "enum IRolldownPrimitives.Origin"
                  },
                  {
                    "name": "id",
                    "type": "uint256",
                    "internalType": "uint256"
                  }
                ]
              },
              {
                "name": "depositRecipient",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "tokenAddress",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "amount",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "timeStamp",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "ferryTip",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "pendingCancelResolutions",
            "type": "tuple[]",
            "internalType": "struct IRolldownPrimitives.CancelResolution[]",
            "components": [
              {
                "name": "requestId",
                "type": "tuple",
                "internalType": "struct IRolldownPrimitives.RequestId",
                "components": [
                  {
                    "name": "origin",
                    "type": "uint8",
                    "internalType": "enum IRolldownPrimitives.Origin"
                  },
                  {
                    "name": "id",
                    "type": "uint256",
                    "internalType": "uint256"
                  }
                ]
              },
              {
                "name": "l2RequestId",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "cancelJustified",
                "type": "bool",
                "internalType": "bool"
              },
              {
                "name": "timeStamp",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "grantRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "hasRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "hashCancel",
    "inputs": [
      {
        "name": "cancel",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.Cancel",
        "components": [
          {
            "name": "requestId",
            "type": "tuple",
            "internalType": "struct IRolldownPrimitives.RequestId",
            "components": [
              {
                "name": "origin",
                "type": "uint8",
                "internalType": "enum IRolldownPrimitives.Origin"
              },
              {
                "name": "id",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "range",
            "type": "tuple",
            "internalType": "struct IRolldownPrimitives.Range",
            "components": [
              {
                "name": "start",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "end",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "hash",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "hashFailedDepositResolution",
    "inputs": [
      {
        "name": "failedDeposit",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.FailedDepositResolution",
        "components": [
          {
            "name": "requestId",
            "type": "tuple",
            "internalType": "struct IRolldownPrimitives.RequestId",
            "components": [
              {
                "name": "origin",
                "type": "uint8",
                "internalType": "enum IRolldownPrimitives.Origin"
              },
              {
                "name": "id",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "originRequestId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "ferry",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "hashWithdrawal",
    "inputs": [
      {
        "name": "withdrawal",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.Withdrawal",
        "components": [
          {
            "name": "requestId",
            "type": "tuple",
            "internalType": "struct IRolldownPrimitives.RequestId",
            "components": [
              {
                "name": "origin",
                "type": "uint8",
                "internalType": "enum IRolldownPrimitives.Origin"
              },
              {
                "name": "id",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "recipient",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "tokenAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "amount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "ferryTip",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "admin",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "chainId",
        "type": "uint8",
        "internalType": "enum IRolldownPrimitives.ChainId"
      },
      {
        "name": "updater",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "lastProcessedUpdate_origin_l1",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "lastProcessedUpdate_origin_l2",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "merkleRootRange",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "start",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "end",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "pause",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "paused",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "processedL2Requests",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "renounceRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "revokeRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "roots",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "setUpdater",
    "inputs": [
      {
        "name": "updater",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "supportsInterface",
    "inputs": [
      {
        "name": "interfaceId",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "unpause",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateL1FromL2",
    "inputs": [
      {
        "name": "merkleRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "range",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.Range",
        "components": [
          {
            "name": "start",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "end",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "update_l1_from_l2",
    "inputs": [
      {
        "name": "merkleRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "range",
        "type": "tuple",
        "internalType": "struct IRolldownPrimitives.Range",
        "components": [
          {
            "name": "start",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "end",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updaterAccount",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "DepositAcceptedIntoQueue",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "depositRecipient",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "tokenAddress",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "ferryTip",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DisputeResolutionAcceptedIntoQueue",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "cancelJustified",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      },
      {
        "name": "cancelResolutionHash",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ERC20TokensWithdrawn",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "tokenAddress",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "FailedDepositResolutionClosed",
    "inputs": [
      {
        "name": "indexedrequestId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "originDepositId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "failedDespotiResolutionHash",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "FerriedWithdrawalClosed",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "withdrawalHash",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Initialized",
    "inputs": [
      {
        "name": "version",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "L2UpdateAccepted",
    "inputs": [
      {
        "name": "root",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "range",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IRolldownPrimitives.Range",
        "components": [
          {
            "name": "start",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "end",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "NativeTokensWithdrawn",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "NewUpdaterSet",
    "inputs": [
      {
        "name": "updater",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Paused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleAdminChanged",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "previousAdminRole",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "newAdminRole",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleGranted",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleRevoked",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Unpaused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "WithdrawalClosed",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "withdrawalHash",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "WithdrawalFerried",
    "inputs": [
      {
        "name": "indexedrequestId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "recipient",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "ferry",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "withdrawalHash",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "FerryTipExceedsAmount",
    "inputs": [
      {
        "name": "ferryTip",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidFerriedAmount",
    "inputs": [
      {
        "name": "actualAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "expectedAmount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidRequestId",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidRequestProof",
    "inputs": [
      {
        "name": "merkleRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidRequestRange",
    "inputs": [
      {
        "name": "start",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "end",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidUpdateRange",
    "inputs": [
      {
        "name": "start",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "end",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "L2RequestAlreadyProcessed",
    "inputs": [
      {
        "name": "requestHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "PreviousUpdateMissed",
    "inputs": [
      {
        "name": "currentStartRange",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "lastProcessedUpdate",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "RequestOutOfRange",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "start",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "end",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "RequestRangeTooLarge",
    "inputs": [
      {
        "name": "count",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "UnexpectedMerkleRoot",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UpdateAlreadyApplied",
    "inputs": [
      {
        "name": "currentEndRange",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "lastProcessedUpdate",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "WithdrawalAlreadyFerried",
    "inputs": [
      {
        "name": "withdrawalHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "ZeroAdmin",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ZeroAmount",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ZeroRecipient",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ZeroRootCount",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ZeroToken",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ZeroTransferAmount",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ZeroUpdateRange",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ZeroUpdater",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod Rolldown {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b506139aa806100206000396000f3fe60806040526004361061031a5760003560e01c8063890e95ce116101ab578063ca9b21ae116100f7578063de70e0b811610095578063f26ee9d01161006f578063f26ee9d01461082d578063f35f9e4514610843578063f9ecd01e146107a1578063ff2bae861461086357600080fd5b8063de70e0b8146107e1578063df2ebdbb14610818578063dffbdd9f1461058057600080fd5b8063d16544f0116100d1578063d16544f0146104fe578063d1cb26b41461031f578063d547741f146107c1578063db6b52461461074957600080fd5b8063ca9b21ae14610751578063cc8c909f14610781578063ce2de1bc146107a157600080fd5b8063a217fddf11610164578063b15387061161013e578063b1538706146106ed578063c2b40ae414610702578063c763e5a114610722578063c87c22241461074957600080fd5b8063a217fddf14610686578063ae46db111461069b578063b02c43d0146106bb57600080fd5b8063890e95ce146106265780638e24e392146103b657806391d1485414610646578063950ac4871461046757806397feb926146104fe5780639d54f4191461066657600080fd5b80633f4ba83a1161026a578063608fc37a1161022357806371c54461116101fd57806371c54461146105a957806379e041f2146105ce5780637fd4f845146105fb5780638456cb591461061157600080fd5b8063608fc37a1461058057806361bc221a14610593578063676f536b146103d657600080fd5b80633f4ba83a146104c757806347e63380146104dc57806347e7ef24146104fe5780634bf5fec3146103765780634f48eedf1461051e5780635c975abb1461056857600080fd5b80630e2636a3116102d7578063248a9ca3116102b1578063248a9ca31461042957806325afc76a146104675780632f2ff15d1461048757806336568abe146104a757600080fd5b80630e2636a3146103e95780630efe6a8b1461039657806321425ee01461039657600080fd5b806301ef69661461031f57806301ffc9a71461034157806303ed49d31461037657806308aba1b21461039657806308f42d40146103b65780630cac57ab146103d6575b600080fd5b34801561032b57600080fd5b5061033f61033a366004613089565b610879565b005b34801561034d57600080fd5b5061036161035c3660046130e4565b6108c8565b60405190151581526020015b60405180910390f35b34801561038257600080fd5b5061033f610391366004613120565b6108ff565b3480156103a257600080fd5b5061033f6103b1366004613180565b61093a565b3480156103c257600080fd5b5061033f6103d13660046131b3565b61097e565b61033f6103e43660046131eb565b6109ad565b3480156103f557600080fd5b5061041173111111111111111111111111111111111111111181565b6040516001600160a01b03909116815260200161036d565b34801561043557600080fd5b50610459610444366004613207565b60009081526065602052604090206001015490565b60405190815260200161036d565b34801561047357600080fd5b5061033f610482366004613232565b6109ed565b34801561049357600080fd5b5061033f6104a2366004613275565b610a28565b3480156104b357600080fd5b5061033f6104c2366004613275565b610a4d565b3480156104d357600080fd5b5061033f610acb565b3480156104e857600080fd5b5061045960008051602061395583398151915281565b34801561050a57600080fd5b5061033f6105193660046132a1565b610ae1565b34801561052a57600080fd5b50610553610539366004613207565b610101602052600090815260409020805460019091015482565b6040805192835260208301919091520161036d565b34801561057457600080fd5b5060c95460ff16610361565b61033f61058e366004613207565b610b25565b34801561059f57600080fd5b5061045960fb5481565b3480156105b557600080fd5b5060fe546104119061010090046001600160a01b031681565b3480156105da57600080fd5b506105ee6105e93660046132cb565b610b36565b60405161036d9190613393565b34801561060757600080fd5b5061045960fc5481565b34801561061d57600080fd5b5061033f610fda565b34801561063257600080fd5b506104596106413660046131eb565b610fed565b34801561065257600080fd5b50610361610661366004613275565b61105b565b34801561067257600080fd5b5061033f610681366004613456565b611086565b34801561069257600080fd5b50610459600081565b3480156106a757600080fd5b506104596106b6366004613471565b611150565b3480156106c757600080fd5b506106db6106d6366004613207565b611184565b60405161036d9695949392919061348d565b3480156106f957600080fd5b506105ee61120c565b34801561070e57600080fd5b5061045961071d366004613207565b611257565b34801561072e57600080fd5b5060fe5461073c9060ff1681565b60405161036d91906134cf565b61033f611279565b34801561075d57600080fd5b5061077161076c366004613207565b61128d565b60405161036d94939291906134e2565b34801561078d57600080fd5b5061045961079c36600461350b565b6112ff565b3480156107ad57600080fd5b506104596107bc366004613207565b611333565b3480156107cd57600080fd5b5061033f6107dc366004613275565b61133e565b3480156107ed57600080fd5b506104116107fc366004613207565b610102602052600090815260409020546001600160a01b031681565b34801561082457600080fd5b50610411600181565b34801561083957600080fd5b5061045960fd5481565b34801561084f57600080fd5b5061033f61085e366004613534565b611363565b34801561086f57600080fd5b5061010354610459565b610881611566565b6002609754036108ac5760405162461bcd60e51b81526004016108a390613579565b60405180910390fd5b60026097556108bd848484846115ac565b505060016097555050565b60006001600160e01b03198216637965db0b60e01b14806108f957506301ffc9a760e01b6001600160e01b03198316145b92915050565b6002609754036109215760405162461bcd60e51b81526004016108a390613579565b600260975561092e611566565b6108bd8484848461160d565b60026097540361095c5760405162461bcd60e51b81526004016108a390613579565b6002609755610969611566565b6109748383836117fd565b5050600160975550565b610986611566565b60008051602061395583398151915261099e816119d3565b6109a883836119dd565b505050565b6002609754036109cf5760405162461bcd60e51b81526004016108a390613579565b60026097556109dc611566565b6109e581611b46565b506001609755565b600260975403610a0f5760405162461bcd60e51b81526004016108a390613579565b6002609755610a1c611566565b6108bd84848484611dbd565b600082815260656020526040902060010154610a43816119d3565b6109a88383611de5565b6001600160a01b0381163314610abd5760405162461bcd60e51b815260206004820152602f60248201527f416363657373436f6e74726f6c3a2063616e206f6e6c792072656e6f756e636560448201526e103937b632b9903337b91039b2b63360891b60648201526084016108a3565b610ac78282611e6b565b5050565b6000610ad6816119d3565b610ade611ed2565b50565b600260975403610b035760405162461bcd60e51b81526004016108a390613579565b6002609755610b10611566565b610b1c828260006117fd565b50506001609755565b610b2d611566565b610ade81611f24565b610b5b6040805160608101909152806000815260200160608152602001606081525090565b604080516060810190915260fe5460009190819060ff166001811115610b8357610b836132ed565b81526020016000604051908082528060200260200182016040528015610bf857816020015b6040805161010081018252600060c0820181815260e0830182905282526020808301829052928201819052606082018190526080820181905260a08201528252600019909201910181610ba85790505b5081526020016000604051908082528060200260200182016040528015610c5f57816020015b6040805160c08101825260006080820181815260a083018290528252602080830182905292820181905260608201528252600019909201910181610c1e5790505b509052905083158015610c70575082155b15610c7c5790506108f9565b600080855b858111610d02576000818152610100602052604090206001015415610cb057610ca9836135dc565b9250610cfa565b600081815260ff602052604090206001015415610cd757610cd0826135dc565b9150610cfa565b6040516354b4960f60e11b815260048101889052602481018790526044016108a3565b600101610c81565b508167ffffffffffffffff811115610d1c57610d1c6135b0565b604051908082528060200260200182016040528015610d8a57816020015b6040805161010081018252600060c0820181815260e0830182905282526020808301829052928201819052606082018190526080820181905260a08201528252600019909201910181610d3a5790505b5060208401528067ffffffffffffffff811115610da957610da96135b0565b604051908082528060200260200182016040528015610e0857816020015b6040805160c08101825260006080820181815260a083018290528252602080830182905292820181905260608201528252600019909201910181610dc75790505b506040840152506000905080855b858111610fcf576000818152610100602052604090206001015415610f05576000818152610100602081905260409182902082519182019092528154909190829060c08201908390829060ff166001811115610e7457610e746132ed565b6001811115610e8557610e856132ed565b81526001919091015460209182015290825260028301546001600160a01b03908116838301526003840154166040830152600483015460608301526005830154608083015260069092015460a09091015285015184610ee3816135dc565b955081518110610ef557610ef56135f5565b6020026020010181905250610fc7565b600081815260ff602052604090206002015415610fc257600081815260ff6020819052604091829020825160c08101909352805490918391608083019184918391166001811115610f5857610f586132ed565b6001811115610f6957610f696132ed565b815260019190910154602091820152908252600283015490820152600382015460ff16151560408083019190915260049092015460609091015285015183610fb0816135dc565b945081518110610ef557610ef56135f5565b610fcf565b600101610e16565b509195945050505050565b6000610fe5816119d3565b610ade6120ba565b600080604051602001611000919061360b565b604051602081830303815290604052826040516020016110209190613640565b60408051601f198184030181529082905261103e92916020016136c5565b604051602081830303815290604052805190602001209050919050565b60009182526065602090815260408084206001600160a01b0393909316845291905290205460ff1690565b6000611091816119d3565b6001600160a01b0382166110bb5760405160016279c35d60e01b0319815260040160405180910390fd5b60fe546110e5906000805160206139558339815191529061010090046001600160a01b0316611e6b565b6110fd60008051602061395583398151915283611de5565b60fe8054610100600160a81b0319166101006001600160a01b038516908102919091179091556040517f1b0f2f500df596b42b73e80dbec6a1fb570f01978a586723f988a5fc54d773a190600090a25050565b60006002604051602001611164919061360b565b6040516020818303038152906040528260405160200161102091906136f4565b6101006020526000908152604090819020815180830190925280549091908290829060ff1660018111156111ba576111ba6132ed565b60018111156111cb576111cb6132ed565b815260019190910154602090910152600282015460038301546004840154600585015460069095015493946001600160a01b03938416949290931692909186565b6112316040805160608101909152806000815260200160608152602001606081525090565b61125260fc546001611243919061372c565b600160fb546105e99190613744565b905090565b610103818154811061126857600080fd5b600091825260209091200154905081565b611281611566565b61128b6000611f24565b565b60ff6020819052600091825260409182902082518084019093528054909291839183911660018111156112c2576112c26132ed565b60018111156112d3576112d36132ed565b815260019190910154602090910152600282015460038301546004909301549192909160ff9091169084565b60006001604051602001611313919061360b565b60405160208183030381529060405282604051602001611020919061375b565b60006108f9826120f7565b600082815260656020526040902060010154611359816119d3565b6109a88383611e6b565b600054610100900460ff16158080156113835750600054600160ff909116105b8061139d5750303b15801561139d575060005460ff166001145b6114005760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084016108a3565b6000805460ff191660011790558015611423576000805461ff0019166101001790555b61142b61221a565b61143361221a565b61143b612241565b611443612270565b6001600160a01b03841661146a57604051633944ed8760e11b815260040160405180910390fd5b611475600085611de5565b6001600160a01b03821661149f5760405160016279c35d60e01b0319815260040160405180910390fd5b6114b760008051602061395583398151915283611de5565b60fe8054610100600160a81b031981166101006001600160a01b038616029081178355600160fb819055600060fc81905560fd558693926001600160a81b03191660ff1990921691909117908381811115611514576115146132ed565b02179055508015611560576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498906020015b60405180910390a15b50505050565b60c95460ff161561128b5760405162461bcd60e51b815260206004820152601060248201526f14185d5cd8589b194e881c185d5cd95960821b60448201526064016108a3565b60006115b7856112ff565b90506115ca60208601358286868661229f565b6115d48582612490565b60009081526101026020526040902080546001600160a01b03191673111111111111111111111111111111111111111117905550505050565b600061161885610fed565b905061162b60208601358286868661229f565b60008181526101026020526040902080546001600160a01b03198116731111111111111111111111111111111111111111179091556001600160a01b031680156116ff5760016116816080880160608901613456565b6001600160a01b0316146116b2576116ad816116a36080890160608a01613456565b88608001356125f6565b6116c0565b6116c0818760800135612677565b604051828152602080880135917f2996fd546c37d74c170466ea6aa4a308e3ca2d4aa689e6e9e32994db5039cc0e910160405180910390a25050611560565b600061171360a08801356080890135613744565b905060016117276080890160608a01613456565b6001600160a01b03160361176b5761174e6117486060890160408a01613456565b82612677565b60a08701351561176657611766338860a00135612677565b6117bc565b61179461177e6060890160408a01613456565b61178e60808a0160608b01613456565b836125f6565b60a0870135156117bc576117bc336117b260808a0160608b01613456565b8960a001356125f6565b604051838152602080890135917f935f26d94be31907080aa78b3e6e2ac6d48a072af096c20268388621bbc11789910160405180910390a250505050505050565b81818160000361182057604051631f2a200560e01b815260040160405180910390fd5b8181111561184b5760405163202b316960e21b815260048101829052602481018390526044016108a3565b6001600160a01b0385166118725760405163ad1991f560e01b815260040160405180910390fd5b60006040518060c0016040528061188960006126e5565b8152602001336001600160a01b03908116825288166020808301919091526040808301899052426060840152608090920187905282518101516000908152610100909152208151805182549394508493839190829060ff1916600183818111156118f5576118f56132ed565b0217905550602091820151600191909101558201516002820180546001600160a01b03199081166001600160a01b03938416179091556040840151600384018054909216908316179055606083015160048301556080830151600583015560a0909201516006909101558616336001600160a01b03168260000151602001517f225305ecb66fa9b9b29f8dead9baea365a6c225d639dfd866e782ccb63e2f05b88886040516119ae929190918252602082015260400190565b60405180910390a46119cb6001600160a01b03871633308861273a565b505050505050565b610ade81336127a5565b80356000036119ff576040516369f1cfef60e01b815260040160405180910390fd5b602081013581351115611a325760405163722fc3f760e11b815281356004820152602082013560248201526044016108a3565b60fd54611a4160018335613744565b1115611a6e5760fd54604051630650047360e51b81528235600482015260248101919091526044016108a3565b60fd54816020013511611aa55760fd546040516350a792b160e01b81526020830135600482015260248101919091526044016108a3565b6101038054600181019091557f02c297ab74aad0aede3a1895c857b1f2c71e6a203feb727bec95ac752998cb78018290556000828152610101602052604090208190611afe828281358155602082013560018201555050565b5050602081013560fd556040517f49c158d490db9e066f01b5d4f1a094485a6598cb6c5296b4c07e46c12a1dc11c90611b3a9084908490613791565b60405180910390a15050565b80608001358160a0013581600003611b7157604051631f2a200560e01b815260040160405180910390fd5b81811115611b9c5760405163202b316960e21b815260048101829052602481018390526044016108a3565b6000611bae6060850160408601613456565b6001600160a01b031603611bd55760405163d27b444360e01b815260040160405180910390fd5b6000611be084610fed565b600081815261010260205260409020549091506001600160a01b031615611c1d5760405163fea5945360e01b8152600481018290526024016108a3565b60008181526101026020526040812080546001600160a01b03191633179055611c4e60a08601356080870135613744565b90506001611c626080870160608801613456565b6001600160a01b031603611d2557803414611c9957604051634ceaf5d360e11b8152346004820152602481018290526044016108a3565b33611caa6060870160408801613456565b604080516020808a0135825281018590529081018590526001600160a01b0391909116907f7a9abd9eb86bdbca89cba4069a632c37d93db82e3e14ad8119a3a7812814853e9060600160405180910390a3611d1e81611d0f6060880160408901613456565b6001600160a01b031690612809565b5050505050565b33611d366060870160408801613456565b604080516020808a0135825281018590529081018590526001600160a01b0391909116907f7a9abd9eb86bdbca89cba4069a632c37d93db82e3e14ad8119a3a7812814853e9060600160405180910390a3611d1e33611d9b6060880160408901613456565b83611dac60808a0160608b01613456565b6001600160a01b031692919061273a565b6000611dc885611150565b9050611ddb60208601358286868661229f565b6115d48582612922565b611def828261105b565b610ac75760008281526065602090815260408083206001600160a01b03851684529091529020805460ff19166001179055611e273390565b6001600160a01b0316816001600160a01b0316837f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a45050565b611e75828261105b565b15610ac75760008281526065602090815260408083206001600160a01b0385168085529252808320805460ff1916905551339285917ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b9190a45050565b611eda612a07565b60c9805460ff191690557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa335b6040516001600160a01b03909116815260200160405180910390a1565b348181600003611f4757604051631f2a200560e01b815260040160405180910390fd5b81811115611f725760405163202b316960e21b815260048101829052602481018390526044016108a3565b60006040518060c00160405280611f8960006126e5565b8152336020808301919091526001604080840182905234606085015242608085015260a0909301889052835182015160009081526101009092529190208251805182549495508594929391928492839160ff1916908381811115611fef57611fef6132ed565b0217905550602091820151600191820155908301516002830180546001600160a01b039283166001600160a01b0319918216179091556040850151600385018054919093169116179055606083015160048301556080830151600583015560a0909201516006909101556120603390565b6001600160a01b03168260000151602001517f225305ecb66fa9b9b29f8dead9baea365a6c225d639dfd866e782ccb63e2f05b34886040516120ac929190918252602082015260400190565b60405180910390a450505050565b6120c2611566565b60c9805460ff191660011790557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a258611f073390565b600060fd5482111561211f576040516364b4f07960e11b8152600481018390526024016108a3565b61010354600081900361214557604051635d43707560e01b815260040160405180910390fd5b805b80156121d157600061010361215d600184613744565b8154811061216d5761216d6135f5565b600091825260208083209091015480835261010182526040928390208351808501909452805480855260019091015492840192909252925086108015906121b8575080602001518611155b156121c65750949350505050565b505060001901612147565b5060405162461bcd60e51b815260206004820152601c60248201527f426174636820776974682072657175657374206e6f7420666f756e640000000060448201526064016108a3565b600054610100900460ff1661128b5760405162461bcd60e51b81526004016108a3906137af565b600054610100900460ff166122685760405162461bcd60e51b81526004016108a3906137af565b61128b612a50565b600054610100900460ff166122975760405162461bcd60e51b81526004016108a3906137af565b61128b612a83565b600084815261010260205260409020546001600160a01b031673111111111111111111111111111111111111111019016122ef5760405163e99711f160e01b8152600481018590526024016108a3565b600083815261010160209081526040918290208251808401909352805480845260019091015491830191909152158061232a57506020810151155b15612348576040516339075ba160e21b815260040160405180910390fd5b80516020820151101561237e57805160208201516040516354b4960f60e11b8152600481019290925260248201526044016108a3565b80518610806123905750806020015186115b156123c55780516020820151604051634d346e8960e01b815260048101899052602481019290925260448201526064016108a3565b805160208201516000916123d891613744565b6123e390600161372c565b905063ffffffff81111561240d57604051632095a53d60e21b8152600481018290526024016108a3565b815160009061241c9089613744565b905060006124608883888880806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250899250612ab1915050565b90508087146124855760405163f6ae8d5360e01b8152600481018890526024016108a3565b505050505050505050565b6000600160fb546124a19190613744565b606084013511156124b4575060016124fc565b60006124c860408501356060860135610b36565b9050806040516020016124db9190613393565b60405160208183030381529060405280519060200120846080013514159150505b6000604051806080016040528061251360006126e5565b815260208681013581830152841515604080840191909152426060909301929092528251810151600090815260ff909152208151805182549394508493839190829060ff19166001838181111561256c5761256c6132ed565b02179055506020918201516001919091015582810151600283015560408084015160038401805460ff1916911515919091179055606090930151600490920191909155828101518383015183519015158152918201869052917f9ef11353afd97d339a77732850b7c22704656558d9ba63cc7e321e0ac4c20aa9910160405180910390a250505050565b80600003612617576040516329c5442960e01b815260040160405180910390fd5b816001600160a01b0316836001600160a01b03167ee763f7778b8ceef7270c89b7d1df1008b0e482da39c43831417733af96fb0d8360405161265b91815260200190565b60405180910390a36109a86001600160a01b0383168483612aff565b80600003612698576040516329c5442960e01b815260040160405180910390fd5b816001600160a01b03167fe0495355c1e04c512584521854d222d239a4b782b39ac8a7e835a34f5ec7c1e1826040516126d391815260200190565b60405180910390a2610ac78282612809565b60408051808201909152600080825260208201526040518060400160405280836001811115612716576127166132ed565b815260200160fb600081548092919061272e906135dc565b90915550905292915050565b6040516001600160a01b03808516602483015283166044820152606481018290526115609085906323b872dd60e01b906084015b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b031990931692909217909152612b2f565b6127af828261105b565b610ac7576127c7816001600160a01b03166014612c01565b6127d2836020612c01565b6040516020016127e39291906137fa565b60408051601f198184030181529082905262461bcd60e51b82526108a39160040161386f565b804710156128595760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a20696e73756666696369656e742062616c616e636500000060448201526064016108a3565b6000826001600160a01b03168260405160006040518083038185875af1925050503d80600081146128a6576040519150601f19603f3d011682016040523d82523d6000602084013e6128ab565b606091505b50509050806109a85760405162461bcd60e51b815260206004820152603a60248201527f416464726573733a20756e61626c6520746f2073656e642076616c75652c207260448201527f6563697069656e74206d6179206861766520726576657274656400000000000060648201526084016108a3565b6040808301356000908152610100602052908120600281015490916001600160a01b03909116906129596080860160608701613456565b6001600160a01b03161461297a576129776080850160608601613456565b90505b60038201546001600160a01b03166001146129b557600382015460048301546129b09183916001600160a01b03909116906125f6565b6129c3565b6129c3818360040154612677565b60408051602086810135825286830135908201529081018490527f13750c731f87c15242874ce74bf4649502cc8e7c829067ce846505acdb96289d90606001611557565b60c95460ff1661128b5760405162461bcd60e51b815260206004820152601460248201527314185d5cd8589b194e881b9bdd081c185d5cd95960621b60448201526064016108a3565b600054610100900460ff16612a775760405162461bcd60e51b81526004016108a3906137af565b60c9805460ff19169055565b600054610100900460ff16612aaa5760405162461bcd60e51b81526004016108a3906137af565b6001609755565b600080825b8015612adb57612ac76002826138b8565b9050612ad460018361372c565b9150612ab6565b612af4828789886000612aef60018b613744565b612da4565b979650505050505050565b6040516001600160a01b0383166024820152604481018290526109a890849063a9059cbb60e01b9060640161276e565b6000612b84826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b0316612eb49092919063ffffffff16565b8051909150156109a85780806020019051810190612ba291906138cc565b6109a85760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b60648201526084016108a3565b60606000612c108360026138ee565b612c1b90600261372c565b67ffffffffffffffff811115612c3357612c336135b0565b6040519080825280601f01601f191660200182016040528015612c5d576020820181803683370190505b509050600360fc1b81600081518110612c7857612c786135f5565b60200101906001600160f81b031916908160001a905350600f60fb1b81600181518110612ca757612ca76135f5565b60200101906001600160f81b031916908160001a9053506000612ccb8460026138ee565b612cd690600161372c565b90505b6001811115612d4e576f181899199a1a9b1b9c1cb0b131b232b360811b85600f1660108110612d0a57612d0a6135f5565b1a60f81b828281518110612d2057612d206135f5565b60200101906001600160f81b031916908160001a90535060049490941c93612d478161390d565b9050612cd9565b508315612d9d5760405162461bcd60e51b815260206004820181905260248201527f537472696e67733a20686578206c656e67746820696e73756666696369656e7460448201526064016108a3565b9392505050565b6000612db1600287613924565b600003612e1c57858214612e7557848484612dcb816135dc565b955081518110612ddd57612ddd6135f5565b6020026020010151604051602001612dff929190918252602082015260400190565b604051602081830303815290604052805190602001209450612e75565b8383612e27816135dc565b945081518110612e3957612e396135f5565b602002602001015185604051602001612e5c929190918252602082015260400190565b6040516020818303038152906040528051906020012094505b86600114612ea957612ea4612e8b600189613744565b612e966002896138b8565b878787612aef6002896138b8565b612af4565b509295945050505050565b6060612ec38484600085612ecb565b949350505050565b606082471015612f2c5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f6044820152651c8818d85b1b60d21b60648201526084016108a3565b6001600160a01b0385163b612f835760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e747261637400000060448201526064016108a3565b600080866001600160a01b03168587604051612f9f9190613938565b60006040518083038185875af1925050503d8060008114612fdc576040519150601f19603f3d011682016040523d82523d6000602084013e612fe1565b606091505b5091509150612af482828660608315612ffb575081612d9d565b82511561300b5782518084602001fd5b8160405162461bcd60e51b81526004016108a3919061386f565b600060a0828403121561303757600080fd5b50919050565b60008083601f84011261304f57600080fd5b50813567ffffffffffffffff81111561306757600080fd5b6020830191508360208260051b850101111561308257600080fd5b9250929050565b60008060008060e0858703121561309f57600080fd5b6130a98686613025565b935060a0850135925060c085013567ffffffffffffffff8111156130cc57600080fd5b6130d88782880161303d565b95989497509550505050565b6000602082840312156130f657600080fd5b81356001600160e01b031981168114612d9d57600080fd5b600060c0828403121561303757600080fd5b600080600080610100858703121561313757600080fd5b613141868661310e565b935060c0850135925060e085013567ffffffffffffffff8111156130cc57600080fd5b80356001600160a01b038116811461317b57600080fd5b919050565b60008060006060848603121561319557600080fd5b61319e84613164565b95602085013595506040909401359392505050565b60008082840360608112156131c757600080fd5b833592506040601f19820112156131dd57600080fd5b506020830190509250929050565b600060c082840312156131fd57600080fd5b612d9d838361310e565b60006020828403121561321957600080fd5b5035919050565b60006080828403121561303757600080fd5b60008060008060c0858703121561324857600080fd5b6132528686613220565b93506080850135925060a085013567ffffffffffffffff8111156130cc57600080fd5b6000806040838503121561328857600080fd5b8235915061329860208401613164565b90509250929050565b600080604083850312156132b457600080fd5b6132bd83613164565b946020939093013593505050565b600080604083850312156132de57600080fd5b50508035926020909101359150565b634e487b7160e01b600052602160045260246000fd5b60028110610ade57610ade6132ed565b805161331e81613303565b8252602090810151910152565b600081518084526020808501945080840160005b83811015613388578151613354888251613313565b8084015160408981019190915281015115156060808a01919091520151608088015260a0909601959082019060010161333f565b509495945050505050565b60006020808352608080840185516133aa81613303565b85840152858301516060604080880182905282519384905260a093928601928489019060005b8181101561342b5785516133e5848251613313565b808a01516001600160a01b03908116858701528582015116868501528581015189850152888101518885015287015160c08401529488019460e0909201916001016133d0565b505089820151898203601f1901848b01529650613448818861332b565b9a9950505050505050505050565b60006020828403121561346857600080fd5b612d9d82613164565b60006080828403121561348357600080fd5b612d9d8383613220565b60e0810161349b8289613313565b6001600160a01b039687166040830152949095166060860152608085019290925260a084015260c090920191909152919050565b602081016134dc83613303565b91905290565b60a081016134f08287613313565b60408201949094529115156060830152608090910152919050565b600060a0828403121561351d57600080fd5b612d9d8383613025565b60028110610ade57600080fd5b60008060006060848603121561354957600080fd5b61355284613164565b9250602084013561356281613527565b915061357060408501613164565b90509250925092565b6020808252601f908201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c00604082015260600190565b634e487b7160e01b600052604160045260246000fd5b634e487b7160e01b600052601160045260246000fd5b6000600182016135ee576135ee6135c6565b5060010190565b634e487b7160e01b600052603260045260246000fd5b60208101600383106134dc576134dc6132ed565b803561362a81613527565b61363381613303565b8252602090810135910152565b60c0810161364e828461361f565b61365a60408401613164565b6001600160a01b0381811660408501528061367760608701613164565b16606085015250506080830135608083015260a083013560a083015292915050565b60005b838110156136b457818101518382015260200161369c565b838111156115605750506000910152565b600083516136d7818460208801613699565b8351908301906136eb818360208801613699565b01949350505050565b60808101613702828461361f565b604083810135908301526001600160a01b0361372060608501613164565b16606083015292915050565b6000821982111561373f5761373f6135c6565b500190565b600082821015613756576137566135c6565b500390565b60a08101613769828461361f565b613783604083016040850180358252602090810135910152565b608092830135919092015290565b82815260608101612d9d602083018480358252602090810135910152565b6020808252602b908201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960408201526a6e697469616c697a696e6760a81b606082015260800190565b7f416363657373436f6e74726f6c3a206163636f756e7420000000000000000000815260008351613832816017850160208801613699565b7001034b99036b4b9b9b4b733903937b6329607d1b6017918401918201528351613863816028840160208801613699565b01602801949350505050565b602081526000825180602084015261388e816040850160208701613699565b601f01601f19169190910160400192915050565b634e487b7160e01b600052601260045260246000fd5b6000826138c7576138c76138a2565b500490565b6000602082840312156138de57600080fd5b81518015158114612d9d57600080fd5b6000816000190483118215151615613908576139086135c6565b500290565b60008161391c5761391c6135c6565b506000190190565b600082613933576139336138a2565b500690565b6000825161394a818460208701613699565b919091019291505056fe73e573f9566d61418a34d5de3ff49360f9c51fec37f7486551670290f6285daba2646970667358221220d0aa7bcf1b59b8eb1ebf01f0f80c4dd46832c645f0af2a6d356c99d2a885a67664736f6c634300080d0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa9\xAA\x80a\0 `\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x03\x1AW`\x005`\xE0\x1C\x80c\x89\x0E\x95\xCE\x11a\x01\xABW\x80c\xCA\x9B!\xAE\x11a\0\xF7W\x80c\xDEp\xE0\xB8\x11a\0\x95W\x80c\xF2n\xE9\xD0\x11a\0oW\x80c\xF2n\xE9\xD0\x14a\x08-W\x80c\xF3_\x9EE\x14a\x08CW\x80c\xF9\xEC\xD0\x1E\x14a\x07\xA1W\x80c\xFF+\xAE\x86\x14a\x08cW`\0\x80\xFD[\x80c\xDEp\xE0\xB8\x14a\x07\xE1W\x80c\xDF.\xBD\xBB\x14a\x08\x18W\x80c\xDF\xFB\xDD\x9F\x14a\x05\x80W`\0\x80\xFD[\x80c\xD1eD\xF0\x11a\0\xD1W\x80c\xD1eD\xF0\x14a\x04\xFEW\x80c\xD1\xCB&\xB4\x14a\x03\x1FW\x80c\xD5Gt\x1F\x14a\x07\xC1W\x80c\xDBkRF\x14a\x07IW`\0\x80\xFD[\x80c\xCA\x9B!\xAE\x14a\x07QW\x80c\xCC\x8C\x90\x9F\x14a\x07\x81W\x80c\xCE-\xE1\xBC\x14a\x07\xA1W`\0\x80\xFD[\x80c\xA2\x17\xFD\xDF\x11a\x01dW\x80c\xB1S\x87\x06\x11a\x01>W\x80c\xB1S\x87\x06\x14a\x06\xEDW\x80c\xC2\xB4\n\xE4\x14a\x07\x02W\x80c\xC7c\xE5\xA1\x14a\x07\"W\x80c\xC8|\"$\x14a\x07IW`\0\x80\xFD[\x80c\xA2\x17\xFD\xDF\x14a\x06\x86W\x80c\xAEF\xDB\x11\x14a\x06\x9BW\x80c\xB0,C\xD0\x14a\x06\xBBW`\0\x80\xFD[\x80c\x89\x0E\x95\xCE\x14a\x06&W\x80c\x8E$\xE3\x92\x14a\x03\xB6W\x80c\x91\xD1HT\x14a\x06FW\x80c\x95\n\xC4\x87\x14a\x04gW\x80c\x97\xFE\xB9&\x14a\x04\xFEW\x80c\x9DT\xF4\x19\x14a\x06fW`\0\x80\xFD[\x80c?K\xA8:\x11a\x02jW\x80c`\x8F\xC3z\x11a\x02#W\x80cq\xC5Da\x11a\x01\xFDW\x80cq\xC5Da\x14a\x05\xA9W\x80cy\xE0A\xF2\x14a\x05\xCEW\x80c\x7F\xD4\xF8E\x14a\x05\xFBW\x80c\x84V\xCBY\x14a\x06\x11W`\0\x80\xFD[\x80c`\x8F\xC3z\x14a\x05\x80W\x80ca\xBC\"\x1A\x14a\x05\x93W\x80cgoSk\x14a\x03\xD6W`\0\x80\xFD[\x80c?K\xA8:\x14a\x04\xC7W\x80cG\xE63\x80\x14a\x04\xDCW\x80cG\xE7\xEF$\x14a\x04\xFEW\x80cK\xF5\xFE\xC3\x14a\x03vW\x80cOH\xEE\xDF\x14a\x05\x1EW\x80c\\\x97Z\xBB\x14a\x05hW`\0\x80\xFD[\x80c\x0E&6\xA3\x11a\x02\xD7W\x80c$\x8A\x9C\xA3\x11a\x02\xB1W\x80c$\x8A\x9C\xA3\x14a\x04)W\x80c%\xAF\xC7j\x14a\x04gW\x80c//\xF1]\x14a\x04\x87W\x80c6V\x8A\xBE\x14a\x04\xA7W`\0\x80\xFD[\x80c\x0E&6\xA3\x14a\x03\xE9W\x80c\x0E\xFEj\x8B\x14a\x03\x96W\x80c!B^\xE0\x14a\x03\x96W`\0\x80\xFD[\x80c\x01\xEFif\x14a\x03\x1FW\x80c\x01\xFF\xC9\xA7\x14a\x03AW\x80c\x03\xEDI\xD3\x14a\x03vW\x80c\x08\xAB\xA1\xB2\x14a\x03\x96W\x80c\x08\xF4-@\x14a\x03\xB6W\x80c\x0C\xACW\xAB\x14a\x03\xD6W[`\0\x80\xFD[4\x80\x15a\x03+W`\0\x80\xFD[Pa\x03?a\x03:6`\x04a0\x89V[a\x08yV[\0[4\x80\x15a\x03MW`\0\x80\xFD[Pa\x03aa\x03\\6`\x04a0\xE4V[a\x08\xC8V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x82W`\0\x80\xFD[Pa\x03?a\x03\x916`\x04a1 V[a\x08\xFFV[4\x80\x15a\x03\xA2W`\0\x80\xFD[Pa\x03?a\x03\xB16`\x04a1\x80V[a\t:V[4\x80\x15a\x03\xC2W`\0\x80\xFD[Pa\x03?a\x03\xD16`\x04a1\xB3V[a\t~V[a\x03?a\x03\xE46`\x04a1\xEBV[a\t\xADV[4\x80\x15a\x03\xF5W`\0\x80\xFD[Pa\x04\x11s\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03mV[4\x80\x15a\x045W`\0\x80\xFD[Pa\x04Ya\x04D6`\x04a2\x07V[`\0\x90\x81R`e` R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x03mV[4\x80\x15a\x04sW`\0\x80\xFD[Pa\x03?a\x04\x826`\x04a22V[a\t\xEDV[4\x80\x15a\x04\x93W`\0\x80\xFD[Pa\x03?a\x04\xA26`\x04a2uV[a\n(V[4\x80\x15a\x04\xB3W`\0\x80\xFD[Pa\x03?a\x04\xC26`\x04a2uV[a\nMV[4\x80\x15a\x04\xD3W`\0\x80\xFD[Pa\x03?a\n\xCBV[4\x80\x15a\x04\xE8W`\0\x80\xFD[Pa\x04Y`\0\x80Q` a9U\x839\x81Q\x91R\x81V[4\x80\x15a\x05\nW`\0\x80\xFD[Pa\x03?a\x05\x196`\x04a2\xA1V[a\n\xE1V[4\x80\x15a\x05*W`\0\x80\xFD[Pa\x05Sa\x0596`\x04a2\x07V[a\x01\x01` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x03mV[4\x80\x15a\x05tW`\0\x80\xFD[P`\xC9T`\xFF\x16a\x03aV[a\x03?a\x05\x8E6`\x04a2\x07V[a\x0B%V[4\x80\x15a\x05\x9FW`\0\x80\xFD[Pa\x04Y`\xFBT\x81V[4\x80\x15a\x05\xB5W`\0\x80\xFD[P`\xFETa\x04\x11\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x05\xDAW`\0\x80\xFD[Pa\x05\xEEa\x05\xE96`\x04a2\xCBV[a\x0B6V[`@Qa\x03m\x91\x90a3\x93V[4\x80\x15a\x06\x07W`\0\x80\xFD[Pa\x04Y`\xFCT\x81V[4\x80\x15a\x06\x1DW`\0\x80\xFD[Pa\x03?a\x0F\xDAV[4\x80\x15a\x062W`\0\x80\xFD[Pa\x04Ya\x06A6`\x04a1\xEBV[a\x0F\xEDV[4\x80\x15a\x06RW`\0\x80\xFD[Pa\x03aa\x06a6`\x04a2uV[a\x10[V[4\x80\x15a\x06rW`\0\x80\xFD[Pa\x03?a\x06\x816`\x04a4VV[a\x10\x86V[4\x80\x15a\x06\x92W`\0\x80\xFD[Pa\x04Y`\0\x81V[4\x80\x15a\x06\xA7W`\0\x80\xFD[Pa\x04Ya\x06\xB66`\x04a4qV[a\x11PV[4\x80\x15a\x06\xC7W`\0\x80\xFD[Pa\x06\xDBa\x06\xD66`\x04a2\x07V[a\x11\x84V[`@Qa\x03m\x96\x95\x94\x93\x92\x91\x90a4\x8DV[4\x80\x15a\x06\xF9W`\0\x80\xFD[Pa\x05\xEEa\x12\x0CV[4\x80\x15a\x07\x0EW`\0\x80\xFD[Pa\x04Ya\x07\x1D6`\x04a2\x07V[a\x12WV[4\x80\x15a\x07.W`\0\x80\xFD[P`\xFETa\x07<\x90`\xFF\x16\x81V[`@Qa\x03m\x91\x90a4\xCFV[a\x03?a\x12yV[4\x80\x15a\x07]W`\0\x80\xFD[Pa\x07qa\x07l6`\x04a2\x07V[a\x12\x8DV[`@Qa\x03m\x94\x93\x92\x91\x90a4\xE2V[4\x80\x15a\x07\x8DW`\0\x80\xFD[Pa\x04Ya\x07\x9C6`\x04a5\x0BV[a\x12\xFFV[4\x80\x15a\x07\xADW`\0\x80\xFD[Pa\x04Ya\x07\xBC6`\x04a2\x07V[a\x133V[4\x80\x15a\x07\xCDW`\0\x80\xFD[Pa\x03?a\x07\xDC6`\x04a2uV[a\x13>V[4\x80\x15a\x07\xEDW`\0\x80\xFD[Pa\x04\x11a\x07\xFC6`\x04a2\x07V[a\x01\x02` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x08$W`\0\x80\xFD[Pa\x04\x11`\x01\x81V[4\x80\x15a\x089W`\0\x80\xFD[Pa\x04Y`\xFDT\x81V[4\x80\x15a\x08OW`\0\x80\xFD[Pa\x03?a\x08^6`\x04a54V[a\x13cV[4\x80\x15a\x08oW`\0\x80\xFD[Pa\x01\x03Ta\x04YV[a\x08\x81a\x15fV[`\x02`\x97T\x03a\x08\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a5yV[`@Q\x80\x91\x03\x90\xFD[`\x02`\x97Ua\x08\xBD\x84\x84\x84\x84a\x15\xACV[PP`\x01`\x97UPPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x08\xF9WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[`\x02`\x97T\x03a\t!W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a5yV[`\x02`\x97Ua\t.a\x15fV[a\x08\xBD\x84\x84\x84\x84a\x16\rV[`\x02`\x97T\x03a\t\\W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a5yV[`\x02`\x97Ua\tia\x15fV[a\tt\x83\x83\x83a\x17\xFDV[PP`\x01`\x97UPV[a\t\x86a\x15fV[`\0\x80Q` a9U\x839\x81Q\x91Ra\t\x9E\x81a\x19\xD3V[a\t\xA8\x83\x83a\x19\xDDV[PPPV[`\x02`\x97T\x03a\t\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a5yV[`\x02`\x97Ua\t\xDCa\x15fV[a\t\xE5\x81a\x1BFV[P`\x01`\x97UV[`\x02`\x97T\x03a\n\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a5yV[`\x02`\x97Ua\n\x1Ca\x15fV[a\x08\xBD\x84\x84\x84\x84a\x1D\xBDV[`\0\x82\x81R`e` R`@\x90 `\x01\x01Ta\nC\x81a\x19\xD3V[a\t\xA8\x83\x83a\x1D\xE5V[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\n\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\x08\xA3V[a\n\xC7\x82\x82a\x1EkV[PPV[`\0a\n\xD6\x81a\x19\xD3V[a\n\xDEa\x1E\xD2V[PV[`\x02`\x97T\x03a\x0B\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a5yV[`\x02`\x97Ua\x0B\x10a\x15fV[a\x0B\x1C\x82\x82`\0a\x17\xFDV[PP`\x01`\x97UV[a\x0B-a\x15fV[a\n\xDE\x81a\x1F$V[a\x0B[`@\x80Q``\x81\x01\x90\x91R\x80`\0\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@\x80Q``\x81\x01\x90\x91R`\xFET`\0\x91\x90\x81\x90`\xFF\x16`\x01\x81\x11\x15a\x0B\x83Wa\x0B\x83a2\xEDV[\x81R` \x01`\0`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\xF8W\x81` \x01[`@\x80Qa\x01\0\x81\x01\x82R`\0`\xC0\x82\x01\x81\x81R`\xE0\x83\x01\x82\x90R\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x0B\xA8W\x90P[P\x81R` \x01`\0`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C_W\x81` \x01[`@\x80Q`\xC0\x81\x01\x82R`\0`\x80\x82\x01\x81\x81R`\xA0\x83\x01\x82\x90R\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x0C\x1EW\x90P[P\x90R\x90P\x83\x15\x80\x15a\x0CpWP\x82\x15[\x15a\x0C|W\x90Pa\x08\xF9V[`\0\x80\x85[\x85\x81\x11a\r\x02W`\0\x81\x81Ra\x01\0` R`@\x90 `\x01\x01T\x15a\x0C\xB0Wa\x0C\xA9\x83a5\xDCV[\x92Pa\x0C\xFAV[`\0\x81\x81R`\xFF` R`@\x90 `\x01\x01T\x15a\x0C\xD7Wa\x0C\xD0\x82a5\xDCV[\x91Pa\x0C\xFAV[`@QcT\xB4\x96\x0F`\xE1\x1B\x81R`\x04\x81\x01\x88\x90R`$\x81\x01\x87\x90R`D\x01a\x08\xA3V[`\x01\x01a\x0C\x81V[P\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x1CWa\r\x1Ca5\xB0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\x8AW\x81` \x01[`@\x80Qa\x01\0\x81\x01\x82R`\0`\xC0\x82\x01\x81\x81R`\xE0\x83\x01\x82\x90R\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\r:W\x90P[P` \x84\x01R\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xA9Wa\r\xA9a5\xB0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\x08W\x81` \x01[`@\x80Q`\xC0\x81\x01\x82R`\0`\x80\x82\x01\x81\x81R`\xA0\x83\x01\x82\x90R\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\r\xC7W\x90P[P`@\x84\x01RP`\0\x90P\x80\x85[\x85\x81\x11a\x0F\xCFW`\0\x81\x81Ra\x01\0` R`@\x90 `\x01\x01T\x15a\x0F\x05W`\0\x81\x81Ra\x01\0` \x81\x90R`@\x91\x82\x90 \x82Q\x91\x82\x01\x90\x92R\x81T\x90\x91\x90\x82\x90`\xC0\x82\x01\x90\x83\x90\x82\x90`\xFF\x16`\x01\x81\x11\x15a\x0EtWa\x0Eta2\xEDV[`\x01\x81\x11\x15a\x0E\x85Wa\x0E\x85a2\xEDV[\x81R`\x01\x91\x90\x91\x01T` \x91\x82\x01R\x90\x82R`\x02\x83\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83\x83\x01R`\x03\x84\x01T\x16`@\x83\x01R`\x04\x83\x01T``\x83\x01R`\x05\x83\x01T`\x80\x83\x01R`\x06\x90\x92\x01T`\xA0\x90\x91\x01R\x85\x01Q\x84a\x0E\xE3\x81a5\xDCV[\x95P\x81Q\x81\x10a\x0E\xF5Wa\x0E\xF5a5\xF5V[` \x02` \x01\x01\x81\x90RPa\x0F\xC7V[`\0\x81\x81R`\xFF` R`@\x90 `\x02\x01T\x15a\x0F\xC2W`\0\x81\x81R`\xFF` \x81\x90R`@\x91\x82\x90 \x82Q`\xC0\x81\x01\x90\x93R\x80T\x90\x91\x83\x91`\x80\x83\x01\x91\x84\x91\x83\x91\x16`\x01\x81\x11\x15a\x0FXWa\x0FXa2\xEDV[`\x01\x81\x11\x15a\x0FiWa\x0Fia2\xEDV[\x81R`\x01\x91\x90\x91\x01T` \x91\x82\x01R\x90\x82R`\x02\x83\x01T\x90\x82\x01R`\x03\x82\x01T`\xFF\x16\x15\x15`@\x80\x83\x01\x91\x90\x91R`\x04\x90\x92\x01T``\x90\x91\x01R\x85\x01Q\x83a\x0F\xB0\x81a5\xDCV[\x94P\x81Q\x81\x10a\x0E\xF5Wa\x0E\xF5a5\xF5V[a\x0F\xCFV[`\x01\x01a\x0E\x16V[P\x91\x95\x94PPPPPV[`\0a\x0F\xE5\x81a\x19\xD3V[a\n\xDEa \xBAV[`\0\x80`@Q` \x01a\x10\0\x91\x90a6\x0BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x82`@Q` \x01a\x10 \x91\x90a6@V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x10>\x92\x91` \x01a6\xC5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x91\x82R`e` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0a\x10\x91\x81a\x19\xD3V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x10\xBBW`@Q`\x01by\xC3]`\xE0\x1B\x03\x19\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFETa\x10\xE5\x90`\0\x80Q` a9U\x839\x81Q\x91R\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16a\x1EkV[a\x10\xFD`\0\x80Q` a9U\x839\x81Q\x91R\x83a\x1D\xE5V[`\xFE\x80Ta\x01\0`\x01`\xA8\x1B\x03\x19\x16a\x01\0`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U`@Q\x7F\x1B\x0F/P\r\xF5\x96\xB4+s\xE8\r\xBE\xC6\xA1\xFBW\x0F\x01\x97\x8AXg#\xF9\x88\xA5\xFCT\xD7s\xA1\x90`\0\x90\xA2PPV[`\0`\x02`@Q` \x01a\x11d\x91\x90a6\x0BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x82`@Q` \x01a\x10 \x91\x90a6\xF4V[a\x01\0` R`\0\x90\x81R`@\x90\x81\x90 \x81Q\x80\x83\x01\x90\x92R\x80T\x90\x91\x90\x82\x90\x82\x90`\xFF\x16`\x01\x81\x11\x15a\x11\xBAWa\x11\xBAa2\xEDV[`\x01\x81\x11\x15a\x11\xCBWa\x11\xCBa2\xEDV[\x81R`\x01\x91\x90\x91\x01T` \x90\x91\x01R`\x02\x82\x01T`\x03\x83\x01T`\x04\x84\x01T`\x05\x85\x01T`\x06\x90\x95\x01T\x93\x94`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x94\x92\x90\x93\x16\x92\x90\x91\x86V[a\x121`@\x80Q``\x81\x01\x90\x91R\x80`\0\x81R` \x01``\x81R` \x01``\x81RP\x90V[a\x12R`\xFCT`\x01a\x12C\x91\x90a7,V[`\x01`\xFBTa\x05\xE9\x91\x90a7DV[\x90P\x90V[a\x01\x03\x81\x81T\x81\x10a\x12hW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T\x90P\x81V[a\x12\x81a\x15fV[a\x12\x8B`\0a\x1F$V[V[`\xFF` \x81\x90R`\0\x91\x82R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x90\x92\x91\x83\x91\x83\x91\x16`\x01\x81\x11\x15a\x12\xC2Wa\x12\xC2a2\xEDV[`\x01\x81\x11\x15a\x12\xD3Wa\x12\xD3a2\xEDV[\x81R`\x01\x91\x90\x91\x01T` \x90\x91\x01R`\x02\x82\x01T`\x03\x83\x01T`\x04\x90\x93\x01T\x91\x92\x90\x91`\xFF\x90\x91\x16\x90\x84V[`\0`\x01`@Q` \x01a\x13\x13\x91\x90a6\x0BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x82`@Q` \x01a\x10 \x91\x90a7[V[`\0a\x08\xF9\x82a \xF7V[`\0\x82\x81R`e` R`@\x90 `\x01\x01Ta\x13Y\x81a\x19\xD3V[a\t\xA8\x83\x83a\x1EkV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x13\x83WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x13\x9DWP0;\x15\x80\x15a\x13\x9DWP`\0T`\xFF\x16`\x01\x14[a\x14\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x08\xA3V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x14#W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x14+a\"\x1AV[a\x143a\"\x1AV[a\x14;a\"AV[a\x14Ca\"pV[`\x01`\x01`\xA0\x1B\x03\x84\x16a\x14jW`@Qc9D\xED\x87`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14u`\0\x85a\x1D\xE5V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x14\x9FW`@Q`\x01by\xC3]`\xE0\x1B\x03\x19\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14\xB7`\0\x80Q` a9U\x839\x81Q\x91R\x83a\x1D\xE5V[`\xFE\x80Ta\x01\0`\x01`\xA8\x1B\x03\x19\x81\x16a\x01\0`\x01`\x01`\xA0\x1B\x03\x86\x16\x02\x90\x81\x17\x83U`\x01`\xFB\x81\x90U`\0`\xFC\x81\x90U`\xFDU\x86\x93\x92`\x01`\x01`\xA8\x1B\x03\x19\x16`\xFF\x19\x90\x92\x16\x91\x90\x91\x17\x90\x83\x81\x81\x11\x15a\x15\x14Wa\x15\x14a2\xEDV[\x02\x17\x90UP\x80\x15a\x15`W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01[`@Q\x80\x91\x03\x90\xA1[PPPPV[`\xC9T`\xFF\x16\x15a\x12\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Ro\x14\x18]\\\xD8X\x9B\x19N\x88\x1C\x18]\\\xD9Y`\x82\x1B`D\x82\x01R`d\x01a\x08\xA3V[`\0a\x15\xB7\x85a\x12\xFFV[\x90Pa\x15\xCA` \x86\x015\x82\x86\x86\x86a\"\x9FV[a\x15\xD4\x85\x82a$\x90V[`\0\x90\x81Ra\x01\x02` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16s\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x17\x90UPPPPV[`\0a\x16\x18\x85a\x0F\xEDV[\x90Pa\x16+` \x86\x015\x82\x86\x86\x86a\"\x9FV[`\0\x81\x81Ra\x01\x02` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16s\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x17\x90\x91U`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\x16\xFFW`\x01a\x16\x81`\x80\x88\x01``\x89\x01a4VV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x16\xB2Wa\x16\xAD\x81a\x16\xA3`\x80\x89\x01``\x8A\x01a4VV[\x88`\x80\x015a%\xF6V[a\x16\xC0V[a\x16\xC0\x81\x87`\x80\x015a&wV[`@Q\x82\x81R` \x80\x88\x015\x91\x7F)\x96\xFDTl7\xD7L\x17\x04f\xEAj\xA4\xA3\x08\xE3\xCA-J\xA6\x89\xE6\xE9\xE3)\x94\xDBP9\xCC\x0E\x91\x01`@Q\x80\x91\x03\x90\xA2PPa\x15`V[`\0a\x17\x13`\xA0\x88\x015`\x80\x89\x015a7DV[\x90P`\x01a\x17'`\x80\x89\x01``\x8A\x01a4VV[`\x01`\x01`\xA0\x1B\x03\x16\x03a\x17kWa\x17Na\x17H``\x89\x01`@\x8A\x01a4VV[\x82a&wV[`\xA0\x87\x015\x15a\x17fWa\x17f3\x88`\xA0\x015a&wV[a\x17\xBCV[a\x17\x94a\x17~``\x89\x01`@\x8A\x01a4VV[a\x17\x8E`\x80\x8A\x01``\x8B\x01a4VV[\x83a%\xF6V[`\xA0\x87\x015\x15a\x17\xBCWa\x17\xBC3a\x17\xB2`\x80\x8A\x01``\x8B\x01a4VV[\x89`\xA0\x015a%\xF6V[`@Q\x83\x81R` \x80\x89\x015\x91\x7F\x93_&\xD9K\xE3\x19\x07\x08\n\xA7\x8B>n*\xC6\xD4\x8A\x07*\xF0\x96\xC2\x02h8\x86!\xBB\xC1\x17\x89\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPPPPV[\x81\x81\x81`\0\x03a\x18 W`@Qc\x1F* \x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81\x11\x15a\x18KW`@Qc +1i`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x83\x90R`D\x01a\x08\xA3V[`\x01`\x01`\xA0\x1B\x03\x85\x16a\x18rW`@Qc\xAD\x19\x91\xF5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x80`\xC0\x01`@R\x80a\x18\x89`\0a&\xE5V[\x81R` \x013`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x88\x16` \x80\x83\x01\x91\x90\x91R`@\x80\x83\x01\x89\x90RB``\x84\x01R`\x80\x90\x92\x01\x87\x90R\x82Q\x81\x01Q`\0\x90\x81Ra\x01\0\x90\x91R \x81Q\x80Q\x82T\x93\x94P\x84\x93\x83\x91\x90\x82\x90`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a\x18\xF5Wa\x18\xF5a2\xEDV[\x02\x17\x90UP` \x91\x82\x01Q`\x01\x91\x90\x91\x01U\x82\x01Q`\x02\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`@\x84\x01Q`\x03\x84\x01\x80T\x90\x92\x16\x90\x83\x16\x17\x90U``\x83\x01Q`\x04\x83\x01U`\x80\x83\x01Q`\x05\x83\x01U`\xA0\x90\x92\x01Q`\x06\x90\x91\x01U\x86\x163`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q` \x01Q\x7F\"S\x05\xEC\xB6o\xA9\xB9\xB2\x9F\x8D\xEA\xD9\xBA\xEA6Zl\"]c\x9D\xFD\x86nx,\xCBc\xE2\xF0[\x88\x88`@Qa\x19\xAE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA4a\x19\xCB`\x01`\x01`\xA0\x1B\x03\x87\x1630\x88a':V[PPPPPPV[a\n\xDE\x813a'\xA5V[\x805`\0\x03a\x19\xFFW`@Qci\xF1\xCF\xEF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x015\x815\x11\x15a\x1A2W`@Qcr/\xC3\xF7`\xE1\x1B\x81R\x815`\x04\x82\x01R` \x82\x015`$\x82\x01R`D\x01a\x08\xA3V[`\xFDTa\x1AA`\x01\x835a7DV[\x11\x15a\x1AnW`\xFDT`@Qc\x06P\x04s`\xE5\x1B\x81R\x825`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`D\x01a\x08\xA3V[`\xFDT\x81` \x015\x11a\x1A\xA5W`\xFDT`@QcP\xA7\x92\xB1`\xE0\x1B\x81R` \x83\x015`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`D\x01a\x08\xA3V[a\x01\x03\x80T`\x01\x81\x01\x90\x91U\x7F\x02\xC2\x97\xABt\xAA\xD0\xAE\xDE:\x18\x95\xC8W\xB1\xF2\xC7\x1Ej ?\xEBr{\xEC\x95\xACu)\x98\xCBx\x01\x82\x90U`\0\x82\x81Ra\x01\x01` R`@\x90 \x81\x90a\x1A\xFE\x82\x82\x815\x81U` \x82\x015`\x01\x82\x01UPPV[PP` \x81\x015`\xFDU`@Q\x7FI\xC1X\xD4\x90\xDB\x9E\x06o\x01\xB5\xD4\xF1\xA0\x94HZe\x98\xCBlR\x96\xB4\xC0~F\xC1*\x1D\xC1\x1C\x90a\x1B:\x90\x84\x90\x84\x90a7\x91V[`@Q\x80\x91\x03\x90\xA1PPV[\x80`\x80\x015\x81`\xA0\x015\x81`\0\x03a\x1BqW`@Qc\x1F* \x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81\x11\x15a\x1B\x9CW`@Qc +1i`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x83\x90R`D\x01a\x08\xA3V[`\0a\x1B\xAE``\x85\x01`@\x86\x01a4VV[`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1B\xD5W`@Qc\xD2{DC`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1B\xE0\x84a\x0F\xEDV[`\0\x81\x81Ra\x01\x02` R`@\x90 T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16\x15a\x1C\x1DW`@Qc\xFE\xA5\x94S`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x08\xA3V[`\0\x81\x81Ra\x01\x02` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90Ua\x1CN`\xA0\x86\x015`\x80\x87\x015a7DV[\x90P`\x01a\x1Cb`\x80\x87\x01``\x88\x01a4VV[`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1D%W\x804\x14a\x1C\x99W`@QcL\xEA\xF5\xD3`\xE1\x1B\x81R4`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x01a\x08\xA3V[3a\x1C\xAA``\x87\x01`@\x88\x01a4VV[`@\x80Q` \x80\x8A\x015\x82R\x81\x01\x85\x90R\x90\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90\x7Fz\x9A\xBD\x9E\xB8k\xDB\xCA\x89\xCB\xA4\x06\x9Ac,7\xD9=\xB8.>\x14\xAD\x81\x19\xA3\xA7\x81(\x14\x85>\x90``\x01`@Q\x80\x91\x03\x90\xA3a\x1D\x1E\x81a\x1D\x0F``\x88\x01`@\x89\x01a4VV[`\x01`\x01`\xA0\x1B\x03\x16\x90a(\tV[PPPPPV[3a\x1D6``\x87\x01`@\x88\x01a4VV[`@\x80Q` \x80\x8A\x015\x82R\x81\x01\x85\x90R\x90\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90\x7Fz\x9A\xBD\x9E\xB8k\xDB\xCA\x89\xCB\xA4\x06\x9Ac,7\xD9=\xB8.>\x14\xAD\x81\x19\xA3\xA7\x81(\x14\x85>\x90``\x01`@Q\x80\x91\x03\x90\xA3a\x1D\x1E3a\x1D\x9B``\x88\x01`@\x89\x01a4VV[\x83a\x1D\xAC`\x80\x8A\x01``\x8B\x01a4VV[`\x01`\x01`\xA0\x1B\x03\x16\x92\x91\x90a':V[`\0a\x1D\xC8\x85a\x11PV[\x90Pa\x1D\xDB` \x86\x015\x82\x86\x86\x86a\"\x9FV[a\x15\xD4\x85\x82a)\"V[a\x1D\xEF\x82\x82a\x10[V[a\n\xC7W`\0\x82\x81R`e` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x1E'3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[a\x1Eu\x82\x82a\x10[V[\x15a\n\xC7W`\0\x82\x81R`e` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[a\x1E\xDAa*\x07V[`\xC9\x80T`\xFF\x19\x16\x90U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA3[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA1V[4\x81\x81`\0\x03a\x1FGW`@Qc\x1F* \x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81\x11\x15a\x1FrW`@Qc +1i`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x83\x90R`D\x01a\x08\xA3V[`\0`@Q\x80`\xC0\x01`@R\x80a\x1F\x89`\0a&\xE5V[\x81R3` \x80\x83\x01\x91\x90\x91R`\x01`@\x80\x84\x01\x82\x90R4``\x85\x01RB`\x80\x85\x01R`\xA0\x90\x93\x01\x88\x90R\x83Q\x82\x01Q`\0\x90\x81Ra\x01\0\x90\x92R\x91\x90 \x82Q\x80Q\x82T\x94\x95P\x85\x94\x92\x93\x91\x92\x84\x92\x83\x91`\xFF\x19\x16\x90\x83\x81\x81\x11\x15a\x1F\xEFWa\x1F\xEFa2\xEDV[\x02\x17\x90UP` \x91\x82\x01Q`\x01\x91\x82\x01U\x90\x83\x01Q`\x02\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`@\x85\x01Q`\x03\x85\x01\x80T\x91\x90\x93\x16\x91\x16\x17\x90U``\x83\x01Q`\x04\x83\x01U`\x80\x83\x01Q`\x05\x83\x01U`\xA0\x90\x92\x01Q`\x06\x90\x91\x01Ua `3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q` \x01Q\x7F\"S\x05\xEC\xB6o\xA9\xB9\xB2\x9F\x8D\xEA\xD9\xBA\xEA6Zl\"]c\x9D\xFD\x86nx,\xCBc\xE2\xF0[4\x88`@Qa \xAC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA4PPPPV[a \xC2a\x15fV[`\xC9\x80T`\xFF\x19\x16`\x01\x17\x90U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa\x1F\x073\x90V[`\0`\xFDT\x82\x11\x15a!\x1FW`@Qcd\xB4\xF0y`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x08\xA3V[a\x01\x03T`\0\x81\x90\x03a!EW`@Qc]Cpu`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80[\x80\x15a!\xD1W`\0a\x01\x03a!]`\x01\x84a7DV[\x81T\x81\x10a!mWa!ma5\xF5V[`\0\x91\x82R` \x80\x83 \x90\x91\x01T\x80\x83Ra\x01\x01\x82R`@\x92\x83\x90 \x83Q\x80\x85\x01\x90\x94R\x80T\x80\x85R`\x01\x90\x91\x01T\x92\x84\x01\x92\x90\x92R\x92P\x86\x10\x80\x15\x90a!\xB8WP\x80` \x01Q\x86\x11\x15[\x15a!\xC6WP\x94\x93PPPPV[PP`\0\x19\x01a!GV[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FBatch with request not found\0\0\0\0`D\x82\x01R`d\x01a\x08\xA3V[`\0Ta\x01\0\x90\x04`\xFF\x16a\x12\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a7\xAFV[`\0Ta\x01\0\x90\x04`\xFF\x16a\"hW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a7\xAFV[a\x12\x8Ba*PV[`\0Ta\x01\0\x90\x04`\xFF\x16a\"\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a7\xAFV[a\x12\x8Ba*\x83V[`\0\x84\x81Ra\x01\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16s\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x10\x19\x01a\"\xEFW`@Qc\xE9\x97\x11\xF1`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01a\x08\xA3V[`\0\x83\x81Ra\x01\x01` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x80\x84R`\x01\x90\x91\x01T\x91\x83\x01\x91\x90\x91R\x15\x80a#*WP` \x81\x01Q\x15[\x15a#HW`@Qc9\x07[\xA1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q` \x82\x01Q\x10\x15a#~W\x80Q` \x82\x01Q`@QcT\xB4\x96\x0F`\xE1\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x08\xA3V[\x80Q\x86\x10\x80a#\x90WP\x80` \x01Q\x86\x11[\x15a#\xC5W\x80Q` \x82\x01Q`@QcM4n\x89`\xE0\x1B\x81R`\x04\x81\x01\x89\x90R`$\x81\x01\x92\x90\x92R`D\x82\x01R`d\x01a\x08\xA3V[\x80Q` \x82\x01Q`\0\x91a#\xD8\x91a7DV[a#\xE3\x90`\x01a7,V[\x90Pc\xFF\xFF\xFF\xFF\x81\x11\x15a$\rW`@Qc \x95\xA5=`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x08\xA3V[\x81Q`\0\x90a$\x1C\x90\x89a7DV[\x90P`\0a$`\x88\x83\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92Pa*\xB1\x91PPV[\x90P\x80\x87\x14a$\x85W`@Qc\xF6\xAE\x8DS`\xE0\x1B\x81R`\x04\x81\x01\x88\x90R`$\x01a\x08\xA3V[PPPPPPPPPV[`\0`\x01`\xFBTa$\xA1\x91\x90a7DV[``\x84\x015\x11\x15a$\xB4WP`\x01a$\xFCV[`\0a$\xC8`@\x85\x015``\x86\x015a\x0B6V[\x90P\x80`@Q` \x01a$\xDB\x91\x90a3\x93V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\x80\x015\x14\x15\x91PP[`\0`@Q\x80`\x80\x01`@R\x80a%\x13`\0a&\xE5V[\x81R` \x86\x81\x015\x81\x83\x01R\x84\x15\x15`@\x80\x84\x01\x91\x90\x91RB``\x90\x93\x01\x92\x90\x92R\x82Q\x81\x01Q`\0\x90\x81R`\xFF\x90\x91R \x81Q\x80Q\x82T\x93\x94P\x84\x93\x83\x91\x90\x82\x90`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a%lWa%la2\xEDV[\x02\x17\x90UP` \x91\x82\x01Q`\x01\x91\x90\x91\x01U\x82\x81\x01Q`\x02\x83\x01U`@\x80\x84\x01Q`\x03\x84\x01\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90U``\x90\x93\x01Q`\x04\x90\x92\x01\x91\x90\x91U\x82\x81\x01Q\x83\x83\x01Q\x83Q\x90\x15\x15\x81R\x91\x82\x01\x86\x90R\x91\x7F\x9E\xF1\x13S\xAF\xD9}3\x9Aws(P\xB7\xC2'\x04eeX\xD9\xBAc\xCC~2\x1E\n\xC4\xC2\n\xA9\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPV[\x80`\0\x03a&\x17W`@Qc)\xC5D)`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16~\xE7c\xF7w\x8B\x8C\xEE\xF7'\x0C\x89\xB7\xD1\xDF\x10\x08\xB0\xE4\x82\xDA9\xC481Aw3\xAF\x96\xFB\r\x83`@Qa&[\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3a\t\xA8`\x01`\x01`\xA0\x1B\x03\x83\x16\x84\x83a*\xFFV[\x80`\0\x03a&\x98W`@Qc)\xC5D)`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\xE0ISU\xC1\xE0LQ%\x84R\x18T\xD2\"\xD29\xA4\xB7\x82\xB3\x9A\xC8\xA7\xE85\xA3O^\xC7\xC1\xE1\x82`@Qa&\xD3\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2a\n\xC7\x82\x82a(\tV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`@Q\x80`@\x01`@R\x80\x83`\x01\x81\x11\x15a'\x16Wa'\x16a2\xEDV[\x81R` \x01`\xFB`\0\x81T\x80\x92\x91\x90a'.\x90a5\xDCV[\x90\x91UP\x90R\x92\x91PPV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x15`\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra+/V[a'\xAF\x82\x82a\x10[V[a\n\xC7Wa'\xC7\x81`\x01`\x01`\xA0\x1B\x03\x16`\x14a,\x01V[a'\xD2\x83` a,\x01V[`@Q` \x01a'\xE3\x92\x91\x90a7\xFAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x08\xA3\x91`\x04\x01a8oV[\x80G\x10\x15a(YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: insufficient balance\0\0\0`D\x82\x01R`d\x01a\x08\xA3V[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a(\xA6W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a(\xABV[``\x91P[PP\x90P\x80a\t\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FAddress: unable to send value, r`D\x82\x01R\x7Fecipient may have reverted\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\xA3V[`@\x80\x83\x015`\0\x90\x81Ra\x01\0` R\x90\x81 `\x02\x81\x01T\x90\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a)Y`\x80\x86\x01``\x87\x01a4VV[`\x01`\x01`\xA0\x1B\x03\x16\x14a)zWa)w`\x80\x85\x01``\x86\x01a4VV[\x90P[`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x16`\x01\x14a)\xB5W`\x03\x82\x01T`\x04\x83\x01Ta)\xB0\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a%\xF6V[a)\xC3V[a)\xC3\x81\x83`\x04\x01Ta&wV[`@\x80Q` \x86\x81\x015\x82R\x86\x83\x015\x90\x82\x01R\x90\x81\x01\x84\x90R\x7F\x13u\x0Cs\x1F\x87\xC1RB\x87L\xE7K\xF4d\x95\x02\xCC\x8E|\x82\x90g\xCE\x84e\x05\xAC\xDB\x96(\x9D\x90``\x01a\x15WV[`\xC9T`\xFF\x16a\x12\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x14\x18]\\\xD8X\x9B\x19N\x88\x1B\x9B\xDD\x08\x1C\x18]\\\xD9Y`b\x1B`D\x82\x01R`d\x01a\x08\xA3V[`\0Ta\x01\0\x90\x04`\xFF\x16a*wW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a7\xAFV[`\xC9\x80T`\xFF\x19\x16\x90UV[`\0Ta\x01\0\x90\x04`\xFF\x16a*\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a7\xAFV[`\x01`\x97UV[`\0\x80\x82[\x80\x15a*\xDBWa*\xC7`\x02\x82a8\xB8V[\x90Pa*\xD4`\x01\x83a7,V[\x91Pa*\xB6V[a*\xF4\x82\x87\x89\x88`\0a*\xEF`\x01\x8Ba7DV[a-\xA4V[\x97\x96PPPPPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\t\xA8\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a'nV[`\0a+\x84\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a.\xB4\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\t\xA8W\x80\x80` \x01\x90Q\x81\x01\x90a+\xA2\x91\x90a8\xCCV[a\t\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x08\xA3V[```\0a,\x10\x83`\x02a8\xEEV[a,\x1B\x90`\x02a7,V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,3Wa,3a5\xB0V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a,]W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a,xWa,xa5\xF5V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a,\xA7Wa,\xA7a5\xF5V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a,\xCB\x84`\x02a8\xEEV[a,\xD6\x90`\x01a7,V[\x90P[`\x01\x81\x11\x15a-NWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a-\nWa-\na5\xF5V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a- Wa- a5\xF5V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a-G\x81a9\rV[\x90Pa,\xD9V[P\x83\x15a-\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x08\xA3V[\x93\x92PPPV[`\0a-\xB1`\x02\x87a9$V[`\0\x03a.\x1CW\x85\x82\x14a.uW\x84\x84\x84a-\xCB\x81a5\xDCV[\x95P\x81Q\x81\x10a-\xDDWa-\xDDa5\xF5V[` \x02` \x01\x01Q`@Q` \x01a-\xFF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x94Pa.uV[\x83\x83a.'\x81a5\xDCV[\x94P\x81Q\x81\x10a.9Wa.9a5\xF5V[` \x02` \x01\x01Q\x85`@Q` \x01a.\\\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x94P[\x86`\x01\x14a.\xA9Wa.\xA4a.\x8B`\x01\x89a7DV[a.\x96`\x02\x89a8\xB8V[\x87\x87\x87a*\xEF`\x02\x89a8\xB8V[a*\xF4V[P\x92\x95\x94PPPPPV[``a.\xC3\x84\x84`\0\x85a.\xCBV[\x94\x93PPPPV[``\x82G\x10\x15a/,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x08\xA3V[`\x01`\x01`\xA0\x1B\x03\x85\x16;a/\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x08\xA3V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa/\x9F\x91\x90a98V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a/\xDCW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a/\xE1V[``\x91P[P\x91P\x91Pa*\xF4\x82\x82\x86``\x83\x15a/\xFBWP\x81a-\x9DV[\x82Q\x15a0\x0BW\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x91\x90a8oV[`\0`\xA0\x82\x84\x03\x12\x15a07W`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a0OW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0gW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a0\x82W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\xE0\x85\x87\x03\x12\x15a0\x9FW`\0\x80\xFD[a0\xA9\x86\x86a0%V[\x93P`\xA0\x85\x015\x92P`\xC0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0\xCCW`\0\x80\xFD[a0\xD8\x87\x82\x88\x01a0=V[\x95\x98\x94\x97P\x95PPPPV[`\0` \x82\x84\x03\x12\x15a0\xF6W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a-\x9DW`\0\x80\xFD[`\0`\xC0\x82\x84\x03\x12\x15a07W`\0\x80\xFD[`\0\x80`\0\x80a\x01\0\x85\x87\x03\x12\x15a17W`\0\x80\xFD[a1A\x86\x86a1\x0EV[\x93P`\xC0\x85\x015\x92P`\xE0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0\xCCW`\0\x80\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a1{W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a1\x95W`\0\x80\xFD[a1\x9E\x84a1dV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0\x80\x82\x84\x03``\x81\x12\x15a1\xC7W`\0\x80\xFD[\x835\x92P`@`\x1F\x19\x82\x01\x12\x15a1\xDDW`\0\x80\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[`\0`\xC0\x82\x84\x03\x12\x15a1\xFDW`\0\x80\xFD[a-\x9D\x83\x83a1\x0EV[`\0` \x82\x84\x03\x12\x15a2\x19W`\0\x80\xFD[P5\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a07W`\0\x80\xFD[`\0\x80`\0\x80`\xC0\x85\x87\x03\x12\x15a2HW`\0\x80\xFD[a2R\x86\x86a2 V[\x93P`\x80\x85\x015\x92P`\xA0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0\xCCW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a2\x88W`\0\x80\xFD[\x825\x91Pa2\x98` \x84\x01a1dV[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a2\xB4W`\0\x80\xFD[a2\xBD\x83a1dV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a2\xDEW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x02\x81\x10a\n\xDEWa\n\xDEa2\xEDV[\x80Qa3\x1E\x81a3\x03V[\x82R` \x90\x81\x01Q\x91\x01RV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a3\x88W\x81Qa3T\x88\x82Qa3\x13V[\x80\x84\x01Q`@\x89\x81\x01\x91\x90\x91R\x81\x01Q\x15\x15``\x80\x8A\x01\x91\x90\x91R\x01Q`\x80\x88\x01R`\xA0\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a3?V[P\x94\x95\x94PPPPPV[`\0` \x80\x83R`\x80\x80\x84\x01\x85Qa3\xAA\x81a3\x03V[\x85\x84\x01R\x85\x83\x01Q```@\x80\x88\x01\x82\x90R\x82Q\x93\x84\x90R`\xA0\x93\x92\x86\x01\x92\x84\x89\x01\x90`\0[\x81\x81\x10\x15a4+W\x85Qa3\xE5\x84\x82Qa3\x13V[\x80\x8A\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x85\x87\x01R\x85\x82\x01Q\x16\x86\x85\x01R\x85\x81\x01Q\x89\x85\x01R\x88\x81\x01Q\x88\x85\x01R\x87\x01Q`\xC0\x84\x01R\x94\x88\x01\x94`\xE0\x90\x92\x01\x91`\x01\x01a3\xD0V[PP\x89\x82\x01Q\x89\x82\x03`\x1F\x19\x01\x84\x8B\x01R\x96Pa4H\x81\x88a3+V[\x9A\x99PPPPPPPPPPV[`\0` \x82\x84\x03\x12\x15a4hW`\0\x80\xFD[a-\x9D\x82a1dV[`\0`\x80\x82\x84\x03\x12\x15a4\x83W`\0\x80\xFD[a-\x9D\x83\x83a2 V[`\xE0\x81\x01a4\x9B\x82\x89a3\x13V[`\x01`\x01`\xA0\x1B\x03\x96\x87\x16`@\x83\x01R\x94\x90\x95\x16``\x86\x01R`\x80\x85\x01\x92\x90\x92R`\xA0\x84\x01R`\xC0\x90\x92\x01\x91\x90\x91R\x91\x90PV[` \x81\x01a4\xDC\x83a3\x03V[\x91\x90R\x90V[`\xA0\x81\x01a4\xF0\x82\x87a3\x13V[`@\x82\x01\x94\x90\x94R\x91\x15\x15``\x83\x01R`\x80\x90\x91\x01R\x91\x90PV[`\0`\xA0\x82\x84\x03\x12\x15a5\x1DW`\0\x80\xFD[a-\x9D\x83\x83a0%V[`\x02\x81\x10a\n\xDEW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a5IW`\0\x80\xFD[a5R\x84a1dV[\x92P` \x84\x015a5b\x81a5'V[\x91Pa5p`@\x85\x01a1dV[\x90P\x92P\x92P\x92V[` \x80\x82R`\x1F\x90\x82\x01R\x7FReentrancyGuard: reentrant call\0`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a5\xEEWa5\xEEa5\xC6V[P`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x81\x01`\x03\x83\x10a4\xDCWa4\xDCa2\xEDV[\x805a6*\x81a5'V[a63\x81a3\x03V[\x82R` \x90\x81\x015\x91\x01RV[`\xC0\x81\x01a6N\x82\x84a6\x1FV[a6Z`@\x84\x01a1dV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`@\x85\x01R\x80a6w``\x87\x01a1dV[\x16``\x85\x01RPP`\x80\x83\x015`\x80\x83\x01R`\xA0\x83\x015`\xA0\x83\x01R\x92\x91PPV[`\0[\x83\x81\x10\x15a6\xB4W\x81\x81\x01Q\x83\x82\x01R` \x01a6\x9CV[\x83\x81\x11\x15a\x15`WPP`\0\x91\x01RV[`\0\x83Qa6\xD7\x81\x84` \x88\x01a6\x99V[\x83Q\x90\x83\x01\x90a6\xEB\x81\x83` \x88\x01a6\x99V[\x01\x94\x93PPPPV[`\x80\x81\x01a7\x02\x82\x84a6\x1FV[`@\x83\x81\x015\x90\x83\x01R`\x01`\x01`\xA0\x1B\x03a7 ``\x85\x01a1dV[\x16``\x83\x01R\x92\x91PPV[`\0\x82\x19\x82\x11\x15a7?Wa7?a5\xC6V[P\x01\x90V[`\0\x82\x82\x10\x15a7VWa7Va5\xC6V[P\x03\x90V[`\xA0\x81\x01a7i\x82\x84a6\x1FV[a7\x83`@\x83\x01`@\x85\x01\x805\x82R` \x90\x81\x015\x91\x01RV[`\x80\x92\x83\x015\x91\x90\x92\x01R\x90V[\x82\x81R``\x81\x01a-\x9D` \x83\x01\x84\x805\x82R` \x90\x81\x015\x91\x01RV[` \x80\x82R`+\x90\x82\x01R\x7FInitializable: contract is not i`@\x82\x01Rjnitializing`\xA8\x1B``\x82\x01R`\x80\x01\x90V[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa82\x81`\x17\x85\x01` \x88\x01a6\x99V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa8c\x81`(\x84\x01` \x88\x01a6\x99V[\x01`(\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra8\x8E\x81`@\x85\x01` \x87\x01a6\x99V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a8\xC7Wa8\xC7a8\xA2V[P\x04\x90V[`\0` \x82\x84\x03\x12\x15a8\xDEW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a-\x9DW`\0\x80\xFD[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a9\x08Wa9\x08a5\xC6V[P\x02\x90V[`\0\x81a9\x1CWa9\x1Ca5\xC6V[P`\0\x19\x01\x90V[`\0\x82a93Wa93a8\xA2V[P\x06\x90V[`\0\x82Qa9J\x81\x84` \x87\x01a6\x99V[\x91\x90\x91\x01\x92\x91PPV\xFEs\xE5s\xF9VmaA\x8A4\xD5\xDE?\xF4\x93`\xF9\xC5\x1F\xEC7\xF7HeQg\x02\x90\xF6(]\xAB\xA2dipfsX\"\x12 \xD0\xAA{\xCF\x1BY\xB8\xEB\x1E\xBF\x01\xF0\xF8\x0CM\xD4h2\xC6E\xF0\xAF*m5l\x99\xD2\xA8\x85\xA6vdsolcC\0\x08\r\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361061031a5760003560e01c8063890e95ce116101ab578063ca9b21ae116100f7578063de70e0b811610095578063f26ee9d01161006f578063f26ee9d01461082d578063f35f9e4514610843578063f9ecd01e146107a1578063ff2bae861461086357600080fd5b8063de70e0b8146107e1578063df2ebdbb14610818578063dffbdd9f1461058057600080fd5b8063d16544f0116100d1578063d16544f0146104fe578063d1cb26b41461031f578063d547741f146107c1578063db6b52461461074957600080fd5b8063ca9b21ae14610751578063cc8c909f14610781578063ce2de1bc146107a157600080fd5b8063a217fddf11610164578063b15387061161013e578063b1538706146106ed578063c2b40ae414610702578063c763e5a114610722578063c87c22241461074957600080fd5b8063a217fddf14610686578063ae46db111461069b578063b02c43d0146106bb57600080fd5b8063890e95ce146106265780638e24e392146103b657806391d1485414610646578063950ac4871461046757806397feb926146104fe5780639d54f4191461066657600080fd5b80633f4ba83a1161026a578063608fc37a1161022357806371c54461116101fd57806371c54461146105a957806379e041f2146105ce5780637fd4f845146105fb5780638456cb591461061157600080fd5b8063608fc37a1461058057806361bc221a14610593578063676f536b146103d657600080fd5b80633f4ba83a146104c757806347e63380146104dc57806347e7ef24146104fe5780634bf5fec3146103765780634f48eedf1461051e5780635c975abb1461056857600080fd5b80630e2636a3116102d7578063248a9ca3116102b1578063248a9ca31461042957806325afc76a146104675780632f2ff15d1461048757806336568abe146104a757600080fd5b80630e2636a3146103e95780630efe6a8b1461039657806321425ee01461039657600080fd5b806301ef69661461031f57806301ffc9a71461034157806303ed49d31461037657806308aba1b21461039657806308f42d40146103b65780630cac57ab146103d6575b600080fd5b34801561032b57600080fd5b5061033f61033a366004613089565b610879565b005b34801561034d57600080fd5b5061036161035c3660046130e4565b6108c8565b60405190151581526020015b60405180910390f35b34801561038257600080fd5b5061033f610391366004613120565b6108ff565b3480156103a257600080fd5b5061033f6103b1366004613180565b61093a565b3480156103c257600080fd5b5061033f6103d13660046131b3565b61097e565b61033f6103e43660046131eb565b6109ad565b3480156103f557600080fd5b5061041173111111111111111111111111111111111111111181565b6040516001600160a01b03909116815260200161036d565b34801561043557600080fd5b50610459610444366004613207565b60009081526065602052604090206001015490565b60405190815260200161036d565b34801561047357600080fd5b5061033f610482366004613232565b6109ed565b34801561049357600080fd5b5061033f6104a2366004613275565b610a28565b3480156104b357600080fd5b5061033f6104c2366004613275565b610a4d565b3480156104d357600080fd5b5061033f610acb565b3480156104e857600080fd5b5061045960008051602061395583398151915281565b34801561050a57600080fd5b5061033f6105193660046132a1565b610ae1565b34801561052a57600080fd5b50610553610539366004613207565b610101602052600090815260409020805460019091015482565b6040805192835260208301919091520161036d565b34801561057457600080fd5b5060c95460ff16610361565b61033f61058e366004613207565b610b25565b34801561059f57600080fd5b5061045960fb5481565b3480156105b557600080fd5b5060fe546104119061010090046001600160a01b031681565b3480156105da57600080fd5b506105ee6105e93660046132cb565b610b36565b60405161036d9190613393565b34801561060757600080fd5b5061045960fc5481565b34801561061d57600080fd5b5061033f610fda565b34801561063257600080fd5b506104596106413660046131eb565b610fed565b34801561065257600080fd5b50610361610661366004613275565b61105b565b34801561067257600080fd5b5061033f610681366004613456565b611086565b34801561069257600080fd5b50610459600081565b3480156106a757600080fd5b506104596106b6366004613471565b611150565b3480156106c757600080fd5b506106db6106d6366004613207565b611184565b60405161036d9695949392919061348d565b3480156106f957600080fd5b506105ee61120c565b34801561070e57600080fd5b5061045961071d366004613207565b611257565b34801561072e57600080fd5b5060fe5461073c9060ff1681565b60405161036d91906134cf565b61033f611279565b34801561075d57600080fd5b5061077161076c366004613207565b61128d565b60405161036d94939291906134e2565b34801561078d57600080fd5b5061045961079c36600461350b565b6112ff565b3480156107ad57600080fd5b506104596107bc366004613207565b611333565b3480156107cd57600080fd5b5061033f6107dc366004613275565b61133e565b3480156107ed57600080fd5b506104116107fc366004613207565b610102602052600090815260409020546001600160a01b031681565b34801561082457600080fd5b50610411600181565b34801561083957600080fd5b5061045960fd5481565b34801561084f57600080fd5b5061033f61085e366004613534565b611363565b34801561086f57600080fd5b5061010354610459565b610881611566565b6002609754036108ac5760405162461bcd60e51b81526004016108a390613579565b60405180910390fd5b60026097556108bd848484846115ac565b505060016097555050565b60006001600160e01b03198216637965db0b60e01b14806108f957506301ffc9a760e01b6001600160e01b03198316145b92915050565b6002609754036109215760405162461bcd60e51b81526004016108a390613579565b600260975561092e611566565b6108bd8484848461160d565b60026097540361095c5760405162461bcd60e51b81526004016108a390613579565b6002609755610969611566565b6109748383836117fd565b5050600160975550565b610986611566565b60008051602061395583398151915261099e816119d3565b6109a883836119dd565b505050565b6002609754036109cf5760405162461bcd60e51b81526004016108a390613579565b60026097556109dc611566565b6109e581611b46565b506001609755565b600260975403610a0f5760405162461bcd60e51b81526004016108a390613579565b6002609755610a1c611566565b6108bd84848484611dbd565b600082815260656020526040902060010154610a43816119d3565b6109a88383611de5565b6001600160a01b0381163314610abd5760405162461bcd60e51b815260206004820152602f60248201527f416363657373436f6e74726f6c3a2063616e206f6e6c792072656e6f756e636560448201526e103937b632b9903337b91039b2b63360891b60648201526084016108a3565b610ac78282611e6b565b5050565b6000610ad6816119d3565b610ade611ed2565b50565b600260975403610b035760405162461bcd60e51b81526004016108a390613579565b6002609755610b10611566565b610b1c828260006117fd565b50506001609755565b610b2d611566565b610ade81611f24565b610b5b6040805160608101909152806000815260200160608152602001606081525090565b604080516060810190915260fe5460009190819060ff166001811115610b8357610b836132ed565b81526020016000604051908082528060200260200182016040528015610bf857816020015b6040805161010081018252600060c0820181815260e0830182905282526020808301829052928201819052606082018190526080820181905260a08201528252600019909201910181610ba85790505b5081526020016000604051908082528060200260200182016040528015610c5f57816020015b6040805160c08101825260006080820181815260a083018290528252602080830182905292820181905260608201528252600019909201910181610c1e5790505b509052905083158015610c70575082155b15610c7c5790506108f9565b600080855b858111610d02576000818152610100602052604090206001015415610cb057610ca9836135dc565b9250610cfa565b600081815260ff602052604090206001015415610cd757610cd0826135dc565b9150610cfa565b6040516354b4960f60e11b815260048101889052602481018790526044016108a3565b600101610c81565b508167ffffffffffffffff811115610d1c57610d1c6135b0565b604051908082528060200260200182016040528015610d8a57816020015b6040805161010081018252600060c0820181815260e0830182905282526020808301829052928201819052606082018190526080820181905260a08201528252600019909201910181610d3a5790505b5060208401528067ffffffffffffffff811115610da957610da96135b0565b604051908082528060200260200182016040528015610e0857816020015b6040805160c08101825260006080820181815260a083018290528252602080830182905292820181905260608201528252600019909201910181610dc75790505b506040840152506000905080855b858111610fcf576000818152610100602052604090206001015415610f05576000818152610100602081905260409182902082519182019092528154909190829060c08201908390829060ff166001811115610e7457610e746132ed565b6001811115610e8557610e856132ed565b81526001919091015460209182015290825260028301546001600160a01b03908116838301526003840154166040830152600483015460608301526005830154608083015260069092015460a09091015285015184610ee3816135dc565b955081518110610ef557610ef56135f5565b6020026020010181905250610fc7565b600081815260ff602052604090206002015415610fc257600081815260ff6020819052604091829020825160c08101909352805490918391608083019184918391166001811115610f5857610f586132ed565b6001811115610f6957610f696132ed565b815260019190910154602091820152908252600283015490820152600382015460ff16151560408083019190915260049092015460609091015285015183610fb0816135dc565b945081518110610ef557610ef56135f5565b610fcf565b600101610e16565b509195945050505050565b6000610fe5816119d3565b610ade6120ba565b600080604051602001611000919061360b565b604051602081830303815290604052826040516020016110209190613640565b60408051601f198184030181529082905261103e92916020016136c5565b604051602081830303815290604052805190602001209050919050565b60009182526065602090815260408084206001600160a01b0393909316845291905290205460ff1690565b6000611091816119d3565b6001600160a01b0382166110bb5760405160016279c35d60e01b0319815260040160405180910390fd5b60fe546110e5906000805160206139558339815191529061010090046001600160a01b0316611e6b565b6110fd60008051602061395583398151915283611de5565b60fe8054610100600160a81b0319166101006001600160a01b038516908102919091179091556040517f1b0f2f500df596b42b73e80dbec6a1fb570f01978a586723f988a5fc54d773a190600090a25050565b60006002604051602001611164919061360b565b6040516020818303038152906040528260405160200161102091906136f4565b6101006020526000908152604090819020815180830190925280549091908290829060ff1660018111156111ba576111ba6132ed565b60018111156111cb576111cb6132ed565b815260019190910154602090910152600282015460038301546004840154600585015460069095015493946001600160a01b03938416949290931692909186565b6112316040805160608101909152806000815260200160608152602001606081525090565b61125260fc546001611243919061372c565b600160fb546105e99190613744565b905090565b610103818154811061126857600080fd5b600091825260209091200154905081565b611281611566565b61128b6000611f24565b565b60ff6020819052600091825260409182902082518084019093528054909291839183911660018111156112c2576112c26132ed565b60018111156112d3576112d36132ed565b815260019190910154602090910152600282015460038301546004909301549192909160ff9091169084565b60006001604051602001611313919061360b565b60405160208183030381529060405282604051602001611020919061375b565b60006108f9826120f7565b600082815260656020526040902060010154611359816119d3565b6109a88383611e6b565b600054610100900460ff16158080156113835750600054600160ff909116105b8061139d5750303b15801561139d575060005460ff166001145b6114005760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084016108a3565b6000805460ff191660011790558015611423576000805461ff0019166101001790555b61142b61221a565b61143361221a565b61143b612241565b611443612270565b6001600160a01b03841661146a57604051633944ed8760e11b815260040160405180910390fd5b611475600085611de5565b6001600160a01b03821661149f5760405160016279c35d60e01b0319815260040160405180910390fd5b6114b760008051602061395583398151915283611de5565b60fe8054610100600160a81b031981166101006001600160a01b038616029081178355600160fb819055600060fc81905560fd558693926001600160a81b03191660ff1990921691909117908381811115611514576115146132ed565b02179055508015611560576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498906020015b60405180910390a15b50505050565b60c95460ff161561128b5760405162461bcd60e51b815260206004820152601060248201526f14185d5cd8589b194e881c185d5cd95960821b60448201526064016108a3565b60006115b7856112ff565b90506115ca60208601358286868661229f565b6115d48582612490565b60009081526101026020526040902080546001600160a01b03191673111111111111111111111111111111111111111117905550505050565b600061161885610fed565b905061162b60208601358286868661229f565b60008181526101026020526040902080546001600160a01b03198116731111111111111111111111111111111111111111179091556001600160a01b031680156116ff5760016116816080880160608901613456565b6001600160a01b0316146116b2576116ad816116a36080890160608a01613456565b88608001356125f6565b6116c0565b6116c0818760800135612677565b604051828152602080880135917f2996fd546c37d74c170466ea6aa4a308e3ca2d4aa689e6e9e32994db5039cc0e910160405180910390a25050611560565b600061171360a08801356080890135613744565b905060016117276080890160608a01613456565b6001600160a01b03160361176b5761174e6117486060890160408a01613456565b82612677565b60a08701351561176657611766338860a00135612677565b6117bc565b61179461177e6060890160408a01613456565b61178e60808a0160608b01613456565b836125f6565b60a0870135156117bc576117bc336117b260808a0160608b01613456565b8960a001356125f6565b604051838152602080890135917f935f26d94be31907080aa78b3e6e2ac6d48a072af096c20268388621bbc11789910160405180910390a250505050505050565b81818160000361182057604051631f2a200560e01b815260040160405180910390fd5b8181111561184b5760405163202b316960e21b815260048101829052602481018390526044016108a3565b6001600160a01b0385166118725760405163ad1991f560e01b815260040160405180910390fd5b60006040518060c0016040528061188960006126e5565b8152602001336001600160a01b03908116825288166020808301919091526040808301899052426060840152608090920187905282518101516000908152610100909152208151805182549394508493839190829060ff1916600183818111156118f5576118f56132ed565b0217905550602091820151600191909101558201516002820180546001600160a01b03199081166001600160a01b03938416179091556040840151600384018054909216908316179055606083015160048301556080830151600583015560a0909201516006909101558616336001600160a01b03168260000151602001517f225305ecb66fa9b9b29f8dead9baea365a6c225d639dfd866e782ccb63e2f05b88886040516119ae929190918252602082015260400190565b60405180910390a46119cb6001600160a01b03871633308861273a565b505050505050565b610ade81336127a5565b80356000036119ff576040516369f1cfef60e01b815260040160405180910390fd5b602081013581351115611a325760405163722fc3f760e11b815281356004820152602082013560248201526044016108a3565b60fd54611a4160018335613744565b1115611a6e5760fd54604051630650047360e51b81528235600482015260248101919091526044016108a3565b60fd54816020013511611aa55760fd546040516350a792b160e01b81526020830135600482015260248101919091526044016108a3565b6101038054600181019091557f02c297ab74aad0aede3a1895c857b1f2c71e6a203feb727bec95ac752998cb78018290556000828152610101602052604090208190611afe828281358155602082013560018201555050565b5050602081013560fd556040517f49c158d490db9e066f01b5d4f1a094485a6598cb6c5296b4c07e46c12a1dc11c90611b3a9084908490613791565b60405180910390a15050565b80608001358160a0013581600003611b7157604051631f2a200560e01b815260040160405180910390fd5b81811115611b9c5760405163202b316960e21b815260048101829052602481018390526044016108a3565b6000611bae6060850160408601613456565b6001600160a01b031603611bd55760405163d27b444360e01b815260040160405180910390fd5b6000611be084610fed565b600081815261010260205260409020549091506001600160a01b031615611c1d5760405163fea5945360e01b8152600481018290526024016108a3565b60008181526101026020526040812080546001600160a01b03191633179055611c4e60a08601356080870135613744565b90506001611c626080870160608801613456565b6001600160a01b031603611d2557803414611c9957604051634ceaf5d360e11b8152346004820152602481018290526044016108a3565b33611caa6060870160408801613456565b604080516020808a0135825281018590529081018590526001600160a01b0391909116907f7a9abd9eb86bdbca89cba4069a632c37d93db82e3e14ad8119a3a7812814853e9060600160405180910390a3611d1e81611d0f6060880160408901613456565b6001600160a01b031690612809565b5050505050565b33611d366060870160408801613456565b604080516020808a0135825281018590529081018590526001600160a01b0391909116907f7a9abd9eb86bdbca89cba4069a632c37d93db82e3e14ad8119a3a7812814853e9060600160405180910390a3611d1e33611d9b6060880160408901613456565b83611dac60808a0160608b01613456565b6001600160a01b031692919061273a565b6000611dc885611150565b9050611ddb60208601358286868661229f565b6115d48582612922565b611def828261105b565b610ac75760008281526065602090815260408083206001600160a01b03851684529091529020805460ff19166001179055611e273390565b6001600160a01b0316816001600160a01b0316837f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a45050565b611e75828261105b565b15610ac75760008281526065602090815260408083206001600160a01b0385168085529252808320805460ff1916905551339285917ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b9190a45050565b611eda612a07565b60c9805460ff191690557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa335b6040516001600160a01b03909116815260200160405180910390a1565b348181600003611f4757604051631f2a200560e01b815260040160405180910390fd5b81811115611f725760405163202b316960e21b815260048101829052602481018390526044016108a3565b60006040518060c00160405280611f8960006126e5565b8152336020808301919091526001604080840182905234606085015242608085015260a0909301889052835182015160009081526101009092529190208251805182549495508594929391928492839160ff1916908381811115611fef57611fef6132ed565b0217905550602091820151600191820155908301516002830180546001600160a01b039283166001600160a01b0319918216179091556040850151600385018054919093169116179055606083015160048301556080830151600583015560a0909201516006909101556120603390565b6001600160a01b03168260000151602001517f225305ecb66fa9b9b29f8dead9baea365a6c225d639dfd866e782ccb63e2f05b34886040516120ac929190918252602082015260400190565b60405180910390a450505050565b6120c2611566565b60c9805460ff191660011790557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a258611f073390565b600060fd5482111561211f576040516364b4f07960e11b8152600481018390526024016108a3565b61010354600081900361214557604051635d43707560e01b815260040160405180910390fd5b805b80156121d157600061010361215d600184613744565b8154811061216d5761216d6135f5565b600091825260208083209091015480835261010182526040928390208351808501909452805480855260019091015492840192909252925086108015906121b8575080602001518611155b156121c65750949350505050565b505060001901612147565b5060405162461bcd60e51b815260206004820152601c60248201527f426174636820776974682072657175657374206e6f7420666f756e640000000060448201526064016108a3565b600054610100900460ff1661128b5760405162461bcd60e51b81526004016108a3906137af565b600054610100900460ff166122685760405162461bcd60e51b81526004016108a3906137af565b61128b612a50565b600054610100900460ff166122975760405162461bcd60e51b81526004016108a3906137af565b61128b612a83565b600084815261010260205260409020546001600160a01b031673111111111111111111111111111111111111111019016122ef5760405163e99711f160e01b8152600481018590526024016108a3565b600083815261010160209081526040918290208251808401909352805480845260019091015491830191909152158061232a57506020810151155b15612348576040516339075ba160e21b815260040160405180910390fd5b80516020820151101561237e57805160208201516040516354b4960f60e11b8152600481019290925260248201526044016108a3565b80518610806123905750806020015186115b156123c55780516020820151604051634d346e8960e01b815260048101899052602481019290925260448201526064016108a3565b805160208201516000916123d891613744565b6123e390600161372c565b905063ffffffff81111561240d57604051632095a53d60e21b8152600481018290526024016108a3565b815160009061241c9089613744565b905060006124608883888880806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250899250612ab1915050565b90508087146124855760405163f6ae8d5360e01b8152600481018890526024016108a3565b505050505050505050565b6000600160fb546124a19190613744565b606084013511156124b4575060016124fc565b60006124c860408501356060860135610b36565b9050806040516020016124db9190613393565b60405160208183030381529060405280519060200120846080013514159150505b6000604051806080016040528061251360006126e5565b815260208681013581830152841515604080840191909152426060909301929092528251810151600090815260ff909152208151805182549394508493839190829060ff19166001838181111561256c5761256c6132ed565b02179055506020918201516001919091015582810151600283015560408084015160038401805460ff1916911515919091179055606090930151600490920191909155828101518383015183519015158152918201869052917f9ef11353afd97d339a77732850b7c22704656558d9ba63cc7e321e0ac4c20aa9910160405180910390a250505050565b80600003612617576040516329c5442960e01b815260040160405180910390fd5b816001600160a01b0316836001600160a01b03167ee763f7778b8ceef7270c89b7d1df1008b0e482da39c43831417733af96fb0d8360405161265b91815260200190565b60405180910390a36109a86001600160a01b0383168483612aff565b80600003612698576040516329c5442960e01b815260040160405180910390fd5b816001600160a01b03167fe0495355c1e04c512584521854d222d239a4b782b39ac8a7e835a34f5ec7c1e1826040516126d391815260200190565b60405180910390a2610ac78282612809565b60408051808201909152600080825260208201526040518060400160405280836001811115612716576127166132ed565b815260200160fb600081548092919061272e906135dc565b90915550905292915050565b6040516001600160a01b03808516602483015283166044820152606481018290526115609085906323b872dd60e01b906084015b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b031990931692909217909152612b2f565b6127af828261105b565b610ac7576127c7816001600160a01b03166014612c01565b6127d2836020612c01565b6040516020016127e39291906137fa565b60408051601f198184030181529082905262461bcd60e51b82526108a39160040161386f565b804710156128595760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a20696e73756666696369656e742062616c616e636500000060448201526064016108a3565b6000826001600160a01b03168260405160006040518083038185875af1925050503d80600081146128a6576040519150601f19603f3d011682016040523d82523d6000602084013e6128ab565b606091505b50509050806109a85760405162461bcd60e51b815260206004820152603a60248201527f416464726573733a20756e61626c6520746f2073656e642076616c75652c207260448201527f6563697069656e74206d6179206861766520726576657274656400000000000060648201526084016108a3565b6040808301356000908152610100602052908120600281015490916001600160a01b03909116906129596080860160608701613456565b6001600160a01b03161461297a576129776080850160608601613456565b90505b60038201546001600160a01b03166001146129b557600382015460048301546129b09183916001600160a01b03909116906125f6565b6129c3565b6129c3818360040154612677565b60408051602086810135825286830135908201529081018490527f13750c731f87c15242874ce74bf4649502cc8e7c829067ce846505acdb96289d90606001611557565b60c95460ff1661128b5760405162461bcd60e51b815260206004820152601460248201527314185d5cd8589b194e881b9bdd081c185d5cd95960621b60448201526064016108a3565b600054610100900460ff16612a775760405162461bcd60e51b81526004016108a3906137af565b60c9805460ff19169055565b600054610100900460ff16612aaa5760405162461bcd60e51b81526004016108a3906137af565b6001609755565b600080825b8015612adb57612ac76002826138b8565b9050612ad460018361372c565b9150612ab6565b612af4828789886000612aef60018b613744565b612da4565b979650505050505050565b6040516001600160a01b0383166024820152604481018290526109a890849063a9059cbb60e01b9060640161276e565b6000612b84826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b0316612eb49092919063ffffffff16565b8051909150156109a85780806020019051810190612ba291906138cc565b6109a85760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b60648201526084016108a3565b60606000612c108360026138ee565b612c1b90600261372c565b67ffffffffffffffff811115612c3357612c336135b0565b6040519080825280601f01601f191660200182016040528015612c5d576020820181803683370190505b509050600360fc1b81600081518110612c7857612c786135f5565b60200101906001600160f81b031916908160001a905350600f60fb1b81600181518110612ca757612ca76135f5565b60200101906001600160f81b031916908160001a9053506000612ccb8460026138ee565b612cd690600161372c565b90505b6001811115612d4e576f181899199a1a9b1b9c1cb0b131b232b360811b85600f1660108110612d0a57612d0a6135f5565b1a60f81b828281518110612d2057612d206135f5565b60200101906001600160f81b031916908160001a90535060049490941c93612d478161390d565b9050612cd9565b508315612d9d5760405162461bcd60e51b815260206004820181905260248201527f537472696e67733a20686578206c656e67746820696e73756666696369656e7460448201526064016108a3565b9392505050565b6000612db1600287613924565b600003612e1c57858214612e7557848484612dcb816135dc565b955081518110612ddd57612ddd6135f5565b6020026020010151604051602001612dff929190918252602082015260400190565b604051602081830303815290604052805190602001209450612e75565b8383612e27816135dc565b945081518110612e3957612e396135f5565b602002602001015185604051602001612e5c929190918252602082015260400190565b6040516020818303038152906040528051906020012094505b86600114612ea957612ea4612e8b600189613744565b612e966002896138b8565b878787612aef6002896138b8565b612af4565b509295945050505050565b6060612ec38484600085612ecb565b949350505050565b606082471015612f2c5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f6044820152651c8818d85b1b60d21b60648201526084016108a3565b6001600160a01b0385163b612f835760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e747261637400000060448201526064016108a3565b600080866001600160a01b03168587604051612f9f9190613938565b60006040518083038185875af1925050503d8060008114612fdc576040519150601f19603f3d011682016040523d82523d6000602084013e612fe1565b606091505b5091509150612af482828660608315612ffb575081612d9d565b82511561300b5782518084602001fd5b8160405162461bcd60e51b81526004016108a3919061386f565b600060a0828403121561303757600080fd5b50919050565b60008083601f84011261304f57600080fd5b50813567ffffffffffffffff81111561306757600080fd5b6020830191508360208260051b850101111561308257600080fd5b9250929050565b60008060008060e0858703121561309f57600080fd5b6130a98686613025565b935060a0850135925060c085013567ffffffffffffffff8111156130cc57600080fd5b6130d88782880161303d565b95989497509550505050565b6000602082840312156130f657600080fd5b81356001600160e01b031981168114612d9d57600080fd5b600060c0828403121561303757600080fd5b600080600080610100858703121561313757600080fd5b613141868661310e565b935060c0850135925060e085013567ffffffffffffffff8111156130cc57600080fd5b80356001600160a01b038116811461317b57600080fd5b919050565b60008060006060848603121561319557600080fd5b61319e84613164565b95602085013595506040909401359392505050565b60008082840360608112156131c757600080fd5b833592506040601f19820112156131dd57600080fd5b506020830190509250929050565b600060c082840312156131fd57600080fd5b612d9d838361310e565b60006020828403121561321957600080fd5b5035919050565b60006080828403121561303757600080fd5b60008060008060c0858703121561324857600080fd5b6132528686613220565b93506080850135925060a085013567ffffffffffffffff8111156130cc57600080fd5b6000806040838503121561328857600080fd5b8235915061329860208401613164565b90509250929050565b600080604083850312156132b457600080fd5b6132bd83613164565b946020939093013593505050565b600080604083850312156132de57600080fd5b50508035926020909101359150565b634e487b7160e01b600052602160045260246000fd5b60028110610ade57610ade6132ed565b805161331e81613303565b8252602090810151910152565b600081518084526020808501945080840160005b83811015613388578151613354888251613313565b8084015160408981019190915281015115156060808a01919091520151608088015260a0909601959082019060010161333f565b509495945050505050565b60006020808352608080840185516133aa81613303565b85840152858301516060604080880182905282519384905260a093928601928489019060005b8181101561342b5785516133e5848251613313565b808a01516001600160a01b03908116858701528582015116868501528581015189850152888101518885015287015160c08401529488019460e0909201916001016133d0565b505089820151898203601f1901848b01529650613448818861332b565b9a9950505050505050505050565b60006020828403121561346857600080fd5b612d9d82613164565b60006080828403121561348357600080fd5b612d9d8383613220565b60e0810161349b8289613313565b6001600160a01b039687166040830152949095166060860152608085019290925260a084015260c090920191909152919050565b602081016134dc83613303565b91905290565b60a081016134f08287613313565b60408201949094529115156060830152608090910152919050565b600060a0828403121561351d57600080fd5b612d9d8383613025565b60028110610ade57600080fd5b60008060006060848603121561354957600080fd5b61355284613164565b9250602084013561356281613527565b915061357060408501613164565b90509250925092565b6020808252601f908201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c00604082015260600190565b634e487b7160e01b600052604160045260246000fd5b634e487b7160e01b600052601160045260246000fd5b6000600182016135ee576135ee6135c6565b5060010190565b634e487b7160e01b600052603260045260246000fd5b60208101600383106134dc576134dc6132ed565b803561362a81613527565b61363381613303565b8252602090810135910152565b60c0810161364e828461361f565b61365a60408401613164565b6001600160a01b0381811660408501528061367760608701613164565b16606085015250506080830135608083015260a083013560a083015292915050565b60005b838110156136b457818101518382015260200161369c565b838111156115605750506000910152565b600083516136d7818460208801613699565b8351908301906136eb818360208801613699565b01949350505050565b60808101613702828461361f565b604083810135908301526001600160a01b0361372060608501613164565b16606083015292915050565b6000821982111561373f5761373f6135c6565b500190565b600082821015613756576137566135c6565b500390565b60a08101613769828461361f565b613783604083016040850180358252602090810135910152565b608092830135919092015290565b82815260608101612d9d602083018480358252602090810135910152565b6020808252602b908201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960408201526a6e697469616c697a696e6760a81b606082015260800190565b7f416363657373436f6e74726f6c3a206163636f756e7420000000000000000000815260008351613832816017850160208801613699565b7001034b99036b4b9b9b4b733903937b6329607d1b6017918401918201528351613863816028840160208801613699565b01602801949350505050565b602081526000825180602084015261388e816040850160208701613699565b601f01601f19169190910160400192915050565b634e487b7160e01b600052601260045260246000fd5b6000826138c7576138c76138a2565b500490565b6000602082840312156138de57600080fd5b81518015158114612d9d57600080fd5b6000816000190483118215151615613908576139086135c6565b500290565b60008161391c5761391c6135c6565b506000190190565b600082613933576139336138a2565b500690565b6000825161394a818460208701613699565b919091019291505056fe73e573f9566d61418a34d5de3ff49360f9c51fec37f7486551670290f6285daba2646970667358221220d0aa7bcf1b59b8eb1ebf01f0f80c4dd46832c645f0af2a6d356c99d2a885a67664736f6c634300080d0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x03\x1AW`\x005`\xE0\x1C\x80c\x89\x0E\x95\xCE\x11a\x01\xABW\x80c\xCA\x9B!\xAE\x11a\0\xF7W\x80c\xDEp\xE0\xB8\x11a\0\x95W\x80c\xF2n\xE9\xD0\x11a\0oW\x80c\xF2n\xE9\xD0\x14a\x08-W\x80c\xF3_\x9EE\x14a\x08CW\x80c\xF9\xEC\xD0\x1E\x14a\x07\xA1W\x80c\xFF+\xAE\x86\x14a\x08cW`\0\x80\xFD[\x80c\xDEp\xE0\xB8\x14a\x07\xE1W\x80c\xDF.\xBD\xBB\x14a\x08\x18W\x80c\xDF\xFB\xDD\x9F\x14a\x05\x80W`\0\x80\xFD[\x80c\xD1eD\xF0\x11a\0\xD1W\x80c\xD1eD\xF0\x14a\x04\xFEW\x80c\xD1\xCB&\xB4\x14a\x03\x1FW\x80c\xD5Gt\x1F\x14a\x07\xC1W\x80c\xDBkRF\x14a\x07IW`\0\x80\xFD[\x80c\xCA\x9B!\xAE\x14a\x07QW\x80c\xCC\x8C\x90\x9F\x14a\x07\x81W\x80c\xCE-\xE1\xBC\x14a\x07\xA1W`\0\x80\xFD[\x80c\xA2\x17\xFD\xDF\x11a\x01dW\x80c\xB1S\x87\x06\x11a\x01>W\x80c\xB1S\x87\x06\x14a\x06\xEDW\x80c\xC2\xB4\n\xE4\x14a\x07\x02W\x80c\xC7c\xE5\xA1\x14a\x07\"W\x80c\xC8|\"$\x14a\x07IW`\0\x80\xFD[\x80c\xA2\x17\xFD\xDF\x14a\x06\x86W\x80c\xAEF\xDB\x11\x14a\x06\x9BW\x80c\xB0,C\xD0\x14a\x06\xBBW`\0\x80\xFD[\x80c\x89\x0E\x95\xCE\x14a\x06&W\x80c\x8E$\xE3\x92\x14a\x03\xB6W\x80c\x91\xD1HT\x14a\x06FW\x80c\x95\n\xC4\x87\x14a\x04gW\x80c\x97\xFE\xB9&\x14a\x04\xFEW\x80c\x9DT\xF4\x19\x14a\x06fW`\0\x80\xFD[\x80c?K\xA8:\x11a\x02jW\x80c`\x8F\xC3z\x11a\x02#W\x80cq\xC5Da\x11a\x01\xFDW\x80cq\xC5Da\x14a\x05\xA9W\x80cy\xE0A\xF2\x14a\x05\xCEW\x80c\x7F\xD4\xF8E\x14a\x05\xFBW\x80c\x84V\xCBY\x14a\x06\x11W`\0\x80\xFD[\x80c`\x8F\xC3z\x14a\x05\x80W\x80ca\xBC\"\x1A\x14a\x05\x93W\x80cgoSk\x14a\x03\xD6W`\0\x80\xFD[\x80c?K\xA8:\x14a\x04\xC7W\x80cG\xE63\x80\x14a\x04\xDCW\x80cG\xE7\xEF$\x14a\x04\xFEW\x80cK\xF5\xFE\xC3\x14a\x03vW\x80cOH\xEE\xDF\x14a\x05\x1EW\x80c\\\x97Z\xBB\x14a\x05hW`\0\x80\xFD[\x80c\x0E&6\xA3\x11a\x02\xD7W\x80c$\x8A\x9C\xA3\x11a\x02\xB1W\x80c$\x8A\x9C\xA3\x14a\x04)W\x80c%\xAF\xC7j\x14a\x04gW\x80c//\xF1]\x14a\x04\x87W\x80c6V\x8A\xBE\x14a\x04\xA7W`\0\x80\xFD[\x80c\x0E&6\xA3\x14a\x03\xE9W\x80c\x0E\xFEj\x8B\x14a\x03\x96W\x80c!B^\xE0\x14a\x03\x96W`\0\x80\xFD[\x80c\x01\xEFif\x14a\x03\x1FW\x80c\x01\xFF\xC9\xA7\x14a\x03AW\x80c\x03\xEDI\xD3\x14a\x03vW\x80c\x08\xAB\xA1\xB2\x14a\x03\x96W\x80c\x08\xF4-@\x14a\x03\xB6W\x80c\x0C\xACW\xAB\x14a\x03\xD6W[`\0\x80\xFD[4\x80\x15a\x03+W`\0\x80\xFD[Pa\x03?a\x03:6`\x04a0\x89V[a\x08yV[\0[4\x80\x15a\x03MW`\0\x80\xFD[Pa\x03aa\x03\\6`\x04a0\xE4V[a\x08\xC8V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x82W`\0\x80\xFD[Pa\x03?a\x03\x916`\x04a1 V[a\x08\xFFV[4\x80\x15a\x03\xA2W`\0\x80\xFD[Pa\x03?a\x03\xB16`\x04a1\x80V[a\t:V[4\x80\x15a\x03\xC2W`\0\x80\xFD[Pa\x03?a\x03\xD16`\x04a1\xB3V[a\t~V[a\x03?a\x03\xE46`\x04a1\xEBV[a\t\xADV[4\x80\x15a\x03\xF5W`\0\x80\xFD[Pa\x04\x11s\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03mV[4\x80\x15a\x045W`\0\x80\xFD[Pa\x04Ya\x04D6`\x04a2\x07V[`\0\x90\x81R`e` R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x03mV[4\x80\x15a\x04sW`\0\x80\xFD[Pa\x03?a\x04\x826`\x04a22V[a\t\xEDV[4\x80\x15a\x04\x93W`\0\x80\xFD[Pa\x03?a\x04\xA26`\x04a2uV[a\n(V[4\x80\x15a\x04\xB3W`\0\x80\xFD[Pa\x03?a\x04\xC26`\x04a2uV[a\nMV[4\x80\x15a\x04\xD3W`\0\x80\xFD[Pa\x03?a\n\xCBV[4\x80\x15a\x04\xE8W`\0\x80\xFD[Pa\x04Y`\0\x80Q` a9U\x839\x81Q\x91R\x81V[4\x80\x15a\x05\nW`\0\x80\xFD[Pa\x03?a\x05\x196`\x04a2\xA1V[a\n\xE1V[4\x80\x15a\x05*W`\0\x80\xFD[Pa\x05Sa\x0596`\x04a2\x07V[a\x01\x01` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x03mV[4\x80\x15a\x05tW`\0\x80\xFD[P`\xC9T`\xFF\x16a\x03aV[a\x03?a\x05\x8E6`\x04a2\x07V[a\x0B%V[4\x80\x15a\x05\x9FW`\0\x80\xFD[Pa\x04Y`\xFBT\x81V[4\x80\x15a\x05\xB5W`\0\x80\xFD[P`\xFETa\x04\x11\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x05\xDAW`\0\x80\xFD[Pa\x05\xEEa\x05\xE96`\x04a2\xCBV[a\x0B6V[`@Qa\x03m\x91\x90a3\x93V[4\x80\x15a\x06\x07W`\0\x80\xFD[Pa\x04Y`\xFCT\x81V[4\x80\x15a\x06\x1DW`\0\x80\xFD[Pa\x03?a\x0F\xDAV[4\x80\x15a\x062W`\0\x80\xFD[Pa\x04Ya\x06A6`\x04a1\xEBV[a\x0F\xEDV[4\x80\x15a\x06RW`\0\x80\xFD[Pa\x03aa\x06a6`\x04a2uV[a\x10[V[4\x80\x15a\x06rW`\0\x80\xFD[Pa\x03?a\x06\x816`\x04a4VV[a\x10\x86V[4\x80\x15a\x06\x92W`\0\x80\xFD[Pa\x04Y`\0\x81V[4\x80\x15a\x06\xA7W`\0\x80\xFD[Pa\x04Ya\x06\xB66`\x04a4qV[a\x11PV[4\x80\x15a\x06\xC7W`\0\x80\xFD[Pa\x06\xDBa\x06\xD66`\x04a2\x07V[a\x11\x84V[`@Qa\x03m\x96\x95\x94\x93\x92\x91\x90a4\x8DV[4\x80\x15a\x06\xF9W`\0\x80\xFD[Pa\x05\xEEa\x12\x0CV[4\x80\x15a\x07\x0EW`\0\x80\xFD[Pa\x04Ya\x07\x1D6`\x04a2\x07V[a\x12WV[4\x80\x15a\x07.W`\0\x80\xFD[P`\xFETa\x07<\x90`\xFF\x16\x81V[`@Qa\x03m\x91\x90a4\xCFV[a\x03?a\x12yV[4\x80\x15a\x07]W`\0\x80\xFD[Pa\x07qa\x07l6`\x04a2\x07V[a\x12\x8DV[`@Qa\x03m\x94\x93\x92\x91\x90a4\xE2V[4\x80\x15a\x07\x8DW`\0\x80\xFD[Pa\x04Ya\x07\x9C6`\x04a5\x0BV[a\x12\xFFV[4\x80\x15a\x07\xADW`\0\x80\xFD[Pa\x04Ya\x07\xBC6`\x04a2\x07V[a\x133V[4\x80\x15a\x07\xCDW`\0\x80\xFD[Pa\x03?a\x07\xDC6`\x04a2uV[a\x13>V[4\x80\x15a\x07\xEDW`\0\x80\xFD[Pa\x04\x11a\x07\xFC6`\x04a2\x07V[a\x01\x02` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x08$W`\0\x80\xFD[Pa\x04\x11`\x01\x81V[4\x80\x15a\x089W`\0\x80\xFD[Pa\x04Y`\xFDT\x81V[4\x80\x15a\x08OW`\0\x80\xFD[Pa\x03?a\x08^6`\x04a54V[a\x13cV[4\x80\x15a\x08oW`\0\x80\xFD[Pa\x01\x03Ta\x04YV[a\x08\x81a\x15fV[`\x02`\x97T\x03a\x08\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a5yV[`@Q\x80\x91\x03\x90\xFD[`\x02`\x97Ua\x08\xBD\x84\x84\x84\x84a\x15\xACV[PP`\x01`\x97UPPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x08\xF9WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[`\x02`\x97T\x03a\t!W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a5yV[`\x02`\x97Ua\t.a\x15fV[a\x08\xBD\x84\x84\x84\x84a\x16\rV[`\x02`\x97T\x03a\t\\W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a5yV[`\x02`\x97Ua\tia\x15fV[a\tt\x83\x83\x83a\x17\xFDV[PP`\x01`\x97UPV[a\t\x86a\x15fV[`\0\x80Q` a9U\x839\x81Q\x91Ra\t\x9E\x81a\x19\xD3V[a\t\xA8\x83\x83a\x19\xDDV[PPPV[`\x02`\x97T\x03a\t\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a5yV[`\x02`\x97Ua\t\xDCa\x15fV[a\t\xE5\x81a\x1BFV[P`\x01`\x97UV[`\x02`\x97T\x03a\n\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a5yV[`\x02`\x97Ua\n\x1Ca\x15fV[a\x08\xBD\x84\x84\x84\x84a\x1D\xBDV[`\0\x82\x81R`e` R`@\x90 `\x01\x01Ta\nC\x81a\x19\xD3V[a\t\xA8\x83\x83a\x1D\xE5V[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\n\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\x08\xA3V[a\n\xC7\x82\x82a\x1EkV[PPV[`\0a\n\xD6\x81a\x19\xD3V[a\n\xDEa\x1E\xD2V[PV[`\x02`\x97T\x03a\x0B\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a5yV[`\x02`\x97Ua\x0B\x10a\x15fV[a\x0B\x1C\x82\x82`\0a\x17\xFDV[PP`\x01`\x97UV[a\x0B-a\x15fV[a\n\xDE\x81a\x1F$V[a\x0B[`@\x80Q``\x81\x01\x90\x91R\x80`\0\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@\x80Q``\x81\x01\x90\x91R`\xFET`\0\x91\x90\x81\x90`\xFF\x16`\x01\x81\x11\x15a\x0B\x83Wa\x0B\x83a2\xEDV[\x81R` \x01`\0`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\xF8W\x81` \x01[`@\x80Qa\x01\0\x81\x01\x82R`\0`\xC0\x82\x01\x81\x81R`\xE0\x83\x01\x82\x90R\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x0B\xA8W\x90P[P\x81R` \x01`\0`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C_W\x81` \x01[`@\x80Q`\xC0\x81\x01\x82R`\0`\x80\x82\x01\x81\x81R`\xA0\x83\x01\x82\x90R\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x0C\x1EW\x90P[P\x90R\x90P\x83\x15\x80\x15a\x0CpWP\x82\x15[\x15a\x0C|W\x90Pa\x08\xF9V[`\0\x80\x85[\x85\x81\x11a\r\x02W`\0\x81\x81Ra\x01\0` R`@\x90 `\x01\x01T\x15a\x0C\xB0Wa\x0C\xA9\x83a5\xDCV[\x92Pa\x0C\xFAV[`\0\x81\x81R`\xFF` R`@\x90 `\x01\x01T\x15a\x0C\xD7Wa\x0C\xD0\x82a5\xDCV[\x91Pa\x0C\xFAV[`@QcT\xB4\x96\x0F`\xE1\x1B\x81R`\x04\x81\x01\x88\x90R`$\x81\x01\x87\x90R`D\x01a\x08\xA3V[`\x01\x01a\x0C\x81V[P\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x1CWa\r\x1Ca5\xB0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\x8AW\x81` \x01[`@\x80Qa\x01\0\x81\x01\x82R`\0`\xC0\x82\x01\x81\x81R`\xE0\x83\x01\x82\x90R\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\r:W\x90P[P` \x84\x01R\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xA9Wa\r\xA9a5\xB0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\x08W\x81` \x01[`@\x80Q`\xC0\x81\x01\x82R`\0`\x80\x82\x01\x81\x81R`\xA0\x83\x01\x82\x90R\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\r\xC7W\x90P[P`@\x84\x01RP`\0\x90P\x80\x85[\x85\x81\x11a\x0F\xCFW`\0\x81\x81Ra\x01\0` R`@\x90 `\x01\x01T\x15a\x0F\x05W`\0\x81\x81Ra\x01\0` \x81\x90R`@\x91\x82\x90 \x82Q\x91\x82\x01\x90\x92R\x81T\x90\x91\x90\x82\x90`\xC0\x82\x01\x90\x83\x90\x82\x90`\xFF\x16`\x01\x81\x11\x15a\x0EtWa\x0Eta2\xEDV[`\x01\x81\x11\x15a\x0E\x85Wa\x0E\x85a2\xEDV[\x81R`\x01\x91\x90\x91\x01T` \x91\x82\x01R\x90\x82R`\x02\x83\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83\x83\x01R`\x03\x84\x01T\x16`@\x83\x01R`\x04\x83\x01T``\x83\x01R`\x05\x83\x01T`\x80\x83\x01R`\x06\x90\x92\x01T`\xA0\x90\x91\x01R\x85\x01Q\x84a\x0E\xE3\x81a5\xDCV[\x95P\x81Q\x81\x10a\x0E\xF5Wa\x0E\xF5a5\xF5V[` \x02` \x01\x01\x81\x90RPa\x0F\xC7V[`\0\x81\x81R`\xFF` R`@\x90 `\x02\x01T\x15a\x0F\xC2W`\0\x81\x81R`\xFF` \x81\x90R`@\x91\x82\x90 \x82Q`\xC0\x81\x01\x90\x93R\x80T\x90\x91\x83\x91`\x80\x83\x01\x91\x84\x91\x83\x91\x16`\x01\x81\x11\x15a\x0FXWa\x0FXa2\xEDV[`\x01\x81\x11\x15a\x0FiWa\x0Fia2\xEDV[\x81R`\x01\x91\x90\x91\x01T` \x91\x82\x01R\x90\x82R`\x02\x83\x01T\x90\x82\x01R`\x03\x82\x01T`\xFF\x16\x15\x15`@\x80\x83\x01\x91\x90\x91R`\x04\x90\x92\x01T``\x90\x91\x01R\x85\x01Q\x83a\x0F\xB0\x81a5\xDCV[\x94P\x81Q\x81\x10a\x0E\xF5Wa\x0E\xF5a5\xF5V[a\x0F\xCFV[`\x01\x01a\x0E\x16V[P\x91\x95\x94PPPPPV[`\0a\x0F\xE5\x81a\x19\xD3V[a\n\xDEa \xBAV[`\0\x80`@Q` \x01a\x10\0\x91\x90a6\x0BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x82`@Q` \x01a\x10 \x91\x90a6@V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x10>\x92\x91` \x01a6\xC5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x91\x82R`e` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0a\x10\x91\x81a\x19\xD3V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x10\xBBW`@Q`\x01by\xC3]`\xE0\x1B\x03\x19\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFETa\x10\xE5\x90`\0\x80Q` a9U\x839\x81Q\x91R\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16a\x1EkV[a\x10\xFD`\0\x80Q` a9U\x839\x81Q\x91R\x83a\x1D\xE5V[`\xFE\x80Ta\x01\0`\x01`\xA8\x1B\x03\x19\x16a\x01\0`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U`@Q\x7F\x1B\x0F/P\r\xF5\x96\xB4+s\xE8\r\xBE\xC6\xA1\xFBW\x0F\x01\x97\x8AXg#\xF9\x88\xA5\xFCT\xD7s\xA1\x90`\0\x90\xA2PPV[`\0`\x02`@Q` \x01a\x11d\x91\x90a6\x0BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x82`@Q` \x01a\x10 \x91\x90a6\xF4V[a\x01\0` R`\0\x90\x81R`@\x90\x81\x90 \x81Q\x80\x83\x01\x90\x92R\x80T\x90\x91\x90\x82\x90\x82\x90`\xFF\x16`\x01\x81\x11\x15a\x11\xBAWa\x11\xBAa2\xEDV[`\x01\x81\x11\x15a\x11\xCBWa\x11\xCBa2\xEDV[\x81R`\x01\x91\x90\x91\x01T` \x90\x91\x01R`\x02\x82\x01T`\x03\x83\x01T`\x04\x84\x01T`\x05\x85\x01T`\x06\x90\x95\x01T\x93\x94`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x94\x92\x90\x93\x16\x92\x90\x91\x86V[a\x121`@\x80Q``\x81\x01\x90\x91R\x80`\0\x81R` \x01``\x81R` \x01``\x81RP\x90V[a\x12R`\xFCT`\x01a\x12C\x91\x90a7,V[`\x01`\xFBTa\x05\xE9\x91\x90a7DV[\x90P\x90V[a\x01\x03\x81\x81T\x81\x10a\x12hW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T\x90P\x81V[a\x12\x81a\x15fV[a\x12\x8B`\0a\x1F$V[V[`\xFF` \x81\x90R`\0\x91\x82R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x90\x92\x91\x83\x91\x83\x91\x16`\x01\x81\x11\x15a\x12\xC2Wa\x12\xC2a2\xEDV[`\x01\x81\x11\x15a\x12\xD3Wa\x12\xD3a2\xEDV[\x81R`\x01\x91\x90\x91\x01T` \x90\x91\x01R`\x02\x82\x01T`\x03\x83\x01T`\x04\x90\x93\x01T\x91\x92\x90\x91`\xFF\x90\x91\x16\x90\x84V[`\0`\x01`@Q` \x01a\x13\x13\x91\x90a6\x0BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x82`@Q` \x01a\x10 \x91\x90a7[V[`\0a\x08\xF9\x82a \xF7V[`\0\x82\x81R`e` R`@\x90 `\x01\x01Ta\x13Y\x81a\x19\xD3V[a\t\xA8\x83\x83a\x1EkV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x13\x83WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x13\x9DWP0;\x15\x80\x15a\x13\x9DWP`\0T`\xFF\x16`\x01\x14[a\x14\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x08\xA3V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x14#W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x14+a\"\x1AV[a\x143a\"\x1AV[a\x14;a\"AV[a\x14Ca\"pV[`\x01`\x01`\xA0\x1B\x03\x84\x16a\x14jW`@Qc9D\xED\x87`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14u`\0\x85a\x1D\xE5V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x14\x9FW`@Q`\x01by\xC3]`\xE0\x1B\x03\x19\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14\xB7`\0\x80Q` a9U\x839\x81Q\x91R\x83a\x1D\xE5V[`\xFE\x80Ta\x01\0`\x01`\xA8\x1B\x03\x19\x81\x16a\x01\0`\x01`\x01`\xA0\x1B\x03\x86\x16\x02\x90\x81\x17\x83U`\x01`\xFB\x81\x90U`\0`\xFC\x81\x90U`\xFDU\x86\x93\x92`\x01`\x01`\xA8\x1B\x03\x19\x16`\xFF\x19\x90\x92\x16\x91\x90\x91\x17\x90\x83\x81\x81\x11\x15a\x15\x14Wa\x15\x14a2\xEDV[\x02\x17\x90UP\x80\x15a\x15`W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01[`@Q\x80\x91\x03\x90\xA1[PPPPV[`\xC9T`\xFF\x16\x15a\x12\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Ro\x14\x18]\\\xD8X\x9B\x19N\x88\x1C\x18]\\\xD9Y`\x82\x1B`D\x82\x01R`d\x01a\x08\xA3V[`\0a\x15\xB7\x85a\x12\xFFV[\x90Pa\x15\xCA` \x86\x015\x82\x86\x86\x86a\"\x9FV[a\x15\xD4\x85\x82a$\x90V[`\0\x90\x81Ra\x01\x02` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16s\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x17\x90UPPPPV[`\0a\x16\x18\x85a\x0F\xEDV[\x90Pa\x16+` \x86\x015\x82\x86\x86\x86a\"\x9FV[`\0\x81\x81Ra\x01\x02` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16s\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x17\x90\x91U`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\x16\xFFW`\x01a\x16\x81`\x80\x88\x01``\x89\x01a4VV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x16\xB2Wa\x16\xAD\x81a\x16\xA3`\x80\x89\x01``\x8A\x01a4VV[\x88`\x80\x015a%\xF6V[a\x16\xC0V[a\x16\xC0\x81\x87`\x80\x015a&wV[`@Q\x82\x81R` \x80\x88\x015\x91\x7F)\x96\xFDTl7\xD7L\x17\x04f\xEAj\xA4\xA3\x08\xE3\xCA-J\xA6\x89\xE6\xE9\xE3)\x94\xDBP9\xCC\x0E\x91\x01`@Q\x80\x91\x03\x90\xA2PPa\x15`V[`\0a\x17\x13`\xA0\x88\x015`\x80\x89\x015a7DV[\x90P`\x01a\x17'`\x80\x89\x01``\x8A\x01a4VV[`\x01`\x01`\xA0\x1B\x03\x16\x03a\x17kWa\x17Na\x17H``\x89\x01`@\x8A\x01a4VV[\x82a&wV[`\xA0\x87\x015\x15a\x17fWa\x17f3\x88`\xA0\x015a&wV[a\x17\xBCV[a\x17\x94a\x17~``\x89\x01`@\x8A\x01a4VV[a\x17\x8E`\x80\x8A\x01``\x8B\x01a4VV[\x83a%\xF6V[`\xA0\x87\x015\x15a\x17\xBCWa\x17\xBC3a\x17\xB2`\x80\x8A\x01``\x8B\x01a4VV[\x89`\xA0\x015a%\xF6V[`@Q\x83\x81R` \x80\x89\x015\x91\x7F\x93_&\xD9K\xE3\x19\x07\x08\n\xA7\x8B>n*\xC6\xD4\x8A\x07*\xF0\x96\xC2\x02h8\x86!\xBB\xC1\x17\x89\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPPPPV[\x81\x81\x81`\0\x03a\x18 W`@Qc\x1F* \x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81\x11\x15a\x18KW`@Qc +1i`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x83\x90R`D\x01a\x08\xA3V[`\x01`\x01`\xA0\x1B\x03\x85\x16a\x18rW`@Qc\xAD\x19\x91\xF5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x80`\xC0\x01`@R\x80a\x18\x89`\0a&\xE5V[\x81R` \x013`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x88\x16` \x80\x83\x01\x91\x90\x91R`@\x80\x83\x01\x89\x90RB``\x84\x01R`\x80\x90\x92\x01\x87\x90R\x82Q\x81\x01Q`\0\x90\x81Ra\x01\0\x90\x91R \x81Q\x80Q\x82T\x93\x94P\x84\x93\x83\x91\x90\x82\x90`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a\x18\xF5Wa\x18\xF5a2\xEDV[\x02\x17\x90UP` \x91\x82\x01Q`\x01\x91\x90\x91\x01U\x82\x01Q`\x02\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`@\x84\x01Q`\x03\x84\x01\x80T\x90\x92\x16\x90\x83\x16\x17\x90U``\x83\x01Q`\x04\x83\x01U`\x80\x83\x01Q`\x05\x83\x01U`\xA0\x90\x92\x01Q`\x06\x90\x91\x01U\x86\x163`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q` \x01Q\x7F\"S\x05\xEC\xB6o\xA9\xB9\xB2\x9F\x8D\xEA\xD9\xBA\xEA6Zl\"]c\x9D\xFD\x86nx,\xCBc\xE2\xF0[\x88\x88`@Qa\x19\xAE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA4a\x19\xCB`\x01`\x01`\xA0\x1B\x03\x87\x1630\x88a':V[PPPPPPV[a\n\xDE\x813a'\xA5V[\x805`\0\x03a\x19\xFFW`@Qci\xF1\xCF\xEF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x015\x815\x11\x15a\x1A2W`@Qcr/\xC3\xF7`\xE1\x1B\x81R\x815`\x04\x82\x01R` \x82\x015`$\x82\x01R`D\x01a\x08\xA3V[`\xFDTa\x1AA`\x01\x835a7DV[\x11\x15a\x1AnW`\xFDT`@Qc\x06P\x04s`\xE5\x1B\x81R\x825`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`D\x01a\x08\xA3V[`\xFDT\x81` \x015\x11a\x1A\xA5W`\xFDT`@QcP\xA7\x92\xB1`\xE0\x1B\x81R` \x83\x015`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`D\x01a\x08\xA3V[a\x01\x03\x80T`\x01\x81\x01\x90\x91U\x7F\x02\xC2\x97\xABt\xAA\xD0\xAE\xDE:\x18\x95\xC8W\xB1\xF2\xC7\x1Ej ?\xEBr{\xEC\x95\xACu)\x98\xCBx\x01\x82\x90U`\0\x82\x81Ra\x01\x01` R`@\x90 \x81\x90a\x1A\xFE\x82\x82\x815\x81U` \x82\x015`\x01\x82\x01UPPV[PP` \x81\x015`\xFDU`@Q\x7FI\xC1X\xD4\x90\xDB\x9E\x06o\x01\xB5\xD4\xF1\xA0\x94HZe\x98\xCBlR\x96\xB4\xC0~F\xC1*\x1D\xC1\x1C\x90a\x1B:\x90\x84\x90\x84\x90a7\x91V[`@Q\x80\x91\x03\x90\xA1PPV[\x80`\x80\x015\x81`\xA0\x015\x81`\0\x03a\x1BqW`@Qc\x1F* \x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81\x11\x15a\x1B\x9CW`@Qc +1i`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x83\x90R`D\x01a\x08\xA3V[`\0a\x1B\xAE``\x85\x01`@\x86\x01a4VV[`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1B\xD5W`@Qc\xD2{DC`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1B\xE0\x84a\x0F\xEDV[`\0\x81\x81Ra\x01\x02` R`@\x90 T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16\x15a\x1C\x1DW`@Qc\xFE\xA5\x94S`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x08\xA3V[`\0\x81\x81Ra\x01\x02` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90Ua\x1CN`\xA0\x86\x015`\x80\x87\x015a7DV[\x90P`\x01a\x1Cb`\x80\x87\x01``\x88\x01a4VV[`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1D%W\x804\x14a\x1C\x99W`@QcL\xEA\xF5\xD3`\xE1\x1B\x81R4`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x01a\x08\xA3V[3a\x1C\xAA``\x87\x01`@\x88\x01a4VV[`@\x80Q` \x80\x8A\x015\x82R\x81\x01\x85\x90R\x90\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90\x7Fz\x9A\xBD\x9E\xB8k\xDB\xCA\x89\xCB\xA4\x06\x9Ac,7\xD9=\xB8.>\x14\xAD\x81\x19\xA3\xA7\x81(\x14\x85>\x90``\x01`@Q\x80\x91\x03\x90\xA3a\x1D\x1E\x81a\x1D\x0F``\x88\x01`@\x89\x01a4VV[`\x01`\x01`\xA0\x1B\x03\x16\x90a(\tV[PPPPPV[3a\x1D6``\x87\x01`@\x88\x01a4VV[`@\x80Q` \x80\x8A\x015\x82R\x81\x01\x85\x90R\x90\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90\x7Fz\x9A\xBD\x9E\xB8k\xDB\xCA\x89\xCB\xA4\x06\x9Ac,7\xD9=\xB8.>\x14\xAD\x81\x19\xA3\xA7\x81(\x14\x85>\x90``\x01`@Q\x80\x91\x03\x90\xA3a\x1D\x1E3a\x1D\x9B``\x88\x01`@\x89\x01a4VV[\x83a\x1D\xAC`\x80\x8A\x01``\x8B\x01a4VV[`\x01`\x01`\xA0\x1B\x03\x16\x92\x91\x90a':V[`\0a\x1D\xC8\x85a\x11PV[\x90Pa\x1D\xDB` \x86\x015\x82\x86\x86\x86a\"\x9FV[a\x15\xD4\x85\x82a)\"V[a\x1D\xEF\x82\x82a\x10[V[a\n\xC7W`\0\x82\x81R`e` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x1E'3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[a\x1Eu\x82\x82a\x10[V[\x15a\n\xC7W`\0\x82\x81R`e` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[a\x1E\xDAa*\x07V[`\xC9\x80T`\xFF\x19\x16\x90U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA3[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA1V[4\x81\x81`\0\x03a\x1FGW`@Qc\x1F* \x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81\x11\x15a\x1FrW`@Qc +1i`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x83\x90R`D\x01a\x08\xA3V[`\0`@Q\x80`\xC0\x01`@R\x80a\x1F\x89`\0a&\xE5V[\x81R3` \x80\x83\x01\x91\x90\x91R`\x01`@\x80\x84\x01\x82\x90R4``\x85\x01RB`\x80\x85\x01R`\xA0\x90\x93\x01\x88\x90R\x83Q\x82\x01Q`\0\x90\x81Ra\x01\0\x90\x92R\x91\x90 \x82Q\x80Q\x82T\x94\x95P\x85\x94\x92\x93\x91\x92\x84\x92\x83\x91`\xFF\x19\x16\x90\x83\x81\x81\x11\x15a\x1F\xEFWa\x1F\xEFa2\xEDV[\x02\x17\x90UP` \x91\x82\x01Q`\x01\x91\x82\x01U\x90\x83\x01Q`\x02\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`@\x85\x01Q`\x03\x85\x01\x80T\x91\x90\x93\x16\x91\x16\x17\x90U``\x83\x01Q`\x04\x83\x01U`\x80\x83\x01Q`\x05\x83\x01U`\xA0\x90\x92\x01Q`\x06\x90\x91\x01Ua `3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q` \x01Q\x7F\"S\x05\xEC\xB6o\xA9\xB9\xB2\x9F\x8D\xEA\xD9\xBA\xEA6Zl\"]c\x9D\xFD\x86nx,\xCBc\xE2\xF0[4\x88`@Qa \xAC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA4PPPPV[a \xC2a\x15fV[`\xC9\x80T`\xFF\x19\x16`\x01\x17\x90U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa\x1F\x073\x90V[`\0`\xFDT\x82\x11\x15a!\x1FW`@Qcd\xB4\xF0y`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x08\xA3V[a\x01\x03T`\0\x81\x90\x03a!EW`@Qc]Cpu`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80[\x80\x15a!\xD1W`\0a\x01\x03a!]`\x01\x84a7DV[\x81T\x81\x10a!mWa!ma5\xF5V[`\0\x91\x82R` \x80\x83 \x90\x91\x01T\x80\x83Ra\x01\x01\x82R`@\x92\x83\x90 \x83Q\x80\x85\x01\x90\x94R\x80T\x80\x85R`\x01\x90\x91\x01T\x92\x84\x01\x92\x90\x92R\x92P\x86\x10\x80\x15\x90a!\xB8WP\x80` \x01Q\x86\x11\x15[\x15a!\xC6WP\x94\x93PPPPV[PP`\0\x19\x01a!GV[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FBatch with request not found\0\0\0\0`D\x82\x01R`d\x01a\x08\xA3V[`\0Ta\x01\0\x90\x04`\xFF\x16a\x12\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a7\xAFV[`\0Ta\x01\0\x90\x04`\xFF\x16a\"hW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a7\xAFV[a\x12\x8Ba*PV[`\0Ta\x01\0\x90\x04`\xFF\x16a\"\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a7\xAFV[a\x12\x8Ba*\x83V[`\0\x84\x81Ra\x01\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16s\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x10\x19\x01a\"\xEFW`@Qc\xE9\x97\x11\xF1`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01a\x08\xA3V[`\0\x83\x81Ra\x01\x01` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x80\x84R`\x01\x90\x91\x01T\x91\x83\x01\x91\x90\x91R\x15\x80a#*WP` \x81\x01Q\x15[\x15a#HW`@Qc9\x07[\xA1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q` \x82\x01Q\x10\x15a#~W\x80Q` \x82\x01Q`@QcT\xB4\x96\x0F`\xE1\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x08\xA3V[\x80Q\x86\x10\x80a#\x90WP\x80` \x01Q\x86\x11[\x15a#\xC5W\x80Q` \x82\x01Q`@QcM4n\x89`\xE0\x1B\x81R`\x04\x81\x01\x89\x90R`$\x81\x01\x92\x90\x92R`D\x82\x01R`d\x01a\x08\xA3V[\x80Q` \x82\x01Q`\0\x91a#\xD8\x91a7DV[a#\xE3\x90`\x01a7,V[\x90Pc\xFF\xFF\xFF\xFF\x81\x11\x15a$\rW`@Qc \x95\xA5=`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x08\xA3V[\x81Q`\0\x90a$\x1C\x90\x89a7DV[\x90P`\0a$`\x88\x83\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92Pa*\xB1\x91PPV[\x90P\x80\x87\x14a$\x85W`@Qc\xF6\xAE\x8DS`\xE0\x1B\x81R`\x04\x81\x01\x88\x90R`$\x01a\x08\xA3V[PPPPPPPPPV[`\0`\x01`\xFBTa$\xA1\x91\x90a7DV[``\x84\x015\x11\x15a$\xB4WP`\x01a$\xFCV[`\0a$\xC8`@\x85\x015``\x86\x015a\x0B6V[\x90P\x80`@Q` \x01a$\xDB\x91\x90a3\x93V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\x80\x015\x14\x15\x91PP[`\0`@Q\x80`\x80\x01`@R\x80a%\x13`\0a&\xE5V[\x81R` \x86\x81\x015\x81\x83\x01R\x84\x15\x15`@\x80\x84\x01\x91\x90\x91RB``\x90\x93\x01\x92\x90\x92R\x82Q\x81\x01Q`\0\x90\x81R`\xFF\x90\x91R \x81Q\x80Q\x82T\x93\x94P\x84\x93\x83\x91\x90\x82\x90`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a%lWa%la2\xEDV[\x02\x17\x90UP` \x91\x82\x01Q`\x01\x91\x90\x91\x01U\x82\x81\x01Q`\x02\x83\x01U`@\x80\x84\x01Q`\x03\x84\x01\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90U``\x90\x93\x01Q`\x04\x90\x92\x01\x91\x90\x91U\x82\x81\x01Q\x83\x83\x01Q\x83Q\x90\x15\x15\x81R\x91\x82\x01\x86\x90R\x91\x7F\x9E\xF1\x13S\xAF\xD9}3\x9Aws(P\xB7\xC2'\x04eeX\xD9\xBAc\xCC~2\x1E\n\xC4\xC2\n\xA9\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPV[\x80`\0\x03a&\x17W`@Qc)\xC5D)`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16~\xE7c\xF7w\x8B\x8C\xEE\xF7'\x0C\x89\xB7\xD1\xDF\x10\x08\xB0\xE4\x82\xDA9\xC481Aw3\xAF\x96\xFB\r\x83`@Qa&[\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3a\t\xA8`\x01`\x01`\xA0\x1B\x03\x83\x16\x84\x83a*\xFFV[\x80`\0\x03a&\x98W`@Qc)\xC5D)`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\xE0ISU\xC1\xE0LQ%\x84R\x18T\xD2\"\xD29\xA4\xB7\x82\xB3\x9A\xC8\xA7\xE85\xA3O^\xC7\xC1\xE1\x82`@Qa&\xD3\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2a\n\xC7\x82\x82a(\tV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`@Q\x80`@\x01`@R\x80\x83`\x01\x81\x11\x15a'\x16Wa'\x16a2\xEDV[\x81R` \x01`\xFB`\0\x81T\x80\x92\x91\x90a'.\x90a5\xDCV[\x90\x91UP\x90R\x92\x91PPV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x15`\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra+/V[a'\xAF\x82\x82a\x10[V[a\n\xC7Wa'\xC7\x81`\x01`\x01`\xA0\x1B\x03\x16`\x14a,\x01V[a'\xD2\x83` a,\x01V[`@Q` \x01a'\xE3\x92\x91\x90a7\xFAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x08\xA3\x91`\x04\x01a8oV[\x80G\x10\x15a(YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: insufficient balance\0\0\0`D\x82\x01R`d\x01a\x08\xA3V[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a(\xA6W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a(\xABV[``\x91P[PP\x90P\x80a\t\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FAddress: unable to send value, r`D\x82\x01R\x7Fecipient may have reverted\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\xA3V[`@\x80\x83\x015`\0\x90\x81Ra\x01\0` R\x90\x81 `\x02\x81\x01T\x90\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a)Y`\x80\x86\x01``\x87\x01a4VV[`\x01`\x01`\xA0\x1B\x03\x16\x14a)zWa)w`\x80\x85\x01``\x86\x01a4VV[\x90P[`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x16`\x01\x14a)\xB5W`\x03\x82\x01T`\x04\x83\x01Ta)\xB0\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a%\xF6V[a)\xC3V[a)\xC3\x81\x83`\x04\x01Ta&wV[`@\x80Q` \x86\x81\x015\x82R\x86\x83\x015\x90\x82\x01R\x90\x81\x01\x84\x90R\x7F\x13u\x0Cs\x1F\x87\xC1RB\x87L\xE7K\xF4d\x95\x02\xCC\x8E|\x82\x90g\xCE\x84e\x05\xAC\xDB\x96(\x9D\x90``\x01a\x15WV[`\xC9T`\xFF\x16a\x12\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x14\x18]\\\xD8X\x9B\x19N\x88\x1B\x9B\xDD\x08\x1C\x18]\\\xD9Y`b\x1B`D\x82\x01R`d\x01a\x08\xA3V[`\0Ta\x01\0\x90\x04`\xFF\x16a*wW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a7\xAFV[`\xC9\x80T`\xFF\x19\x16\x90UV[`\0Ta\x01\0\x90\x04`\xFF\x16a*\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a7\xAFV[`\x01`\x97UV[`\0\x80\x82[\x80\x15a*\xDBWa*\xC7`\x02\x82a8\xB8V[\x90Pa*\xD4`\x01\x83a7,V[\x91Pa*\xB6V[a*\xF4\x82\x87\x89\x88`\0a*\xEF`\x01\x8Ba7DV[a-\xA4V[\x97\x96PPPPPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\t\xA8\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a'nV[`\0a+\x84\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a.\xB4\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\t\xA8W\x80\x80` \x01\x90Q\x81\x01\x90a+\xA2\x91\x90a8\xCCV[a\t\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x08\xA3V[```\0a,\x10\x83`\x02a8\xEEV[a,\x1B\x90`\x02a7,V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,3Wa,3a5\xB0V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a,]W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a,xWa,xa5\xF5V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a,\xA7Wa,\xA7a5\xF5V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a,\xCB\x84`\x02a8\xEEV[a,\xD6\x90`\x01a7,V[\x90P[`\x01\x81\x11\x15a-NWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a-\nWa-\na5\xF5V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a- Wa- a5\xF5V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a-G\x81a9\rV[\x90Pa,\xD9V[P\x83\x15a-\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x08\xA3V[\x93\x92PPPV[`\0a-\xB1`\x02\x87a9$V[`\0\x03a.\x1CW\x85\x82\x14a.uW\x84\x84\x84a-\xCB\x81a5\xDCV[\x95P\x81Q\x81\x10a-\xDDWa-\xDDa5\xF5V[` \x02` \x01\x01Q`@Q` \x01a-\xFF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x94Pa.uV[\x83\x83a.'\x81a5\xDCV[\x94P\x81Q\x81\x10a.9Wa.9a5\xF5V[` \x02` \x01\x01Q\x85`@Q` \x01a.\\\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x94P[\x86`\x01\x14a.\xA9Wa.\xA4a.\x8B`\x01\x89a7DV[a.\x96`\x02\x89a8\xB8V[\x87\x87\x87a*\xEF`\x02\x89a8\xB8V[a*\xF4V[P\x92\x95\x94PPPPPV[``a.\xC3\x84\x84`\0\x85a.\xCBV[\x94\x93PPPPV[``\x82G\x10\x15a/,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x08\xA3V[`\x01`\x01`\xA0\x1B\x03\x85\x16;a/\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x08\xA3V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa/\x9F\x91\x90a98V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a/\xDCW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a/\xE1V[``\x91P[P\x91P\x91Pa*\xF4\x82\x82\x86``\x83\x15a/\xFBWP\x81a-\x9DV[\x82Q\x15a0\x0BW\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x91\x90a8oV[`\0`\xA0\x82\x84\x03\x12\x15a07W`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a0OW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0gW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a0\x82W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\xE0\x85\x87\x03\x12\x15a0\x9FW`\0\x80\xFD[a0\xA9\x86\x86a0%V[\x93P`\xA0\x85\x015\x92P`\xC0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0\xCCW`\0\x80\xFD[a0\xD8\x87\x82\x88\x01a0=V[\x95\x98\x94\x97P\x95PPPPV[`\0` \x82\x84\x03\x12\x15a0\xF6W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a-\x9DW`\0\x80\xFD[`\0`\xC0\x82\x84\x03\x12\x15a07W`\0\x80\xFD[`\0\x80`\0\x80a\x01\0\x85\x87\x03\x12\x15a17W`\0\x80\xFD[a1A\x86\x86a1\x0EV[\x93P`\xC0\x85\x015\x92P`\xE0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0\xCCW`\0\x80\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a1{W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a1\x95W`\0\x80\xFD[a1\x9E\x84a1dV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0\x80\x82\x84\x03``\x81\x12\x15a1\xC7W`\0\x80\xFD[\x835\x92P`@`\x1F\x19\x82\x01\x12\x15a1\xDDW`\0\x80\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[`\0`\xC0\x82\x84\x03\x12\x15a1\xFDW`\0\x80\xFD[a-\x9D\x83\x83a1\x0EV[`\0` \x82\x84\x03\x12\x15a2\x19W`\0\x80\xFD[P5\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a07W`\0\x80\xFD[`\0\x80`\0\x80`\xC0\x85\x87\x03\x12\x15a2HW`\0\x80\xFD[a2R\x86\x86a2 V[\x93P`\x80\x85\x015\x92P`\xA0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0\xCCW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a2\x88W`\0\x80\xFD[\x825\x91Pa2\x98` \x84\x01a1dV[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a2\xB4W`\0\x80\xFD[a2\xBD\x83a1dV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a2\xDEW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x02\x81\x10a\n\xDEWa\n\xDEa2\xEDV[\x80Qa3\x1E\x81a3\x03V[\x82R` \x90\x81\x01Q\x91\x01RV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a3\x88W\x81Qa3T\x88\x82Qa3\x13V[\x80\x84\x01Q`@\x89\x81\x01\x91\x90\x91R\x81\x01Q\x15\x15``\x80\x8A\x01\x91\x90\x91R\x01Q`\x80\x88\x01R`\xA0\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a3?V[P\x94\x95\x94PPPPPV[`\0` \x80\x83R`\x80\x80\x84\x01\x85Qa3\xAA\x81a3\x03V[\x85\x84\x01R\x85\x83\x01Q```@\x80\x88\x01\x82\x90R\x82Q\x93\x84\x90R`\xA0\x93\x92\x86\x01\x92\x84\x89\x01\x90`\0[\x81\x81\x10\x15a4+W\x85Qa3\xE5\x84\x82Qa3\x13V[\x80\x8A\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x85\x87\x01R\x85\x82\x01Q\x16\x86\x85\x01R\x85\x81\x01Q\x89\x85\x01R\x88\x81\x01Q\x88\x85\x01R\x87\x01Q`\xC0\x84\x01R\x94\x88\x01\x94`\xE0\x90\x92\x01\x91`\x01\x01a3\xD0V[PP\x89\x82\x01Q\x89\x82\x03`\x1F\x19\x01\x84\x8B\x01R\x96Pa4H\x81\x88a3+V[\x9A\x99PPPPPPPPPPV[`\0` \x82\x84\x03\x12\x15a4hW`\0\x80\xFD[a-\x9D\x82a1dV[`\0`\x80\x82\x84\x03\x12\x15a4\x83W`\0\x80\xFD[a-\x9D\x83\x83a2 V[`\xE0\x81\x01a4\x9B\x82\x89a3\x13V[`\x01`\x01`\xA0\x1B\x03\x96\x87\x16`@\x83\x01R\x94\x90\x95\x16``\x86\x01R`\x80\x85\x01\x92\x90\x92R`\xA0\x84\x01R`\xC0\x90\x92\x01\x91\x90\x91R\x91\x90PV[` \x81\x01a4\xDC\x83a3\x03V[\x91\x90R\x90V[`\xA0\x81\x01a4\xF0\x82\x87a3\x13V[`@\x82\x01\x94\x90\x94R\x91\x15\x15``\x83\x01R`\x80\x90\x91\x01R\x91\x90PV[`\0`\xA0\x82\x84\x03\x12\x15a5\x1DW`\0\x80\xFD[a-\x9D\x83\x83a0%V[`\x02\x81\x10a\n\xDEW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a5IW`\0\x80\xFD[a5R\x84a1dV[\x92P` \x84\x015a5b\x81a5'V[\x91Pa5p`@\x85\x01a1dV[\x90P\x92P\x92P\x92V[` \x80\x82R`\x1F\x90\x82\x01R\x7FReentrancyGuard: reentrant call\0`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a5\xEEWa5\xEEa5\xC6V[P`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x81\x01`\x03\x83\x10a4\xDCWa4\xDCa2\xEDV[\x805a6*\x81a5'V[a63\x81a3\x03V[\x82R` \x90\x81\x015\x91\x01RV[`\xC0\x81\x01a6N\x82\x84a6\x1FV[a6Z`@\x84\x01a1dV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`@\x85\x01R\x80a6w``\x87\x01a1dV[\x16``\x85\x01RPP`\x80\x83\x015`\x80\x83\x01R`\xA0\x83\x015`\xA0\x83\x01R\x92\x91PPV[`\0[\x83\x81\x10\x15a6\xB4W\x81\x81\x01Q\x83\x82\x01R` \x01a6\x9CV[\x83\x81\x11\x15a\x15`WPP`\0\x91\x01RV[`\0\x83Qa6\xD7\x81\x84` \x88\x01a6\x99V[\x83Q\x90\x83\x01\x90a6\xEB\x81\x83` \x88\x01a6\x99V[\x01\x94\x93PPPPV[`\x80\x81\x01a7\x02\x82\x84a6\x1FV[`@\x83\x81\x015\x90\x83\x01R`\x01`\x01`\xA0\x1B\x03a7 ``\x85\x01a1dV[\x16``\x83\x01R\x92\x91PPV[`\0\x82\x19\x82\x11\x15a7?Wa7?a5\xC6V[P\x01\x90V[`\0\x82\x82\x10\x15a7VWa7Va5\xC6V[P\x03\x90V[`\xA0\x81\x01a7i\x82\x84a6\x1FV[a7\x83`@\x83\x01`@\x85\x01\x805\x82R` \x90\x81\x015\x91\x01RV[`\x80\x92\x83\x015\x91\x90\x92\x01R\x90V[\x82\x81R``\x81\x01a-\x9D` \x83\x01\x84\x805\x82R` \x90\x81\x015\x91\x01RV[` \x80\x82R`+\x90\x82\x01R\x7FInitializable: contract is not i`@\x82\x01Rjnitializing`\xA8\x1B``\x82\x01R`\x80\x01\x90V[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa82\x81`\x17\x85\x01` \x88\x01a6\x99V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa8c\x81`(\x84\x01` \x88\x01a6\x99V[\x01`(\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra8\x8E\x81`@\x85\x01` \x87\x01a6\x99V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a8\xC7Wa8\xC7a8\xA2V[P\x04\x90V[`\0` \x82\x84\x03\x12\x15a8\xDEW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a-\x9DW`\0\x80\xFD[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a9\x08Wa9\x08a5\xC6V[P\x02\x90V[`\0\x81a9\x1CWa9\x1Ca5\xC6V[P`\0\x19\x01\x90V[`\0\x82a93Wa93a8\xA2V[P\x06\x90V[`\0\x82Qa9J\x81\x84` \x87\x01a6\x99V[\x91\x90\x91\x01\x92\x91PPV\xFEs\xE5s\xF9VmaA\x8A4\xD5\xDE?\xF4\x93`\xF9\xC5\x1F\xEC7\xF7HeQg\x02\x90\xF6(]\xAB\xA2dipfsX\"\x12 \xD0\xAA{\xCF\x1BY\xB8\xEB\x1E\xBF\x01\xF0\xF8\x0CM\xD4h2\xC6E\xF0\xAF*m5l\x99\xD2\xA8\x85\xA6vdsolcC\0\x08\r\x003",
    );
    /**Custom error with signature `FerryTipExceedsAmount(uint256,uint256)` and selector `0x80acc5a4`.
    ```solidity
    error FerryTipExceedsAmount(uint256 ferryTip, uint256 amount);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FerryTipExceedsAmount {
        pub ferryTip: alloy::sol_types::private::primitives::aliases::U256,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<FerryTipExceedsAmount> for UnderlyingRustTuple<'_> {
            fn from(value: FerryTipExceedsAmount) -> Self {
                (value.ferryTip, value.amount)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FerryTipExceedsAmount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    ferryTip: tuple.0,
                    amount: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FerryTipExceedsAmount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FerryTipExceedsAmount(uint256,uint256)";
            const SELECTOR: [u8; 4] = [128u8, 172u8, 197u8, 164u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.ferryTip,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.amount,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `InvalidFerriedAmount(uint256,uint256)` and selector `0x99d5eba6`.
    ```solidity
    error InvalidFerriedAmount(uint256 actualAmount, uint256 expectedAmount);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidFerriedAmount {
        pub actualAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub expectedAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidFerriedAmount> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidFerriedAmount) -> Self {
                (value.actualAmount, value.expectedAmount)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidFerriedAmount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    actualAmount: tuple.0,
                    expectedAmount: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidFerriedAmount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidFerriedAmount(uint256,uint256)";
            const SELECTOR: [u8; 4] = [153u8, 213u8, 235u8, 166u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.actualAmount,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.expectedAmount,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `InvalidRequestId(uint256)` and selector `0xc969e0f2`.
    ```solidity
    error InvalidRequestId(uint256 requestId);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidRequestId {
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidRequestId> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidRequestId) -> Self {
                (value.requestId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidRequestId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { requestId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidRequestId {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidRequestId(uint256)";
            const SELECTOR: [u8; 4] = [201u8, 105u8, 224u8, 242u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.requestId,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `InvalidRequestProof(bytes32)` and selector `0xf6ae8d53`.
    ```solidity
    error InvalidRequestProof(bytes32 merkleRoot);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidRequestProof {
        pub merkleRoot: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidRequestProof> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidRequestProof) -> Self {
                (value.merkleRoot,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidRequestProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    merkleRoot: tuple.0,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidRequestProof {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidRequestProof(bytes32)";
            const SELECTOR: [u8; 4] = [246u8, 174u8, 141u8, 83u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.merkleRoot),
                )
            }
        }
    };
    /**Custom error with signature `InvalidRequestRange(uint256,uint256)` and selector `0xa9692c1e`.
    ```solidity
    error InvalidRequestRange(uint256 start, uint256 end);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidRequestRange {
        pub start: alloy::sol_types::private::primitives::aliases::U256,
        pub end: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidRequestRange> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidRequestRange) -> Self {
                (value.start, value.end)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidRequestRange {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    start: tuple.0,
                    end: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidRequestRange {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidRequestRange(uint256,uint256)";
            const SELECTOR: [u8; 4] = [169u8, 105u8, 44u8, 30u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.start,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.end,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `InvalidUpdateRange(uint256,uint256)` and selector `0xe45f87ee`.
    ```solidity
    error InvalidUpdateRange(uint256 start, uint256 end);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidUpdateRange {
        pub start: alloy::sol_types::private::primitives::aliases::U256,
        pub end: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidUpdateRange> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidUpdateRange) -> Self {
                (value.start, value.end)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidUpdateRange {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    start: tuple.0,
                    end: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidUpdateRange {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidUpdateRange(uint256,uint256)";
            const SELECTOR: [u8; 4] = [228u8, 95u8, 135u8, 238u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.start,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.end,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `L2RequestAlreadyProcessed(bytes32)` and selector `0xe99711f1`.
    ```solidity
    error L2RequestAlreadyProcessed(bytes32 requestHash);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct L2RequestAlreadyProcessed {
        pub requestHash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<L2RequestAlreadyProcessed> for UnderlyingRustTuple<'_> {
            fn from(value: L2RequestAlreadyProcessed) -> Self {
                (value.requestHash,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for L2RequestAlreadyProcessed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    requestHash: tuple.0,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for L2RequestAlreadyProcessed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "L2RequestAlreadyProcessed(bytes32)";
            const SELECTOR: [u8; 4] = [233u8, 151u8, 17u8, 241u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.requestHash),
                )
            }
        }
    };
    /**Custom error with signature `PreviousUpdateMissed(uint256,uint256)` and selector `0xca008e60`.
    ```solidity
    error PreviousUpdateMissed(uint256 currentStartRange, uint256 lastProcessedUpdate);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PreviousUpdateMissed {
        pub currentStartRange: alloy::sol_types::private::primitives::aliases::U256,
        pub lastProcessedUpdate: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<PreviousUpdateMissed> for UnderlyingRustTuple<'_> {
            fn from(value: PreviousUpdateMissed) -> Self {
                (value.currentStartRange, value.lastProcessedUpdate)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PreviousUpdateMissed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    currentStartRange: tuple.0,
                    lastProcessedUpdate: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PreviousUpdateMissed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PreviousUpdateMissed(uint256,uint256)";
            const SELECTOR: [u8; 4] = [202u8, 0u8, 142u8, 96u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.currentStartRange,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.lastProcessedUpdate,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `RequestOutOfRange(uint256,uint256,uint256)` and selector `0x4d346e89`.
    ```solidity
    error RequestOutOfRange(uint256 requestId, uint256 start, uint256 end);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RequestOutOfRange {
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
        pub start: alloy::sol_types::private::primitives::aliases::U256,
        pub end: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<RequestOutOfRange> for UnderlyingRustTuple<'_> {
            fn from(value: RequestOutOfRange) -> Self {
                (value.requestId, value.start, value.end)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RequestOutOfRange {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    requestId: tuple.0,
                    start: tuple.1,
                    end: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for RequestOutOfRange {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "RequestOutOfRange(uint256,uint256,uint256)";
            const SELECTOR: [u8; 4] = [77u8, 52u8, 110u8, 137u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.requestId,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.start,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.end,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `RequestRangeTooLarge(uint256)` and selector `0x825694f4`.
    ```solidity
    error RequestRangeTooLarge(uint256 count);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RequestRangeTooLarge {
        pub count: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<RequestRangeTooLarge> for UnderlyingRustTuple<'_> {
            fn from(value: RequestRangeTooLarge) -> Self {
                (value.count,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RequestRangeTooLarge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { count: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for RequestRangeTooLarge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "RequestRangeTooLarge(uint256)";
            const SELECTOR: [u8; 4] = [130u8, 86u8, 148u8, 244u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.count,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `UnexpectedMerkleRoot()` and selector `0xe41d6e84`.
    ```solidity
    error UnexpectedMerkleRoot();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnexpectedMerkleRoot {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnexpectedMerkleRoot> for UnderlyingRustTuple<'_> {
            fn from(value: UnexpectedMerkleRoot) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnexpectedMerkleRoot {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnexpectedMerkleRoot {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnexpectedMerkleRoot()";
            const SELECTOR: [u8; 4] = [228u8, 29u8, 110u8, 132u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `UpdateAlreadyApplied(uint256,uint256)` and selector `0x50a792b1`.
    ```solidity
    error UpdateAlreadyApplied(uint256 currentEndRange, uint256 lastProcessedUpdate);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UpdateAlreadyApplied {
        pub currentEndRange: alloy::sol_types::private::primitives::aliases::U256,
        pub lastProcessedUpdate: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UpdateAlreadyApplied> for UnderlyingRustTuple<'_> {
            fn from(value: UpdateAlreadyApplied) -> Self {
                (value.currentEndRange, value.lastProcessedUpdate)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UpdateAlreadyApplied {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    currentEndRange: tuple.0,
                    lastProcessedUpdate: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UpdateAlreadyApplied {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UpdateAlreadyApplied(uint256,uint256)";
            const SELECTOR: [u8; 4] = [80u8, 167u8, 146u8, 177u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.currentEndRange,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.lastProcessedUpdate,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `WithdrawalAlreadyFerried(bytes32)` and selector `0xfea59453`.
    ```solidity
    error WithdrawalAlreadyFerried(bytes32 withdrawalHash);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WithdrawalAlreadyFerried {
        pub withdrawalHash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<WithdrawalAlreadyFerried> for UnderlyingRustTuple<'_> {
            fn from(value: WithdrawalAlreadyFerried) -> Self {
                (value.withdrawalHash,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WithdrawalAlreadyFerried {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    withdrawalHash: tuple.0,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WithdrawalAlreadyFerried {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WithdrawalAlreadyFerried(bytes32)";
            const SELECTOR: [u8; 4] = [254u8, 165u8, 148u8, 83u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalHash),
                )
            }
        }
    };
    /**Custom error with signature `ZeroAdmin()` and selector `0x7289db0e`.
    ```solidity
    error ZeroAdmin();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroAdmin {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ZeroAdmin> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroAdmin) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroAdmin {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroAdmin {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroAdmin()";
            const SELECTOR: [u8; 4] = [114u8, 137u8, 219u8, 14u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `ZeroAmount()` and selector `0x1f2a2005`.
    ```solidity
    error ZeroAmount();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroAmount {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ZeroAmount> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroAmount) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroAmount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroAmount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroAmount()";
            const SELECTOR: [u8; 4] = [31u8, 42u8, 32u8, 5u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `ZeroRecipient()` and selector `0xd27b4443`.
    ```solidity
    error ZeroRecipient();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroRecipient {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ZeroRecipient> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroRecipient) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroRecipient {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroRecipient {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroRecipient()";
            const SELECTOR: [u8; 4] = [210u8, 123u8, 68u8, 67u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `ZeroRootCount()` and selector `0x5d437075`.
    ```solidity
    error ZeroRootCount();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroRootCount {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ZeroRootCount> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroRootCount) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroRootCount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroRootCount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroRootCount()";
            const SELECTOR: [u8; 4] = [93u8, 67u8, 112u8, 117u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `ZeroToken()` and selector `0xad1991f5`.
    ```solidity
    error ZeroToken();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroToken {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ZeroToken> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroToken) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroToken {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroToken {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroToken()";
            const SELECTOR: [u8; 4] = [173u8, 25u8, 145u8, 245u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `ZeroTransferAmount()` and selector `0x29c54429`.
    ```solidity
    error ZeroTransferAmount();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroTransferAmount {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ZeroTransferAmount> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroTransferAmount) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroTransferAmount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroTransferAmount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroTransferAmount()";
            const SELECTOR: [u8; 4] = [41u8, 197u8, 68u8, 41u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `ZeroUpdateRange()` and selector `0x69f1cfef`.
    ```solidity
    error ZeroUpdateRange();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroUpdateRange {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ZeroUpdateRange> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroUpdateRange) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroUpdateRange {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroUpdateRange {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroUpdateRange()";
            const SELECTOR: [u8; 4] = [105u8, 241u8, 207u8, 239u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `ZeroUpdater()` and selector `0xff863ca3`.
    ```solidity
    error ZeroUpdater();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroUpdater {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ZeroUpdater> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroUpdater) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroUpdater {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroUpdater {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroUpdater()";
            const SELECTOR: [u8; 4] = [255u8, 134u8, 60u8, 163u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Event with signature `DepositAcceptedIntoQueue(uint256,address,address,uint256,uint256)` and selector `0x225305ecb66fa9b9b29f8dead9baea365a6c225d639dfd866e782ccb63e2f05b`.
    ```solidity
    event DepositAcceptedIntoQueue(uint256 indexed requestId, address indexed depositRecipient, address indexed tokenAddress, uint256 amount, uint256 ferryTip);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DepositAcceptedIntoQueue {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub depositRecipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub ferryTip: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for DepositAcceptedIntoQueue {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str =
                "DepositAcceptedIntoQueue(uint256,address,address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    34u8, 83u8, 5u8, 236u8, 182u8, 111u8, 169u8, 185u8, 178u8, 159u8, 141u8, 234u8,
                    217u8, 186u8, 234u8, 54u8, 90u8, 108u8, 34u8, 93u8, 99u8, 157u8, 253u8, 134u8,
                    110u8, 120u8, 44u8, 203u8, 99u8, 226u8, 240u8, 91u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    requestId: topics.1,
                    depositRecipient: topics.2,
                    tokenAddress: topics.3,
                    amount: data.0,
                    ferryTip: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.amount,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.ferryTip,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.requestId.clone(),
                    self.depositRecipient.clone(),
                    self.tokenAddress.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.requestId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.depositRecipient,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.tokenAddress,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DepositAcceptedIntoQueue {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DepositAcceptedIntoQueue> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DepositAcceptedIntoQueue) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `DisputeResolutionAcceptedIntoQueue(uint256,bool,bytes32)` and selector `0x9ef11353afd97d339a77732850b7c22704656558d9ba63cc7e321e0ac4c20aa9`.
    ```solidity
    event DisputeResolutionAcceptedIntoQueue(uint256 indexed requestId, bool cancelJustified, bytes32 cancelResolutionHash);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DisputeResolutionAcceptedIntoQueue {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub cancelJustified: bool,
        #[allow(missing_docs)]
        pub cancelResolutionHash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for DisputeResolutionAcceptedIntoQueue {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str =
                "DisputeResolutionAcceptedIntoQueue(uint256,bool,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    158u8, 241u8, 19u8, 83u8, 175u8, 217u8, 125u8, 51u8, 154u8, 119u8, 115u8, 40u8,
                    80u8, 183u8, 194u8, 39u8, 4u8, 101u8, 101u8, 88u8, 217u8, 186u8, 99u8, 204u8,
                    126u8, 50u8, 30u8, 10u8, 196u8, 194u8, 10u8, 169u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    requestId: topics.1,
                    cancelJustified: data.0,
                    cancelResolutionHash: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.cancelJustified,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.cancelResolutionHash),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.requestId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.requestId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DisputeResolutionAcceptedIntoQueue {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DisputeResolutionAcceptedIntoQueue> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &DisputeResolutionAcceptedIntoQueue,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ERC20TokensWithdrawn(address,address,uint256)` and selector `0x00e763f7778b8ceef7270c89b7d1df1008b0e482da39c43831417733af96fb0d`.
    ```solidity
    event ERC20TokensWithdrawn(address indexed sender, address indexed tokenAddress, uint256 amount);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ERC20TokensWithdrawn {
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ERC20TokensWithdrawn {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ERC20TokensWithdrawn(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    0u8, 231u8, 99u8, 247u8, 119u8, 139u8, 140u8, 238u8, 247u8, 39u8, 12u8, 137u8,
                    183u8, 209u8, 223u8, 16u8, 8u8, 176u8, 228u8, 130u8, 218u8, 57u8, 196u8, 56u8,
                    49u8, 65u8, 119u8, 51u8, 175u8, 150u8, 251u8, 13u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    sender: topics.1,
                    tokenAddress: topics.2,
                    amount: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.amount,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.sender.clone(),
                    self.tokenAddress.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.tokenAddress,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ERC20TokensWithdrawn {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ERC20TokensWithdrawn> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ERC20TokensWithdrawn) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `FailedDepositResolutionClosed(uint256,uint256,bytes32)` and selector `0x13750c731f87c15242874ce74bf4649502cc8e7c829067ce846505acdb96289d`.
    ```solidity
    event FailedDepositResolutionClosed(uint256 indexedrequestId, uint256 originDepositId, bytes32 failedDespotiResolutionHash);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct FailedDepositResolutionClosed {
        #[allow(missing_docs)]
        pub indexedrequestId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub originDepositId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub failedDespotiResolutionHash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for FailedDepositResolutionClosed {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str =
                "FailedDepositResolutionClosed(uint256,uint256,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    19u8, 117u8, 12u8, 115u8, 31u8, 135u8, 193u8, 82u8, 66u8, 135u8, 76u8, 231u8,
                    75u8, 244u8, 100u8, 149u8, 2u8, 204u8, 142u8, 124u8, 130u8, 144u8, 103u8,
                    206u8, 132u8, 101u8, 5u8, 172u8, 219u8, 150u8, 40u8, 157u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    indexedrequestId: data.0,
                    originDepositId: data.1,
                    failedDespotiResolutionHash: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.indexedrequestId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.originDepositId),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.failedDespotiResolutionHash,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for FailedDepositResolutionClosed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&FailedDepositResolutionClosed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &FailedDepositResolutionClosed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `FerriedWithdrawalClosed(uint256,bytes32)` and selector `0x2996fd546c37d74c170466ea6aa4a308e3ca2d4aa689e6e9e32994db5039cc0e`.
    ```solidity
    event FerriedWithdrawalClosed(uint256 indexed requestId, bytes32 withdrawalHash);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct FerriedWithdrawalClosed {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub withdrawalHash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for FerriedWithdrawalClosed {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "FerriedWithdrawalClosed(uint256,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    41u8, 150u8, 253u8, 84u8, 108u8, 55u8, 215u8, 76u8, 23u8, 4u8, 102u8, 234u8,
                    106u8, 164u8, 163u8, 8u8, 227u8, 202u8, 45u8, 74u8, 166u8, 137u8, 230u8, 233u8,
                    227u8, 41u8, 148u8, 219u8, 80u8, 57u8, 204u8, 14u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    requestId: topics.1,
                    withdrawalHash: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalHash),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.requestId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.requestId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for FerriedWithdrawalClosed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&FerriedWithdrawalClosed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &FerriedWithdrawalClosed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Initialized(uint8)` and selector `0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498`.
    ```solidity
    event Initialized(uint8 version);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Initialized {
        #[allow(missing_docs)]
        pub version: u8,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Initialized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8,
                    56u8, 82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8,
                    96u8, 206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { version: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.version,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Initialized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Initialized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Initialized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `L2UpdateAccepted(bytes32,(uint256,uint256))` and selector `0x49c158d490db9e066f01b5d4f1a094485a6598cb6c5296b4c07e46c12a1dc11c`.
    ```solidity
    event L2UpdateAccepted(bytes32 root, IRolldownPrimitives.Range range);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct L2UpdateAccepted {
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub range: <IRolldownPrimitives::Range as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for L2UpdateAccepted {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                IRolldownPrimitives::Range,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "L2UpdateAccepted(bytes32,(uint256,uint256))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    73u8, 193u8, 88u8, 212u8, 144u8, 219u8, 158u8, 6u8, 111u8, 1u8, 181u8, 212u8,
                    241u8, 160u8, 148u8, 72u8, 90u8, 101u8, 152u8, 203u8, 108u8, 82u8, 150u8,
                    180u8, 192u8, 126u8, 70u8, 193u8, 42u8, 29u8, 193u8, 28u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    root: data.0,
                    range: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.root),
                    <IRolldownPrimitives::Range as alloy_sol_types::SolType>::tokenize(
                        &self.range,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for L2UpdateAccepted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&L2UpdateAccepted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &L2UpdateAccepted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `NativeTokensWithdrawn(address,uint256)` and selector `0xe0495355c1e04c512584521854d222d239a4b782b39ac8a7e835a34f5ec7c1e1`.
    ```solidity
    event NativeTokensWithdrawn(address indexed sender, uint256 amount);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NativeTokensWithdrawn {
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for NativeTokensWithdrawn {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "NativeTokensWithdrawn(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    224u8, 73u8, 83u8, 85u8, 193u8, 224u8, 76u8, 81u8, 37u8, 132u8, 82u8, 24u8,
                    84u8, 210u8, 34u8, 210u8, 57u8, 164u8, 183u8, 130u8, 179u8, 154u8, 200u8,
                    167u8, 232u8, 53u8, 163u8, 79u8, 94u8, 199u8, 193u8, 225u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    sender: topics.1,
                    amount: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.amount,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.sender.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NativeTokensWithdrawn {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NativeTokensWithdrawn> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NativeTokensWithdrawn) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `NewUpdaterSet(address)` and selector `0x1b0f2f500df596b42b73e80dbec6a1fb570f01978a586723f988a5fc54d773a1`.
    ```solidity
    event NewUpdaterSet(address indexed updater);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NewUpdaterSet {
        #[allow(missing_docs)]
        pub updater: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for NewUpdaterSet {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "NewUpdaterSet(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    27u8, 15u8, 47u8, 80u8, 13u8, 245u8, 150u8, 180u8, 43u8, 115u8, 232u8, 13u8,
                    190u8, 198u8, 161u8, 251u8, 87u8, 15u8, 1u8, 151u8, 138u8, 88u8, 103u8, 35u8,
                    249u8, 136u8, 165u8, 252u8, 84u8, 215u8, 115u8, 161u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { updater: topics.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.updater.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.updater,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NewUpdaterSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NewUpdaterSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NewUpdaterSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Paused(address)` and selector `0x62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a258`.
    ```solidity
    event Paused(address account);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Paused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Paused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Paused(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    98u8, 231u8, 140u8, 234u8, 1u8, 190u8, 227u8, 32u8, 205u8, 78u8, 66u8, 2u8,
                    112u8, 181u8, 234u8, 116u8, 0u8, 13u8, 17u8, 176u8, 201u8, 247u8, 71u8, 84u8,
                    235u8, 219u8, 252u8, 84u8, 75u8, 5u8, 162u8, 88u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { account: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Paused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Paused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Paused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RoleAdminChanged(bytes32,bytes32,bytes32)` and selector `0xbd79b86ffe0ab8e8776151514217cd7cacd52c909f66475c3af44e129f0b00ff`.
    ```solidity
    event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RoleAdminChanged {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub previousAdminRole: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub newAdminRole: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RoleAdminChanged {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "RoleAdminChanged(bytes32,bytes32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    189u8, 121u8, 184u8, 111u8, 254u8, 10u8, 184u8, 232u8, 119u8, 97u8, 81u8, 81u8,
                    66u8, 23u8, 205u8, 124u8, 172u8, 213u8, 44u8, 144u8, 159u8, 102u8, 71u8, 92u8,
                    58u8, 244u8, 78u8, 18u8, 159u8, 11u8, 0u8, 255u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    role: topics.1,
                    previousAdminRole: topics.2,
                    newAdminRole: topics.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.previousAdminRole.clone(),
                    self.newAdminRole.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.previousAdminRole);
                out[3usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.newAdminRole);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleAdminChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleAdminChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleAdminChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RoleGranted(bytes32,address,address)` and selector `0x2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d`.
    ```solidity
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RoleGranted {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RoleGranted {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RoleGranted(bytes32,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    47u8, 135u8, 136u8, 17u8, 126u8, 126u8, 255u8, 29u8, 130u8, 233u8, 38u8, 236u8,
                    121u8, 73u8, 1u8, 209u8, 124u8, 120u8, 2u8, 74u8, 80u8, 39u8, 9u8, 64u8, 48u8,
                    69u8, 64u8, 167u8, 51u8, 101u8, 111u8, 13u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    role: topics.1,
                    account: topics.2,
                    sender: topics.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.account.clone(),
                    self.sender.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleGranted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleGranted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleGranted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RoleRevoked(bytes32,address,address)` and selector `0xf6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b`.
    ```solidity
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RoleRevoked {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RoleRevoked {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RoleRevoked(bytes32,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    246u8, 57u8, 31u8, 92u8, 50u8, 217u8, 198u8, 157u8, 42u8, 71u8, 234u8, 103u8,
                    11u8, 68u8, 41u8, 116u8, 181u8, 57u8, 53u8, 209u8, 237u8, 199u8, 253u8, 100u8,
                    235u8, 33u8, 224u8, 71u8, 168u8, 57u8, 23u8, 27u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    role: topics.1,
                    account: topics.2,
                    sender: topics.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.account.clone(),
                    self.sender.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleRevoked {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleRevoked> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleRevoked) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Unpaused(address)` and selector `0x5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa`.
    ```solidity
    event Unpaused(address account);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Unpaused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Unpaused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Unpaused(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    93u8, 185u8, 238u8, 10u8, 73u8, 91u8, 242u8, 230u8, 255u8, 156u8, 145u8, 167u8,
                    131u8, 76u8, 27u8, 164u8, 253u8, 210u8, 68u8, 165u8, 232u8, 170u8, 78u8, 83u8,
                    123u8, 211u8, 138u8, 234u8, 228u8, 176u8, 115u8, 170u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { account: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Unpaused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Unpaused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Unpaused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `WithdrawalClosed(uint256,bytes32)` and selector `0x935f26d94be31907080aa78b3e6e2ac6d48a072af096c20268388621bbc11789`.
    ```solidity
    event WithdrawalClosed(uint256 indexed requestId, bytes32 withdrawalHash);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct WithdrawalClosed {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub withdrawalHash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for WithdrawalClosed {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "WithdrawalClosed(uint256,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    147u8, 95u8, 38u8, 217u8, 75u8, 227u8, 25u8, 7u8, 8u8, 10u8, 167u8, 139u8,
                    62u8, 110u8, 42u8, 198u8, 212u8, 138u8, 7u8, 42u8, 240u8, 150u8, 194u8, 2u8,
                    104u8, 56u8, 134u8, 33u8, 187u8, 193u8, 23u8, 137u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    requestId: topics.1,
                    withdrawalHash: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalHash),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.requestId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.requestId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for WithdrawalClosed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&WithdrawalClosed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &WithdrawalClosed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `WithdrawalFerried(uint256,uint256,address,address,bytes32)` and selector `0x7a9abd9eb86bdbca89cba4069a632c37d93db82e3e14ad8119a3a7812814853e`.
    ```solidity
    event WithdrawalFerried(uint256 indexedrequestId, uint256 amount, address indexed recipient, address indexed ferry, bytes32 withdrawalHash);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct WithdrawalFerried {
        #[allow(missing_docs)]
        pub indexedrequestId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub ferry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub withdrawalHash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for WithdrawalFerried {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str =
                "WithdrawalFerried(uint256,uint256,address,address,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    122u8, 154u8, 189u8, 158u8, 184u8, 107u8, 219u8, 202u8, 137u8, 203u8, 164u8,
                    6u8, 154u8, 99u8, 44u8, 55u8, 217u8, 61u8, 184u8, 46u8, 62u8, 20u8, 173u8,
                    129u8, 25u8, 163u8, 167u8, 129u8, 40u8, 20u8, 133u8, 62u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    indexedrequestId: data.0,
                    amount: data.1,
                    recipient: topics.1,
                    ferry: topics.2,
                    withdrawalHash: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.indexedrequestId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalHash),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.recipient.clone(),
                    self.ferry.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.recipient,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.ferry,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for WithdrawalFerried {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&WithdrawalFerried> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &WithdrawalFerried) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Function with signature `CLOSED()` and selector `0x0e2636a3`.
    ```solidity
    function CLOSED() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CLOSEDCall {}
    ///Container type for the return parameters of the [`CLOSED()`](CLOSEDCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CLOSEDReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<CLOSEDCall> for UnderlyingRustTuple<'_> {
                fn from(value: CLOSEDCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CLOSEDCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<CLOSEDReturn> for UnderlyingRustTuple<'_> {
                fn from(value: CLOSEDReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CLOSEDReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for CLOSEDCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = CLOSEDReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CLOSED()";
            const SELECTOR: [u8; 4] = [14u8, 38u8, 54u8, 163u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`.
    ```solidity
    function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_ADMIN_ROLECall {}
    ///Container type for the return parameters of the [`DEFAULT_ADMIN_ROLE()`](DEFAULT_ADMIN_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_ADMIN_ROLEReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<DEFAULT_ADMIN_ROLECall> for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_ADMIN_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for DEFAULT_ADMIN_ROLECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<DEFAULT_ADMIN_ROLEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_ADMIN_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for DEFAULT_ADMIN_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DEFAULT_ADMIN_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = DEFAULT_ADMIN_ROLEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DEFAULT_ADMIN_ROLE()";
            const SELECTOR: [u8; 4] = [162u8, 23u8, 253u8, 223u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `NATIVE_TOKEN_ADDRESS()` and selector `0xdf2ebdbb`.
    ```solidity
    function NATIVE_TOKEN_ADDRESS() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NATIVE_TOKEN_ADDRESSCall {}
    ///Container type for the return parameters of the [`NATIVE_TOKEN_ADDRESS()`](NATIVE_TOKEN_ADDRESSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NATIVE_TOKEN_ADDRESSReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<NATIVE_TOKEN_ADDRESSCall> for UnderlyingRustTuple<'_> {
                fn from(value: NATIVE_TOKEN_ADDRESSCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for NATIVE_TOKEN_ADDRESSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<NATIVE_TOKEN_ADDRESSReturn> for UnderlyingRustTuple<'_> {
                fn from(value: NATIVE_TOKEN_ADDRESSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for NATIVE_TOKEN_ADDRESSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for NATIVE_TOKEN_ADDRESSCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = NATIVE_TOKEN_ADDRESSReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NATIVE_TOKEN_ADDRESS()";
            const SELECTOR: [u8; 4] = [223u8, 46u8, 189u8, 187u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `UPDATER_ROLE()` and selector `0x47e63380`.
    ```solidity
    function UPDATER_ROLE() external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UPDATER_ROLECall {}
    ///Container type for the return parameters of the [`UPDATER_ROLE()`](UPDATER_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UPDATER_ROLEReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UPDATER_ROLECall> for UnderlyingRustTuple<'_> {
                fn from(value: UPDATER_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for UPDATER_ROLECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UPDATER_ROLEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: UPDATER_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for UPDATER_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for UPDATER_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = UPDATER_ROLEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UPDATER_ROLE()";
            const SELECTOR: [u8; 4] = [71u8, 230u8, 51u8, 128u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `cancelResolutions(uint256)` and selector `0xca9b21ae`.
    ```solidity
    function cancelResolutions(uint256) external view returns (IRolldownPrimitives.RequestId memory requestId, uint256 l2RequestId, bool cancelJustified, uint256 timeStamp);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelResolutionsCall {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`cancelResolutions(uint256)`](cancelResolutionsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelResolutionsReturn {
        pub requestId: <IRolldownPrimitives::RequestId as alloy::sol_types::SolType>::RustType,
        pub l2RequestId: alloy::sol_types::private::primitives::aliases::U256,
        pub cancelJustified: bool,
        pub timeStamp: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<cancelResolutionsCall> for UnderlyingRustTuple<'_> {
                fn from(value: cancelResolutionsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for cancelResolutionsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IRolldownPrimitives::RequestId,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRolldownPrimitives::RequestId as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
                bool,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<cancelResolutionsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: cancelResolutionsReturn) -> Self {
                    (
                        value.requestId,
                        value.l2RequestId,
                        value.cancelJustified,
                        value.timeStamp,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for cancelResolutionsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        requestId: tuple.0,
                        l2RequestId: tuple.1,
                        cancelJustified: tuple.2,
                        timeStamp: tuple.3,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cancelResolutionsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = cancelResolutionsReturn;
            type ReturnTuple<'a> = (
                IRolldownPrimitives::RequestId,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cancelResolutions(uint256)";
            const SELECTOR: [u8; 4] = [202u8, 155u8, 33u8, 174u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `chain()` and selector `0xc763e5a1`.
    ```solidity
    function chain() external view returns (IRolldownPrimitives.ChainId);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct chainCall {}
    ///Container type for the return parameters of the [`chain()`](chainCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct chainReturn {
        pub _0: <IRolldownPrimitives::ChainId as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<chainCall> for UnderlyingRustTuple<'_> {
                fn from(value: chainCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for chainCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IRolldownPrimitives::ChainId,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IRolldownPrimitives::ChainId as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<chainReturn> for UnderlyingRustTuple<'_> {
                fn from(value: chainReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for chainReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for chainCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = chainReturn;
            type ReturnTuple<'a> = (IRolldownPrimitives::ChainId,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "chain()";
            const SELECTOR: [u8; 4] = [199u8, 99u8, 229u8, 161u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `closeCancel(((uint8,uint256),(uint256,uint256),bytes32),bytes32,bytes32[])` and selector `0xd1cb26b4`.
    ```solidity
    function closeCancel(IRolldownPrimitives.Cancel memory cancel, bytes32 merkleRoot, bytes32[] memory proof) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct closeCancelCall {
        pub cancel: <IRolldownPrimitives::Cancel as alloy::sol_types::SolType>::RustType,
        pub merkleRoot: alloy::sol_types::private::FixedBytes<32>,
        pub proof: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
    }
    ///Container type for the return parameters of the [`closeCancel(((uint8,uint256),(uint256,uint256),bytes32),bytes32,bytes32[])`](closeCancelCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct closeCancelReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IRolldownPrimitives::Cancel,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRolldownPrimitives::Cancel as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<closeCancelCall> for UnderlyingRustTuple<'_> {
                fn from(value: closeCancelCall) -> Self {
                    (value.cancel, value.merkleRoot, value.proof)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for closeCancelCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        cancel: tuple.0,
                        merkleRoot: tuple.1,
                        proof: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<closeCancelReturn> for UnderlyingRustTuple<'_> {
                fn from(value: closeCancelReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for closeCancelReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for closeCancelCall {
            type Parameters<'a> = (
                IRolldownPrimitives::Cancel,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = closeCancelReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "closeCancel(((uint8,uint256),(uint256,uint256),bytes32),bytes32,bytes32[])";
            const SELECTOR: [u8; 4] = [209u8, 203u8, 38u8, 180u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IRolldownPrimitives::Cancel as alloy_sol_types::SolType>::tokenize(
                        &self.cancel,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.merkleRoot),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.proof),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `closeDepositRefund(((uint8,uint256),uint256,address),bytes32,bytes32[])` and selector `0x25afc76a`.
    ```solidity
    function closeDepositRefund(IRolldownPrimitives.FailedDepositResolution memory failedDeposit, bytes32 merkleRoot, bytes32[] memory proof) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct closeDepositRefundCall {
        pub failedDeposit:
            <IRolldownPrimitives::FailedDepositResolution as alloy::sol_types::SolType>::RustType,
        pub merkleRoot: alloy::sol_types::private::FixedBytes<32>,
        pub proof: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
    }
    ///Container type for the return parameters of the [`closeDepositRefund(((uint8,uint256),uint256,address),bytes32,bytes32[])`](closeDepositRefundCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct closeDepositRefundReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IRolldownPrimitives::FailedDepositResolution,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRolldownPrimitives::FailedDepositResolution as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<closeDepositRefundCall> for UnderlyingRustTuple<'_> {
                fn from(value: closeDepositRefundCall) -> Self {
                    (value.failedDeposit, value.merkleRoot, value.proof)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for closeDepositRefundCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        failedDeposit: tuple.0,
                        merkleRoot: tuple.1,
                        proof: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<closeDepositRefundReturn> for UnderlyingRustTuple<'_> {
                fn from(value: closeDepositRefundReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for closeDepositRefundReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for closeDepositRefundCall {
            type Parameters<'a> = (
                IRolldownPrimitives::FailedDepositResolution,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = closeDepositRefundReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "closeDepositRefund(((uint8,uint256),uint256,address),bytes32,bytes32[])";
            const SELECTOR: [u8; 4] = [37u8, 175u8, 199u8, 106u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IRolldownPrimitives::FailedDepositResolution as alloy_sol_types::SolType>::tokenize(
                        &self.failedDeposit,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.merkleRoot),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.proof),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `closeWithdrawal(((uint8,uint256),address,address,uint256,uint256),bytes32,bytes32[])` and selector `0x03ed49d3`.
    ```solidity
    function closeWithdrawal(IRolldownPrimitives.Withdrawal memory withdrawal, bytes32 merkleRoot, bytes32[] memory proof) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct closeWithdrawalCall {
        pub withdrawal: <IRolldownPrimitives::Withdrawal as alloy::sol_types::SolType>::RustType,
        pub merkleRoot: alloy::sol_types::private::FixedBytes<32>,
        pub proof: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
    }
    ///Container type for the return parameters of the [`closeWithdrawal(((uint8,uint256),address,address,uint256,uint256),bytes32,bytes32[])`](closeWithdrawalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct closeWithdrawalReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IRolldownPrimitives::Withdrawal,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRolldownPrimitives::Withdrawal as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<closeWithdrawalCall> for UnderlyingRustTuple<'_> {
                fn from(value: closeWithdrawalCall) -> Self {
                    (value.withdrawal, value.merkleRoot, value.proof)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for closeWithdrawalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawal: tuple.0,
                        merkleRoot: tuple.1,
                        proof: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<closeWithdrawalReturn> for UnderlyingRustTuple<'_> {
                fn from(value: closeWithdrawalReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for closeWithdrawalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for closeWithdrawalCall {
            type Parameters<'a> = (
                IRolldownPrimitives::Withdrawal,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = closeWithdrawalReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "closeWithdrawal(((uint8,uint256),address,address,uint256,uint256),bytes32,bytes32[])";
            const SELECTOR: [u8; 4] = [3u8, 237u8, 73u8, 211u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IRolldownPrimitives::Withdrawal as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawal,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.merkleRoot),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.proof),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `close_cancel(((uint8,uint256),(uint256,uint256),bytes32),bytes32,bytes32[])` and selector `0x01ef6966`.
    ```solidity
    function close_cancel(IRolldownPrimitives.Cancel memory cancel, bytes32 merkleRoot, bytes32[] memory proof) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct close_cancelCall {
        pub cancel: <IRolldownPrimitives::Cancel as alloy::sol_types::SolType>::RustType,
        pub merkleRoot: alloy::sol_types::private::FixedBytes<32>,
        pub proof: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
    }
    ///Container type for the return parameters of the [`close_cancel(((uint8,uint256),(uint256,uint256),bytes32),bytes32,bytes32[])`](close_cancelCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct close_cancelReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IRolldownPrimitives::Cancel,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRolldownPrimitives::Cancel as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<close_cancelCall> for UnderlyingRustTuple<'_> {
                fn from(value: close_cancelCall) -> Self {
                    (value.cancel, value.merkleRoot, value.proof)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for close_cancelCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        cancel: tuple.0,
                        merkleRoot: tuple.1,
                        proof: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<close_cancelReturn> for UnderlyingRustTuple<'_> {
                fn from(value: close_cancelReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for close_cancelReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for close_cancelCall {
            type Parameters<'a> = (
                IRolldownPrimitives::Cancel,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = close_cancelReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "close_cancel(((uint8,uint256),(uint256,uint256),bytes32),bytes32,bytes32[])";
            const SELECTOR: [u8; 4] = [1u8, 239u8, 105u8, 102u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IRolldownPrimitives::Cancel as alloy_sol_types::SolType>::tokenize(
                        &self.cancel,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.merkleRoot),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.proof),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `close_deposit_refund(((uint8,uint256),uint256,address),bytes32,bytes32[])` and selector `0x950ac487`.
    ```solidity
    function close_deposit_refund(IRolldownPrimitives.FailedDepositResolution memory failedDeposit, bytes32 merkleRoot, bytes32[] memory proof) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct close_deposit_refundCall {
        pub failedDeposit:
            <IRolldownPrimitives::FailedDepositResolution as alloy::sol_types::SolType>::RustType,
        pub merkleRoot: alloy::sol_types::private::FixedBytes<32>,
        pub proof: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
    }
    ///Container type for the return parameters of the [`close_deposit_refund(((uint8,uint256),uint256,address),bytes32,bytes32[])`](close_deposit_refundCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct close_deposit_refundReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IRolldownPrimitives::FailedDepositResolution,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRolldownPrimitives::FailedDepositResolution as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<close_deposit_refundCall> for UnderlyingRustTuple<'_> {
                fn from(value: close_deposit_refundCall) -> Self {
                    (value.failedDeposit, value.merkleRoot, value.proof)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for close_deposit_refundCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        failedDeposit: tuple.0,
                        merkleRoot: tuple.1,
                        proof: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<close_deposit_refundReturn> for UnderlyingRustTuple<'_> {
                fn from(value: close_deposit_refundReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for close_deposit_refundReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for close_deposit_refundCall {
            type Parameters<'a> = (
                IRolldownPrimitives::FailedDepositResolution,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = close_deposit_refundReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "close_deposit_refund(((uint8,uint256),uint256,address),bytes32,bytes32[])";
            const SELECTOR: [u8; 4] = [149u8, 10u8, 196u8, 135u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IRolldownPrimitives::FailedDepositResolution as alloy_sol_types::SolType>::tokenize(
                        &self.failedDeposit,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.merkleRoot),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.proof),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `close_withdrawal(((uint8,uint256),address,address,uint256,uint256),bytes32,bytes32[])` and selector `0x4bf5fec3`.
    ```solidity
    function close_withdrawal(IRolldownPrimitives.Withdrawal memory withdrawal, bytes32 merkleRoot, bytes32[] memory proof) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct close_withdrawalCall {
        pub withdrawal: <IRolldownPrimitives::Withdrawal as alloy::sol_types::SolType>::RustType,
        pub merkleRoot: alloy::sol_types::private::FixedBytes<32>,
        pub proof: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
    }
    ///Container type for the return parameters of the [`close_withdrawal(((uint8,uint256),address,address,uint256,uint256),bytes32,bytes32[])`](close_withdrawalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct close_withdrawalReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IRolldownPrimitives::Withdrawal,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRolldownPrimitives::Withdrawal as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<close_withdrawalCall> for UnderlyingRustTuple<'_> {
                fn from(value: close_withdrawalCall) -> Self {
                    (value.withdrawal, value.merkleRoot, value.proof)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for close_withdrawalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawal: tuple.0,
                        merkleRoot: tuple.1,
                        proof: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<close_withdrawalReturn> for UnderlyingRustTuple<'_> {
                fn from(value: close_withdrawalReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for close_withdrawalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for close_withdrawalCall {
            type Parameters<'a> = (
                IRolldownPrimitives::Withdrawal,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = close_withdrawalReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "close_withdrawal(((uint8,uint256),address,address,uint256,uint256),bytes32,bytes32[])";
            const SELECTOR: [u8; 4] = [75u8, 245u8, 254u8, 195u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IRolldownPrimitives::Withdrawal as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawal,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.merkleRoot),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.proof),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `counter()` and selector `0x61bc221a`.
    ```solidity
    function counter() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct counterCall {}
    ///Container type for the return parameters of the [`counter()`](counterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct counterReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<counterCall> for UnderlyingRustTuple<'_> {
                fn from(value: counterCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for counterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<counterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: counterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for counterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for counterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = counterReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "counter()";
            const SELECTOR: [u8; 4] = [97u8, 188u8, 34u8, 26u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `deposit(address,uint256,uint256)` and selector `0x0efe6a8b`.
    ```solidity
    function deposit(address tokenAddress, uint256 amount, uint256 ferryTip) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_0Call {
        pub tokenAddress: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub ferryTip: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`deposit(address,uint256,uint256)`](deposit_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_0Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deposit_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_0Call) -> Self {
                    (value.tokenAddress, value.amount, value.ferryTip)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenAddress: tuple.0,
                        amount: tuple.1,
                        ferryTip: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deposit_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deposit_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deposit_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deposit(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [14u8, 254u8, 106u8, 139u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.amount,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.ferryTip,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `deposit(address,uint256)` and selector `0x47e7ef24`.
    ```solidity
    function deposit(address tokenAddress, uint256 amount) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_1Call {
        pub tokenAddress: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`deposit(address,uint256)`](deposit_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_1Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deposit_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_1Call) -> Self {
                    (value.tokenAddress, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenAddress: tuple.0,
                        amount: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deposit_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deposit_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deposit_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deposit(address,uint256)";
            const SELECTOR: [u8; 4] = [71u8, 231u8, 239u8, 36u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.amount,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `depositERC20(address,uint256,uint256)` and selector `0x21425ee0`.
    ```solidity
    function depositERC20(address tokenAddress, uint256 amount, uint256 ferryTip) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositERC20_0Call {
        pub tokenAddress: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub ferryTip: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`depositERC20(address,uint256,uint256)`](depositERC20_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositERC20_0Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<depositERC20_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: depositERC20_0Call) -> Self {
                    (value.tokenAddress, value.amount, value.ferryTip)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositERC20_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenAddress: tuple.0,
                        amount: tuple.1,
                        ferryTip: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<depositERC20_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: depositERC20_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositERC20_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for depositERC20_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = depositERC20_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "depositERC20(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [33u8, 66u8, 94u8, 224u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.amount,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.ferryTip,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `depositERC20(address,uint256)` and selector `0x97feb926`.
    ```solidity
    function depositERC20(address tokenAddress, uint256 amount) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositERC20_1Call {
        pub tokenAddress: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`depositERC20(address,uint256)`](depositERC20_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositERC20_1Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<depositERC20_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: depositERC20_1Call) -> Self {
                    (value.tokenAddress, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositERC20_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenAddress: tuple.0,
                        amount: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<depositERC20_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: depositERC20_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositERC20_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for depositERC20_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = depositERC20_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "depositERC20(address,uint256)";
            const SELECTOR: [u8; 4] = [151u8, 254u8, 185u8, 38u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.amount,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `depositNative(uint256)` and selector `0x608fc37a`.
    ```solidity
    function depositNative(uint256 ferryTip) external payable;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositNative_0Call {
        pub ferryTip: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`depositNative(uint256)`](depositNative_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositNative_0Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<depositNative_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: depositNative_0Call) -> Self {
                    (value.ferryTip,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositNative_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { ferryTip: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<depositNative_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: depositNative_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositNative_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for depositNative_0Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = depositNative_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "depositNative(uint256)";
            const SELECTOR: [u8; 4] = [96u8, 143u8, 195u8, 122u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.ferryTip,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `depositNative()` and selector `0xdb6b5246`.
    ```solidity
    function depositNative() external payable;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositNative_1Call {}
    ///Container type for the return parameters of the [`depositNative()`](depositNative_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositNative_1Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<depositNative_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: depositNative_1Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositNative_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<depositNative_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: depositNative_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositNative_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for depositNative_1Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = depositNative_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "depositNative()";
            const SELECTOR: [u8; 4] = [219u8, 107u8, 82u8, 70u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `deposit_erc20(address,uint256,uint256)` and selector `0x08aba1b2`.
    ```solidity
    function deposit_erc20(address tokenAddress, uint256 amount, uint256 ferryTip) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_erc20_0Call {
        pub tokenAddress: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub ferryTip: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`deposit_erc20(address,uint256,uint256)`](deposit_erc20_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_erc20_0Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deposit_erc20_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_erc20_0Call) -> Self {
                    (value.tokenAddress, value.amount, value.ferryTip)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_erc20_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenAddress: tuple.0,
                        amount: tuple.1,
                        ferryTip: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deposit_erc20_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_erc20_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_erc20_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deposit_erc20_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deposit_erc20_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deposit_erc20(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [8u8, 171u8, 161u8, 178u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.amount,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.ferryTip,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `deposit_erc20(address,uint256)` and selector `0xd16544f0`.
    ```solidity
    function deposit_erc20(address tokenAddress, uint256 amount) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_erc20_1Call {
        pub tokenAddress: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`deposit_erc20(address,uint256)`](deposit_erc20_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_erc20_1Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deposit_erc20_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_erc20_1Call) -> Self {
                    (value.tokenAddress, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_erc20_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenAddress: tuple.0,
                        amount: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deposit_erc20_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_erc20_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_erc20_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deposit_erc20_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deposit_erc20_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deposit_erc20(address,uint256)";
            const SELECTOR: [u8; 4] = [209u8, 101u8, 68u8, 240u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.amount,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `deposit_native()` and selector `0xc87c2224`.
    ```solidity
    function deposit_native() external payable;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_native_0Call {}
    ///Container type for the return parameters of the [`deposit_native()`](deposit_native_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_native_0Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deposit_native_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_native_0Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_native_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deposit_native_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_native_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_native_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deposit_native_0Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deposit_native_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deposit_native()";
            const SELECTOR: [u8; 4] = [200u8, 124u8, 34u8, 36u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `deposit_native(uint256)` and selector `0xdffbdd9f`.
    ```solidity
    function deposit_native(uint256 ferryTip) external payable;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_native_1Call {
        pub ferryTip: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`deposit_native(uint256)`](deposit_native_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_native_1Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deposit_native_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_native_1Call) -> Self {
                    (value.ferryTip,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_native_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { ferryTip: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deposit_native_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_native_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_native_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deposit_native_1Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deposit_native_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deposit_native(uint256)";
            const SELECTOR: [u8; 4] = [223u8, 251u8, 221u8, 159u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.ferryTip,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `deposits(uint256)` and selector `0xb02c43d0`.
    ```solidity
    function deposits(uint256) external view returns (IRolldownPrimitives.RequestId memory requestId, address depositRecipient, address tokenAddress, uint256 amount, uint256 timeStamp, uint256 ferryTip);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositsCall {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`deposits(uint256)`](depositsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositsReturn {
        pub requestId: <IRolldownPrimitives::RequestId as alloy::sol_types::SolType>::RustType,
        pub depositRecipient: alloy::sol_types::private::Address,
        pub tokenAddress: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub timeStamp: alloy::sol_types::private::primitives::aliases::U256,
        pub ferryTip: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<depositsCall> for UnderlyingRustTuple<'_> {
                fn from(value: depositsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IRolldownPrimitives::RequestId,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRolldownPrimitives::RequestId as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<depositsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: depositsReturn) -> Self {
                    (
                        value.requestId,
                        value.depositRecipient,
                        value.tokenAddress,
                        value.amount,
                        value.timeStamp,
                        value.ferryTip,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        requestId: tuple.0,
                        depositRecipient: tuple.1,
                        tokenAddress: tuple.2,
                        amount: tuple.3,
                        timeStamp: tuple.4,
                        ferryTip: tuple.5,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for depositsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = depositsReturn;
            type ReturnTuple<'a> = (
                IRolldownPrimitives::RequestId,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deposits(uint256)";
            const SELECTOR: [u8; 4] = [176u8, 44u8, 67u8, 208u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `ferryWithdrawal(((uint8,uint256),address,address,uint256,uint256))` and selector `0x676f536b`.
    ```solidity
    function ferryWithdrawal(IRolldownPrimitives.Withdrawal memory withdrawal) external payable;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ferryWithdrawalCall {
        pub withdrawal: <IRolldownPrimitives::Withdrawal as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`ferryWithdrawal(((uint8,uint256),address,address,uint256,uint256))`](ferryWithdrawalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ferryWithdrawalReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IRolldownPrimitives::Withdrawal,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IRolldownPrimitives::Withdrawal as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ferryWithdrawalCall> for UnderlyingRustTuple<'_> {
                fn from(value: ferryWithdrawalCall) -> Self {
                    (value.withdrawal,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ferryWithdrawalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawal: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ferryWithdrawalReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ferryWithdrawalReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ferryWithdrawalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ferryWithdrawalCall {
            type Parameters<'a> = (IRolldownPrimitives::Withdrawal,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ferryWithdrawalReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "ferryWithdrawal(((uint8,uint256),address,address,uint256,uint256))";
            const SELECTOR: [u8; 4] = [103u8, 111u8, 83u8, 107u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IRolldownPrimitives::Withdrawal as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawal,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `ferry_withdrawal(((uint8,uint256),address,address,uint256,uint256))` and selector `0x0cac57ab`.
    ```solidity
    function ferry_withdrawal(IRolldownPrimitives.Withdrawal memory withdrawal) external payable;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ferry_withdrawalCall {
        pub withdrawal: <IRolldownPrimitives::Withdrawal as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`ferry_withdrawal(((uint8,uint256),address,address,uint256,uint256))`](ferry_withdrawalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ferry_withdrawalReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IRolldownPrimitives::Withdrawal,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IRolldownPrimitives::Withdrawal as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ferry_withdrawalCall> for UnderlyingRustTuple<'_> {
                fn from(value: ferry_withdrawalCall) -> Self {
                    (value.withdrawal,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ferry_withdrawalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawal: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ferry_withdrawalReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ferry_withdrawalReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ferry_withdrawalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ferry_withdrawalCall {
            type Parameters<'a> = (IRolldownPrimitives::Withdrawal,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ferry_withdrawalReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "ferry_withdrawal(((uint8,uint256),address,address,uint256,uint256))";
            const SELECTOR: [u8; 4] = [12u8, 172u8, 87u8, 171u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IRolldownPrimitives::Withdrawal as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawal,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `findL2Batch(uint256)` and selector `0xce2de1bc`.
    ```solidity
    function findL2Batch(uint256 requestId) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct findL2BatchCall {
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`findL2Batch(uint256)`](findL2BatchCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct findL2BatchReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<findL2BatchCall> for UnderlyingRustTuple<'_> {
                fn from(value: findL2BatchCall) -> Self {
                    (value.requestId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for findL2BatchCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { requestId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<findL2BatchReturn> for UnderlyingRustTuple<'_> {
                fn from(value: findL2BatchReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for findL2BatchReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for findL2BatchCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = findL2BatchReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "findL2Batch(uint256)";
            const SELECTOR: [u8; 4] = [206u8, 45u8, 225u8, 188u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.requestId,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `find_l2_batch(uint256)` and selector `0xf9ecd01e`.
    ```solidity
    function find_l2_batch(uint256 requestId) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct find_l2_batchCall {
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`find_l2_batch(uint256)`](find_l2_batchCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct find_l2_batchReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<find_l2_batchCall> for UnderlyingRustTuple<'_> {
                fn from(value: find_l2_batchCall) -> Self {
                    (value.requestId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for find_l2_batchCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { requestId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<find_l2_batchReturn> for UnderlyingRustTuple<'_> {
                fn from(value: find_l2_batchReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for find_l2_batchReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for find_l2_batchCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = find_l2_batchReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "find_l2_batch(uint256)";
            const SELECTOR: [u8; 4] = [249u8, 236u8, 208u8, 30u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.requestId,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getMerkleRootsLength()` and selector `0xff2bae86`.
    ```solidity
    function getMerkleRootsLength() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMerkleRootsLengthCall {}
    ///Container type for the return parameters of the [`getMerkleRootsLength()`](getMerkleRootsLengthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMerkleRootsLengthReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getMerkleRootsLengthCall> for UnderlyingRustTuple<'_> {
                fn from(value: getMerkleRootsLengthCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMerkleRootsLengthCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getMerkleRootsLengthReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getMerkleRootsLengthReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMerkleRootsLengthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMerkleRootsLengthCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMerkleRootsLengthReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getMerkleRootsLength()";
            const SELECTOR: [u8; 4] = [255u8, 43u8, 174u8, 134u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getPendingRequests(uint256,uint256)` and selector `0x79e041f2`.
    ```solidity
    function getPendingRequests(uint256 start, uint256 end) external view returns (IRolldownPrimitives.L1Update memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPendingRequestsCall {
        pub start: alloy::sol_types::private::primitives::aliases::U256,
        pub end: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getPendingRequests(uint256,uint256)`](getPendingRequestsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPendingRequestsReturn {
        pub _0: <IRolldownPrimitives::L1Update as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getPendingRequestsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getPendingRequestsCall) -> Self {
                    (value.start, value.end)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPendingRequestsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        start: tuple.0,
                        end: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IRolldownPrimitives::L1Update,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IRolldownPrimitives::L1Update as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getPendingRequestsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getPendingRequestsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPendingRequestsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPendingRequestsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPendingRequestsReturn;
            type ReturnTuple<'a> = (IRolldownPrimitives::L1Update,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPendingRequests(uint256,uint256)";
            const SELECTOR: [u8; 4] = [121u8, 224u8, 65u8, 242u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.start,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.end,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`.
    ```solidity
    function getRoleAdmin(bytes32 role) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleAdminCall {
        pub role: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getRoleAdmin(bytes32)`](getRoleAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleAdminReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getRoleAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRoleAdminCall) -> Self {
                    (value.role,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoleAdminCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { role: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getRoleAdminReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getRoleAdminReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoleAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRoleAdminCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRoleAdminReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRoleAdmin(bytes32)";
            const SELECTOR: [u8; 4] = [36u8, 138u8, 156u8, 163u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getUpdateForL2()` and selector `0xb1538706`.
    ```solidity
    function getUpdateForL2() external view returns (IRolldownPrimitives.L1Update memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getUpdateForL2Call {}
    ///Container type for the return parameters of the [`getUpdateForL2()`](getUpdateForL2Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getUpdateForL2Return {
        pub _0: <IRolldownPrimitives::L1Update as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getUpdateForL2Call> for UnderlyingRustTuple<'_> {
                fn from(value: getUpdateForL2Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getUpdateForL2Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IRolldownPrimitives::L1Update,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IRolldownPrimitives::L1Update as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getUpdateForL2Return> for UnderlyingRustTuple<'_> {
                fn from(value: getUpdateForL2Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getUpdateForL2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getUpdateForL2Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getUpdateForL2Return;
            type ReturnTuple<'a> = (IRolldownPrimitives::L1Update,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getUpdateForL2()";
            const SELECTOR: [u8; 4] = [177u8, 83u8, 135u8, 6u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`.
    ```solidity
    function grantRole(bytes32 role, address account) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct grantRoleCall {
        pub role: alloy::sol_types::private::FixedBytes<32>,
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`grantRole(bytes32,address)`](grantRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct grantRoleReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<grantRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: grantRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for grantRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<grantRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: grantRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for grantRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for grantRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = grantRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "grantRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [47u8, 47u8, 241u8, 93u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `hasRole(bytes32,address)` and selector `0x91d14854`.
    ```solidity
    function hasRole(bytes32 role, address account) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasRoleCall {
        pub role: alloy::sol_types::private::FixedBytes<32>,
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`hasRole(bytes32,address)`](hasRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasRoleReturn {
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hasRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: hasRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hasRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: hasRoleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hasRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = hasRoleReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hasRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [145u8, 209u8, 72u8, 84u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `hashCancel(((uint8,uint256),(uint256,uint256),bytes32))` and selector `0xcc8c909f`.
    ```solidity
    function hashCancel(IRolldownPrimitives.Cancel memory cancel) external pure returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hashCancelCall {
        pub cancel: <IRolldownPrimitives::Cancel as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`hashCancel(((uint8,uint256),(uint256,uint256),bytes32))`](hashCancelCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hashCancelReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IRolldownPrimitives::Cancel,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IRolldownPrimitives::Cancel as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hashCancelCall> for UnderlyingRustTuple<'_> {
                fn from(value: hashCancelCall) -> Self {
                    (value.cancel,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hashCancelCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { cancel: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hashCancelReturn> for UnderlyingRustTuple<'_> {
                fn from(value: hashCancelReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hashCancelReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hashCancelCall {
            type Parameters<'a> = (IRolldownPrimitives::Cancel,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = hashCancelReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "hashCancel(((uint8,uint256),(uint256,uint256),bytes32))";
            const SELECTOR: [u8; 4] = [204u8, 140u8, 144u8, 159u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IRolldownPrimitives::Cancel as alloy_sol_types::SolType>::tokenize(
                        &self.cancel,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `hashFailedDepositResolution(((uint8,uint256),uint256,address))` and selector `0xae46db11`.
    ```solidity
    function hashFailedDepositResolution(IRolldownPrimitives.FailedDepositResolution memory failedDeposit) external pure returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hashFailedDepositResolutionCall {
        pub failedDeposit:
            <IRolldownPrimitives::FailedDepositResolution as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`hashFailedDepositResolution(((uint8,uint256),uint256,address))`](hashFailedDepositResolutionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hashFailedDepositResolutionReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IRolldownPrimitives::FailedDepositResolution,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRolldownPrimitives::FailedDepositResolution as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hashFailedDepositResolutionCall> for UnderlyingRustTuple<'_> {
                fn from(value: hashFailedDepositResolutionCall) -> Self {
                    (value.failedDeposit,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hashFailedDepositResolutionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        failedDeposit: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hashFailedDepositResolutionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: hashFailedDepositResolutionReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hashFailedDepositResolutionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hashFailedDepositResolutionCall {
            type Parameters<'a> = (IRolldownPrimitives::FailedDepositResolution,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = hashFailedDepositResolutionReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "hashFailedDepositResolution(((uint8,uint256),uint256,address))";
            const SELECTOR: [u8; 4] = [174u8, 70u8, 219u8, 17u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IRolldownPrimitives::FailedDepositResolution as alloy_sol_types::SolType>::tokenize(
                        &self.failedDeposit,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `hashWithdrawal(((uint8,uint256),address,address,uint256,uint256))` and selector `0x890e95ce`.
    ```solidity
    function hashWithdrawal(IRolldownPrimitives.Withdrawal memory withdrawal) external pure returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hashWithdrawalCall {
        pub withdrawal: <IRolldownPrimitives::Withdrawal as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`hashWithdrawal(((uint8,uint256),address,address,uint256,uint256))`](hashWithdrawalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hashWithdrawalReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IRolldownPrimitives::Withdrawal,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IRolldownPrimitives::Withdrawal as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hashWithdrawalCall> for UnderlyingRustTuple<'_> {
                fn from(value: hashWithdrawalCall) -> Self {
                    (value.withdrawal,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hashWithdrawalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawal: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hashWithdrawalReturn> for UnderlyingRustTuple<'_> {
                fn from(value: hashWithdrawalReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hashWithdrawalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hashWithdrawalCall {
            type Parameters<'a> = (IRolldownPrimitives::Withdrawal,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = hashWithdrawalReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "hashWithdrawal(((uint8,uint256),address,address,uint256,uint256))";
            const SELECTOR: [u8; 4] = [137u8, 14u8, 149u8, 206u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IRolldownPrimitives::Withdrawal as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawal,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `initialize(address,uint8,address)` and selector `0xf35f9e45`.
    ```solidity
    function initialize(address admin, IRolldownPrimitives.ChainId chainId, address updater) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub admin: alloy::sol_types::private::Address,
        pub chainId: <IRolldownPrimitives::ChainId as alloy::sol_types::SolType>::RustType,
        pub updater: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address,uint8,address)`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                IRolldownPrimitives::ChainId,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <IRolldownPrimitives::ChainId as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value.admin, value.chainId, value.updater)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        admin: tuple.0,
                        chainId: tuple.1,
                        updater: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                IRolldownPrimitives::ChainId,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,uint8,address)";
            const SELECTOR: [u8; 4] = [243u8, 95u8, 158u8, 69u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.admin,
                    ),
                    <IRolldownPrimitives::ChainId as alloy_sol_types::SolType>::tokenize(
                        &self.chainId,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.updater,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `lastProcessedUpdate_origin_l1()` and selector `0x7fd4f845`.
    ```solidity
    function lastProcessedUpdate_origin_l1() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastProcessedUpdate_origin_l1Call {}
    ///Container type for the return parameters of the [`lastProcessedUpdate_origin_l1()`](lastProcessedUpdate_origin_l1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastProcessedUpdate_origin_l1Return {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<lastProcessedUpdate_origin_l1Call> for UnderlyingRustTuple<'_> {
                fn from(value: lastProcessedUpdate_origin_l1Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lastProcessedUpdate_origin_l1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<lastProcessedUpdate_origin_l1Return> for UnderlyingRustTuple<'_> {
                fn from(value: lastProcessedUpdate_origin_l1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lastProcessedUpdate_origin_l1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastProcessedUpdate_origin_l1Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = lastProcessedUpdate_origin_l1Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lastProcessedUpdate_origin_l1()";
            const SELECTOR: [u8; 4] = [127u8, 212u8, 248u8, 69u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `lastProcessedUpdate_origin_l2()` and selector `0xf26ee9d0`.
    ```solidity
    function lastProcessedUpdate_origin_l2() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastProcessedUpdate_origin_l2Call {}
    ///Container type for the return parameters of the [`lastProcessedUpdate_origin_l2()`](lastProcessedUpdate_origin_l2Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastProcessedUpdate_origin_l2Return {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<lastProcessedUpdate_origin_l2Call> for UnderlyingRustTuple<'_> {
                fn from(value: lastProcessedUpdate_origin_l2Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lastProcessedUpdate_origin_l2Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<lastProcessedUpdate_origin_l2Return> for UnderlyingRustTuple<'_> {
                fn from(value: lastProcessedUpdate_origin_l2Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lastProcessedUpdate_origin_l2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastProcessedUpdate_origin_l2Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = lastProcessedUpdate_origin_l2Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lastProcessedUpdate_origin_l2()";
            const SELECTOR: [u8; 4] = [242u8, 110u8, 233u8, 208u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `merkleRootRange(bytes32)` and selector `0x4f48eedf`.
    ```solidity
    function merkleRootRange(bytes32) external view returns (uint256 start, uint256 end);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct merkleRootRangeCall {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`merkleRootRange(bytes32)`](merkleRootRangeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct merkleRootRangeReturn {
        pub start: alloy::sol_types::private::primitives::aliases::U256,
        pub end: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<merkleRootRangeCall> for UnderlyingRustTuple<'_> {
                fn from(value: merkleRootRangeCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for merkleRootRangeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<merkleRootRangeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: merkleRootRangeReturn) -> Self {
                    (value.start, value.end)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for merkleRootRangeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        start: tuple.0,
                        end: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for merkleRootRangeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = merkleRootRangeReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "merkleRootRange(bytes32)";
            const SELECTOR: [u8; 4] = [79u8, 72u8, 238u8, 223u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `pause()` and selector `0x8456cb59`.
    ```solidity
    function pause() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseCall {}
    ///Container type for the return parameters of the [`pause()`](pauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauseCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pause()";
            const SELECTOR: [u8; 4] = [132u8, 86u8, 203u8, 89u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `paused()` and selector `0x5c975abb`.
    ```solidity
    function paused() external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pausedCall {}
    ///Container type for the return parameters of the [`paused()`](pausedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pausedReturn {
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pausedCall> for UnderlyingRustTuple<'_> {
                fn from(value: pausedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pausedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pausedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pausedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pausedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pausedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pausedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused()";
            const SELECTOR: [u8; 4] = [92u8, 151u8, 90u8, 187u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `processedL2Requests(bytes32)` and selector `0xde70e0b8`.
    ```solidity
    function processedL2Requests(bytes32) external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processedL2RequestsCall {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`processedL2Requests(bytes32)`](processedL2RequestsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processedL2RequestsReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<processedL2RequestsCall> for UnderlyingRustTuple<'_> {
                fn from(value: processedL2RequestsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for processedL2RequestsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<processedL2RequestsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: processedL2RequestsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for processedL2RequestsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for processedL2RequestsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = processedL2RequestsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "processedL2Requests(bytes32)";
            const SELECTOR: [u8; 4] = [222u8, 112u8, 224u8, 184u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`.
    ```solidity
    function renounceRole(bytes32 role, address account) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceRoleCall {
        pub role: alloy::sol_types::private::FixedBytes<32>,
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`renounceRole(bytes32,address)`](renounceRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceRoleReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: renounceRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: renounceRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [54u8, 86u8, 138u8, 190u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`.
    ```solidity
    function revokeRole(bytes32 role, address account) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revokeRoleCall {
        pub role: alloy::sol_types::private::FixedBytes<32>,
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`revokeRole(bytes32,address)`](revokeRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revokeRoleReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<revokeRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: revokeRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<revokeRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: revokeRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for revokeRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = revokeRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "revokeRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [213u8, 71u8, 116u8, 31u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `roots(uint256)` and selector `0xc2b40ae4`.
    ```solidity
    function roots(uint256) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rootsCall {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`roots(uint256)`](rootsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rootsReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<rootsCall> for UnderlyingRustTuple<'_> {
                fn from(value: rootsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rootsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<rootsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: rootsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rootsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rootsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = rootsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "roots(uint256)";
            const SELECTOR: [u8; 4] = [194u8, 180u8, 10u8, 228u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `setUpdater(address)` and selector `0x9d54f419`.
    ```solidity
    function setUpdater(address updater) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setUpdaterCall {
        pub updater: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setUpdater(address)`](setUpdaterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setUpdaterReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setUpdaterCall> for UnderlyingRustTuple<'_> {
                fn from(value: setUpdaterCall) -> Self {
                    (value.updater,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setUpdaterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { updater: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setUpdaterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setUpdaterReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setUpdaterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setUpdaterCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setUpdaterReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setUpdater(address)";
            const SELECTOR: [u8; 4] = [157u8, 84u8, 244u8, 25u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.updater,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`.
    ```solidity
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsInterfaceCall {
        pub interfaceId: alloy::sol_types::private::FixedBytes<4>,
    }
    ///Container type for the return parameters of the [`supportsInterface(bytes4)`](supportsInterfaceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsInterfaceReturn {
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<supportsInterfaceCall> for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceCall) -> Self {
                    (value.interfaceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for supportsInterfaceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        interfaceId: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<supportsInterfaceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for supportsInterfaceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for supportsInterfaceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = supportsInterfaceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "supportsInterface(bytes4)";
            const SELECTOR: [u8; 4] = [1u8, 255u8, 201u8, 167u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.interfaceId),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `unpause()` and selector `0x3f4ba83a`.
    ```solidity
    function unpause() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseCall {}
    ///Container type for the return parameters of the [`unpause()`](unpauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unpauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unpauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unpauseCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = unpauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unpause()";
            const SELECTOR: [u8; 4] = [63u8, 75u8, 168u8, 58u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `updateL1FromL2(bytes32,(uint256,uint256))` and selector `0x8e24e392`.
    ```solidity
    function updateL1FromL2(bytes32 merkleRoot, IRolldownPrimitives.Range memory range) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateL1FromL2Call {
        pub merkleRoot: alloy::sol_types::private::FixedBytes<32>,
        pub range: <IRolldownPrimitives::Range as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`updateL1FromL2(bytes32,(uint256,uint256))`](updateL1FromL2Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateL1FromL2Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                IRolldownPrimitives::Range,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                <IRolldownPrimitives::Range as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateL1FromL2Call> for UnderlyingRustTuple<'_> {
                fn from(value: updateL1FromL2Call) -> Self {
                    (value.merkleRoot, value.range)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateL1FromL2Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        merkleRoot: tuple.0,
                        range: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateL1FromL2Return> for UnderlyingRustTuple<'_> {
                fn from(value: updateL1FromL2Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateL1FromL2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateL1FromL2Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                IRolldownPrimitives::Range,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateL1FromL2Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateL1FromL2(bytes32,(uint256,uint256))";
            const SELECTOR: [u8; 4] = [142u8, 36u8, 227u8, 146u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.merkleRoot),
                    <IRolldownPrimitives::Range as alloy_sol_types::SolType>::tokenize(
                        &self.range,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `update_l1_from_l2(bytes32,(uint256,uint256))` and selector `0x08f42d40`.
    ```solidity
    function update_l1_from_l2(bytes32 merkleRoot, IRolldownPrimitives.Range memory range) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct update_l1_from_l2Call {
        pub merkleRoot: alloy::sol_types::private::FixedBytes<32>,
        pub range: <IRolldownPrimitives::Range as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`update_l1_from_l2(bytes32,(uint256,uint256))`](update_l1_from_l2Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct update_l1_from_l2Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                IRolldownPrimitives::Range,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                <IRolldownPrimitives::Range as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<update_l1_from_l2Call> for UnderlyingRustTuple<'_> {
                fn from(value: update_l1_from_l2Call) -> Self {
                    (value.merkleRoot, value.range)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for update_l1_from_l2Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        merkleRoot: tuple.0,
                        range: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<update_l1_from_l2Return> for UnderlyingRustTuple<'_> {
                fn from(value: update_l1_from_l2Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for update_l1_from_l2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for update_l1_from_l2Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                IRolldownPrimitives::Range,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = update_l1_from_l2Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "update_l1_from_l2(bytes32,(uint256,uint256))";
            const SELECTOR: [u8; 4] = [8u8, 244u8, 45u8, 64u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.merkleRoot),
                    <IRolldownPrimitives::Range as alloy_sol_types::SolType>::tokenize(
                        &self.range,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `updaterAccount()` and selector `0x71c54461`.
    ```solidity
    function updaterAccount() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updaterAccountCall {}
    ///Container type for the return parameters of the [`updaterAccount()`](updaterAccountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updaterAccountReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updaterAccountCall> for UnderlyingRustTuple<'_> {
                fn from(value: updaterAccountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updaterAccountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updaterAccountReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updaterAccountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updaterAccountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updaterAccountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updaterAccountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updaterAccount()";
            const SELECTOR: [u8; 4] = [113u8, 197u8, 68u8, 97u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    ///Container for all the [`Rolldown`](self) function calls.
    pub enum RolldownCalls {
        CLOSED(CLOSEDCall),
        DEFAULT_ADMIN_ROLE(DEFAULT_ADMIN_ROLECall),
        NATIVE_TOKEN_ADDRESS(NATIVE_TOKEN_ADDRESSCall),
        UPDATER_ROLE(UPDATER_ROLECall),
        cancelResolutions(cancelResolutionsCall),
        chain(chainCall),
        closeCancel(closeCancelCall),
        closeDepositRefund(closeDepositRefundCall),
        closeWithdrawal(closeWithdrawalCall),
        close_cancel(close_cancelCall),
        close_deposit_refund(close_deposit_refundCall),
        close_withdrawal(close_withdrawalCall),
        counter(counterCall),
        deposit_0(deposit_0Call),
        deposit_1(deposit_1Call),
        depositERC20_0(depositERC20_0Call),
        depositERC20_1(depositERC20_1Call),
        depositNative_0(depositNative_0Call),
        depositNative_1(depositNative_1Call),
        deposit_erc20_0(deposit_erc20_0Call),
        deposit_erc20_1(deposit_erc20_1Call),
        deposit_native_0(deposit_native_0Call),
        deposit_native_1(deposit_native_1Call),
        deposits(depositsCall),
        ferryWithdrawal(ferryWithdrawalCall),
        ferry_withdrawal(ferry_withdrawalCall),
        findL2Batch(findL2BatchCall),
        find_l2_batch(find_l2_batchCall),
        getMerkleRootsLength(getMerkleRootsLengthCall),
        getPendingRequests(getPendingRequestsCall),
        getRoleAdmin(getRoleAdminCall),
        getUpdateForL2(getUpdateForL2Call),
        grantRole(grantRoleCall),
        hasRole(hasRoleCall),
        hashCancel(hashCancelCall),
        hashFailedDepositResolution(hashFailedDepositResolutionCall),
        hashWithdrawal(hashWithdrawalCall),
        initialize(initializeCall),
        lastProcessedUpdate_origin_l1(lastProcessedUpdate_origin_l1Call),
        lastProcessedUpdate_origin_l2(lastProcessedUpdate_origin_l2Call),
        merkleRootRange(merkleRootRangeCall),
        pause(pauseCall),
        paused(pausedCall),
        processedL2Requests(processedL2RequestsCall),
        renounceRole(renounceRoleCall),
        revokeRole(revokeRoleCall),
        roots(rootsCall),
        setUpdater(setUpdaterCall),
        supportsInterface(supportsInterfaceCall),
        unpause(unpauseCall),
        updateL1FromL2(updateL1FromL2Call),
        update_l1_from_l2(update_l1_from_l2Call),
        updaterAccount(updaterAccountCall),
    }
    #[automatically_derived]
    impl RolldownCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 239u8, 105u8, 102u8],
            [1u8, 255u8, 201u8, 167u8],
            [3u8, 237u8, 73u8, 211u8],
            [8u8, 171u8, 161u8, 178u8],
            [8u8, 244u8, 45u8, 64u8],
            [12u8, 172u8, 87u8, 171u8],
            [14u8, 38u8, 54u8, 163u8],
            [14u8, 254u8, 106u8, 139u8],
            [33u8, 66u8, 94u8, 224u8],
            [36u8, 138u8, 156u8, 163u8],
            [37u8, 175u8, 199u8, 106u8],
            [47u8, 47u8, 241u8, 93u8],
            [54u8, 86u8, 138u8, 190u8],
            [63u8, 75u8, 168u8, 58u8],
            [71u8, 230u8, 51u8, 128u8],
            [71u8, 231u8, 239u8, 36u8],
            [75u8, 245u8, 254u8, 195u8],
            [79u8, 72u8, 238u8, 223u8],
            [92u8, 151u8, 90u8, 187u8],
            [96u8, 143u8, 195u8, 122u8],
            [97u8, 188u8, 34u8, 26u8],
            [103u8, 111u8, 83u8, 107u8],
            [113u8, 197u8, 68u8, 97u8],
            [121u8, 224u8, 65u8, 242u8],
            [127u8, 212u8, 248u8, 69u8],
            [132u8, 86u8, 203u8, 89u8],
            [137u8, 14u8, 149u8, 206u8],
            [142u8, 36u8, 227u8, 146u8],
            [145u8, 209u8, 72u8, 84u8],
            [149u8, 10u8, 196u8, 135u8],
            [151u8, 254u8, 185u8, 38u8],
            [157u8, 84u8, 244u8, 25u8],
            [162u8, 23u8, 253u8, 223u8],
            [174u8, 70u8, 219u8, 17u8],
            [176u8, 44u8, 67u8, 208u8],
            [177u8, 83u8, 135u8, 6u8],
            [194u8, 180u8, 10u8, 228u8],
            [199u8, 99u8, 229u8, 161u8],
            [200u8, 124u8, 34u8, 36u8],
            [202u8, 155u8, 33u8, 174u8],
            [204u8, 140u8, 144u8, 159u8],
            [206u8, 45u8, 225u8, 188u8],
            [209u8, 101u8, 68u8, 240u8],
            [209u8, 203u8, 38u8, 180u8],
            [213u8, 71u8, 116u8, 31u8],
            [219u8, 107u8, 82u8, 70u8],
            [222u8, 112u8, 224u8, 184u8],
            [223u8, 46u8, 189u8, 187u8],
            [223u8, 251u8, 221u8, 159u8],
            [242u8, 110u8, 233u8, 208u8],
            [243u8, 95u8, 158u8, 69u8],
            [249u8, 236u8, 208u8, 30u8],
            [255u8, 43u8, 174u8, 134u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for RolldownCalls {
        const NAME: &'static str = "RolldownCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 53usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::CLOSED(_) => <CLOSEDCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::DEFAULT_ADMIN_ROLE(_) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::NATIVE_TOKEN_ADDRESS(_) => {
                    <NATIVE_TOKEN_ADDRESSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::UPDATER_ROLE(_) => <UPDATER_ROLECall as alloy_sol_types::SolCall>::SELECTOR,
                Self::cancelResolutions(_) => {
                    <cancelResolutionsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::chain(_) => <chainCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::closeCancel(_) => <closeCancelCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::closeDepositRefund(_) => {
                    <closeDepositRefundCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::closeWithdrawal(_) => {
                    <closeWithdrawalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::close_cancel(_) => <close_cancelCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::close_deposit_refund(_) => {
                    <close_deposit_refundCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::close_withdrawal(_) => {
                    <close_withdrawalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::counter(_) => <counterCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::deposit_0(_) => <deposit_0Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::deposit_1(_) => <deposit_1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::depositERC20_0(_) => {
                    <depositERC20_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::depositERC20_1(_) => {
                    <depositERC20_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::depositNative_0(_) => {
                    <depositNative_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::depositNative_1(_) => {
                    <depositNative_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deposit_erc20_0(_) => {
                    <deposit_erc20_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deposit_erc20_1(_) => {
                    <deposit_erc20_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deposit_native_0(_) => {
                    <deposit_native_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deposit_native_1(_) => {
                    <deposit_native_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deposits(_) => <depositsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::ferryWithdrawal(_) => {
                    <ferryWithdrawalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ferry_withdrawal(_) => {
                    <ferry_withdrawalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::findL2Batch(_) => <findL2BatchCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::find_l2_batch(_) => <find_l2_batchCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getMerkleRootsLength(_) => {
                    <getMerkleRootsLengthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPendingRequests(_) => {
                    <getPendingRequestsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRoleAdmin(_) => <getRoleAdminCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getUpdateForL2(_) => {
                    <getUpdateForL2Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::grantRole(_) => <grantRoleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::hasRole(_) => <hasRoleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::hashCancel(_) => <hashCancelCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::hashFailedDepositResolution(_) => {
                    <hashFailedDepositResolutionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::hashWithdrawal(_) => {
                    <hashWithdrawalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => <initializeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::lastProcessedUpdate_origin_l1(_) => {
                    <lastProcessedUpdate_origin_l1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lastProcessedUpdate_origin_l2(_) => {
                    <lastProcessedUpdate_origin_l2Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::merkleRootRange(_) => {
                    <merkleRootRangeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused(_) => <pausedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::processedL2Requests(_) => {
                    <processedL2RequestsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceRole(_) => <renounceRoleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::revokeRole(_) => <revokeRoleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::roots(_) => <rootsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::setUpdater(_) => <setUpdaterCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::supportsInterface(_) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::updateL1FromL2(_) => {
                    <updateL1FromL2Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::update_l1_from_l2(_) => {
                    <update_l1_from_l2Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updaterAccount(_) => {
                    <updaterAccountCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(&[u8], bool) -> alloy_sol_types::Result<RolldownCalls>] = &[
                {
                    fn close_cancel(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <close_cancelCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::close_cancel)
                    }
                    close_cancel
                },
                {
                    fn supportsInterface(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn closeWithdrawal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <closeWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::closeWithdrawal)
                    }
                    closeWithdrawal
                },
                {
                    fn deposit_erc20_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <deposit_erc20_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::deposit_erc20_0)
                    }
                    deposit_erc20_0
                },
                {
                    fn update_l1_from_l2(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <update_l1_from_l2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::update_l1_from_l2)
                    }
                    update_l1_from_l2
                },
                {
                    fn ferry_withdrawal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <ferry_withdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::ferry_withdrawal)
                    }
                    ferry_withdrawal
                },
                {
                    fn CLOSED(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <CLOSEDCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RolldownCalls::CLOSED)
                    }
                    CLOSED
                },
                {
                    fn deposit_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <deposit_0Call as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RolldownCalls::deposit_0)
                    }
                    deposit_0
                },
                {
                    fn depositERC20_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <depositERC20_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::depositERC20_0)
                    }
                    depositERC20_0
                },
                {
                    fn getRoleAdmin(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <getRoleAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::getRoleAdmin)
                    }
                    getRoleAdmin
                },
                {
                    fn closeDepositRefund(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <closeDepositRefundCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::closeDepositRefund)
                    }
                    closeDepositRefund
                },
                {
                    fn grantRole(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <grantRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RolldownCalls::grantRole)
                    }
                    grantRole
                },
                {
                    fn renounceRole(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <renounceRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::renounceRole)
                    }
                    renounceRole
                },
                {
                    fn unpause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RolldownCalls::unpause)
                    }
                    unpause
                },
                {
                    fn UPDATER_ROLE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <UPDATER_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::UPDATER_ROLE)
                    }
                    UPDATER_ROLE
                },
                {
                    fn deposit_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <deposit_1Call as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RolldownCalls::deposit_1)
                    }
                    deposit_1
                },
                {
                    fn close_withdrawal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <close_withdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::close_withdrawal)
                    }
                    close_withdrawal
                },
                {
                    fn merkleRootRange(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <merkleRootRangeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::merkleRootRange)
                    }
                    merkleRootRange
                },
                {
                    fn paused(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RolldownCalls::paused)
                    }
                    paused
                },
                {
                    fn depositNative_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <depositNative_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::depositNative_0)
                    }
                    depositNative_0
                },
                {
                    fn counter(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <counterCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RolldownCalls::counter)
                    }
                    counter
                },
                {
                    fn ferryWithdrawal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <ferryWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::ferryWithdrawal)
                    }
                    ferryWithdrawal
                },
                {
                    fn updaterAccount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <updaterAccountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::updaterAccount)
                    }
                    updaterAccount
                },
                {
                    fn getPendingRequests(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <getPendingRequestsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::getPendingRequests)
                    }
                    getPendingRequests
                },
                {
                    fn lastProcessedUpdate_origin_l1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <lastProcessedUpdate_origin_l1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::lastProcessedUpdate_origin_l1)
                    }
                    lastProcessedUpdate_origin_l1
                },
                {
                    fn pause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RolldownCalls::pause)
                    }
                    pause
                },
                {
                    fn hashWithdrawal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <hashWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::hashWithdrawal)
                    }
                    hashWithdrawal
                },
                {
                    fn updateL1FromL2(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <updateL1FromL2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::updateL1FromL2)
                    }
                    updateL1FromL2
                },
                {
                    fn hasRole(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <hasRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RolldownCalls::hasRole)
                    }
                    hasRole
                },
                {
                    fn close_deposit_refund(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <close_deposit_refundCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::close_deposit_refund)
                    }
                    close_deposit_refund
                },
                {
                    fn depositERC20_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <depositERC20_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::depositERC20_1)
                    }
                    depositERC20_1
                },
                {
                    fn setUpdater(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <setUpdaterCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RolldownCalls::setUpdater)
                    }
                    setUpdater
                },
                {
                    fn DEFAULT_ADMIN_ROLE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::DEFAULT_ADMIN_ROLE)
                    }
                    DEFAULT_ADMIN_ROLE
                },
                {
                    fn hashFailedDepositResolution(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <hashFailedDepositResolutionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::hashFailedDepositResolution)
                    }
                    hashFailedDepositResolution
                },
                {
                    fn deposits(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <depositsCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RolldownCalls::deposits)
                    }
                    deposits
                },
                {
                    fn getUpdateForL2(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <getUpdateForL2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::getUpdateForL2)
                    }
                    getUpdateForL2
                },
                {
                    fn roots(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <rootsCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RolldownCalls::roots)
                    }
                    roots
                },
                {
                    fn chain(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <chainCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RolldownCalls::chain)
                    }
                    chain
                },
                {
                    fn deposit_native_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <deposit_native_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::deposit_native_0)
                    }
                    deposit_native_0
                },
                {
                    fn cancelResolutions(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <cancelResolutionsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::cancelResolutions)
                    }
                    cancelResolutions
                },
                {
                    fn hashCancel(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <hashCancelCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RolldownCalls::hashCancel)
                    }
                    hashCancel
                },
                {
                    fn findL2Batch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <findL2BatchCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::findL2Batch)
                    }
                    findL2Batch
                },
                {
                    fn deposit_erc20_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <deposit_erc20_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::deposit_erc20_1)
                    }
                    deposit_erc20_1
                },
                {
                    fn closeCancel(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <closeCancelCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::closeCancel)
                    }
                    closeCancel
                },
                {
                    fn revokeRole(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <revokeRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RolldownCalls::revokeRole)
                    }
                    revokeRole
                },
                {
                    fn depositNative_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <depositNative_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::depositNative_1)
                    }
                    depositNative_1
                },
                {
                    fn processedL2Requests(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <processedL2RequestsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::processedL2Requests)
                    }
                    processedL2Requests
                },
                {
                    fn NATIVE_TOKEN_ADDRESS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <NATIVE_TOKEN_ADDRESSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::NATIVE_TOKEN_ADDRESS)
                    }
                    NATIVE_TOKEN_ADDRESS
                },
                {
                    fn deposit_native_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <deposit_native_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::deposit_native_1)
                    }
                    deposit_native_1
                },
                {
                    fn lastProcessedUpdate_origin_l2(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <lastProcessedUpdate_origin_l2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RolldownCalls::lastProcessedUpdate_origin_l2)
                    }
                    lastProcessedUpdate_origin_l2
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RolldownCalls::initialize)
                    }
                    initialize
                },
                {
                    fn find_l2_batch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <find_l2_batchCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::find_l2_batch)
                    }
                    find_l2_batch
                },
                {
                    fn getMerkleRootsLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownCalls> {
                        <getMerkleRootsLengthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownCalls::getMerkleRootsLength)
                    }
                    getMerkleRootsLength
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::CLOSED(inner) => {
                    <CLOSEDCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::DEFAULT_ADMIN_ROLE(inner) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NATIVE_TOKEN_ADDRESS(inner) => {
                    <NATIVE_TOKEN_ADDRESSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UPDATER_ROLE(inner) => {
                    <UPDATER_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::cancelResolutions(inner) => {
                    <cancelResolutionsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::chain(inner) => {
                    <chainCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::closeCancel(inner) => {
                    <closeCancelCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::closeDepositRefund(inner) => {
                    <closeDepositRefundCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::closeWithdrawal(inner) => {
                    <closeWithdrawalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::close_cancel(inner) => {
                    <close_cancelCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::close_deposit_refund(inner) => {
                    <close_deposit_refundCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::close_withdrawal(inner) => {
                    <close_withdrawalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::counter(inner) => {
                    <counterCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::deposit_0(inner) => {
                    <deposit_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::deposit_1(inner) => {
                    <deposit_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::depositERC20_0(inner) => {
                    <depositERC20_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::depositERC20_1(inner) => {
                    <depositERC20_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::depositNative_0(inner) => {
                    <depositNative_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::depositNative_1(inner) => {
                    <depositNative_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deposit_erc20_0(inner) => {
                    <deposit_erc20_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deposit_erc20_1(inner) => {
                    <deposit_erc20_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deposit_native_0(inner) => {
                    <deposit_native_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deposit_native_1(inner) => {
                    <deposit_native_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deposits(inner) => {
                    <depositsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::ferryWithdrawal(inner) => {
                    <ferryWithdrawalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ferry_withdrawal(inner) => {
                    <ferry_withdrawalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::findL2Batch(inner) => {
                    <findL2BatchCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::find_l2_batch(inner) => {
                    <find_l2_batchCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMerkleRootsLength(inner) => {
                    <getMerkleRootsLengthCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPendingRequests(inner) => {
                    <getPendingRequestsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRoleAdmin(inner) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getUpdateForL2(inner) => {
                    <getUpdateForL2Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::grantRole(inner) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::hasRole(inner) => {
                    <hasRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::hashCancel(inner) => {
                    <hashCancelCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::hashFailedDepositResolution(inner) => {
                    <hashFailedDepositResolutionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::hashWithdrawal(inner) => {
                    <hashWithdrawalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::lastProcessedUpdate_origin_l1(inner) => {
                    <lastProcessedUpdate_origin_l1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::lastProcessedUpdate_origin_l2(inner) => {
                    <lastProcessedUpdate_origin_l2Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::merkleRootRange(inner) => {
                    <merkleRootRangeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::processedL2Requests(inner) => {
                    <processedL2RequestsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceRole(inner) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::revokeRole(inner) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::roots(inner) => {
                    <rootsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setUpdater(inner) => {
                    <setUpdaterCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::updateL1FromL2(inner) => {
                    <updateL1FromL2Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::update_l1_from_l2(inner) => {
                    <update_l1_from_l2Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updaterAccount(inner) => {
                    <updaterAccountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::CLOSED(inner) => {
                    <CLOSEDCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::DEFAULT_ADMIN_ROLE(inner) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::NATIVE_TOKEN_ADDRESS(inner) => {
                    <NATIVE_TOKEN_ADDRESSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::UPDATER_ROLE(inner) => {
                    <UPDATER_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::cancelResolutions(inner) => {
                    <cancelResolutionsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::chain(inner) => {
                    <chainCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::closeCancel(inner) => {
                    <closeCancelCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::closeDepositRefund(inner) => {
                    <closeDepositRefundCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::closeWithdrawal(inner) => {
                    <closeWithdrawalCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::close_cancel(inner) => {
                    <close_cancelCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::close_deposit_refund(inner) => {
                    <close_deposit_refundCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::close_withdrawal(inner) => {
                    <close_withdrawalCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::counter(inner) => {
                    <counterCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::deposit_0(inner) => {
                    <deposit_0Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::deposit_1(inner) => {
                    <deposit_1Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::depositERC20_0(inner) => {
                    <depositERC20_0Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::depositERC20_1(inner) => {
                    <depositERC20_1Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::depositNative_0(inner) => {
                    <depositNative_0Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::depositNative_1(inner) => {
                    <depositNative_1Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::deposit_erc20_0(inner) => {
                    <deposit_erc20_0Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::deposit_erc20_1(inner) => {
                    <deposit_erc20_1Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::deposit_native_0(inner) => {
                    <deposit_native_0Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::deposit_native_1(inner) => {
                    <deposit_native_1Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::deposits(inner) => {
                    <depositsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::ferryWithdrawal(inner) => {
                    <ferryWithdrawalCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::ferry_withdrawal(inner) => {
                    <ferry_withdrawalCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::findL2Batch(inner) => {
                    <findL2BatchCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::find_l2_batch(inner) => {
                    <find_l2_batchCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getMerkleRootsLength(inner) => {
                    <getMerkleRootsLengthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::getPendingRequests(inner) => {
                    <getPendingRequestsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getRoleAdmin(inner) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getUpdateForL2(inner) => {
                    <getUpdateForL2Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::grantRole(inner) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::hasRole(inner) => {
                    <hasRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::hashCancel(inner) => {
                    <hashCancelCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::hashFailedDepositResolution(inner) => {
                    <hashFailedDepositResolutionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::hashWithdrawal(inner) => {
                    <hashWithdrawalCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::lastProcessedUpdate_origin_l1(inner) => {
                    <lastProcessedUpdate_origin_l1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::lastProcessedUpdate_origin_l2(inner) => {
                    <lastProcessedUpdate_origin_l2Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::merkleRootRange(inner) => {
                    <merkleRootRangeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::processedL2Requests(inner) => {
                    <processedL2RequestsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::renounceRole(inner) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::revokeRole(inner) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::roots(inner) => {
                    <rootsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setUpdater(inner) => {
                    <setUpdaterCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::updateL1FromL2(inner) => {
                    <updateL1FromL2Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::update_l1_from_l2(inner) => {
                    <update_l1_from_l2Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::updaterAccount(inner) => {
                    <updaterAccountCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`Rolldown`](self) custom errors.
    pub enum RolldownErrors {
        FerryTipExceedsAmount(FerryTipExceedsAmount),
        InvalidFerriedAmount(InvalidFerriedAmount),
        InvalidRequestId(InvalidRequestId),
        InvalidRequestProof(InvalidRequestProof),
        InvalidRequestRange(InvalidRequestRange),
        InvalidUpdateRange(InvalidUpdateRange),
        L2RequestAlreadyProcessed(L2RequestAlreadyProcessed),
        PreviousUpdateMissed(PreviousUpdateMissed),
        RequestOutOfRange(RequestOutOfRange),
        RequestRangeTooLarge(RequestRangeTooLarge),
        UnexpectedMerkleRoot(UnexpectedMerkleRoot),
        UpdateAlreadyApplied(UpdateAlreadyApplied),
        WithdrawalAlreadyFerried(WithdrawalAlreadyFerried),
        ZeroAdmin(ZeroAdmin),
        ZeroAmount(ZeroAmount),
        ZeroRecipient(ZeroRecipient),
        ZeroRootCount(ZeroRootCount),
        ZeroToken(ZeroToken),
        ZeroTransferAmount(ZeroTransferAmount),
        ZeroUpdateRange(ZeroUpdateRange),
        ZeroUpdater(ZeroUpdater),
    }
    #[automatically_derived]
    impl RolldownErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [31u8, 42u8, 32u8, 5u8],
            [41u8, 197u8, 68u8, 41u8],
            [77u8, 52u8, 110u8, 137u8],
            [80u8, 167u8, 146u8, 177u8],
            [93u8, 67u8, 112u8, 117u8],
            [105u8, 241u8, 207u8, 239u8],
            [114u8, 137u8, 219u8, 14u8],
            [128u8, 172u8, 197u8, 164u8],
            [130u8, 86u8, 148u8, 244u8],
            [153u8, 213u8, 235u8, 166u8],
            [169u8, 105u8, 44u8, 30u8],
            [173u8, 25u8, 145u8, 245u8],
            [201u8, 105u8, 224u8, 242u8],
            [202u8, 0u8, 142u8, 96u8],
            [210u8, 123u8, 68u8, 67u8],
            [228u8, 29u8, 110u8, 132u8],
            [228u8, 95u8, 135u8, 238u8],
            [233u8, 151u8, 17u8, 241u8],
            [246u8, 174u8, 141u8, 83u8],
            [254u8, 165u8, 148u8, 83u8],
            [255u8, 134u8, 60u8, 163u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for RolldownErrors {
        const NAME: &'static str = "RolldownErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 21usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::FerryTipExceedsAmount(_) => {
                    <FerryTipExceedsAmount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidFerriedAmount(_) => {
                    <InvalidFerriedAmount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidRequestId(_) => {
                    <InvalidRequestId as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidRequestProof(_) => {
                    <InvalidRequestProof as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidRequestRange(_) => {
                    <InvalidRequestRange as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidUpdateRange(_) => {
                    <InvalidUpdateRange as alloy_sol_types::SolError>::SELECTOR
                }
                Self::L2RequestAlreadyProcessed(_) => {
                    <L2RequestAlreadyProcessed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::PreviousUpdateMissed(_) => {
                    <PreviousUpdateMissed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::RequestOutOfRange(_) => {
                    <RequestOutOfRange as alloy_sol_types::SolError>::SELECTOR
                }
                Self::RequestRangeTooLarge(_) => {
                    <RequestRangeTooLarge as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnexpectedMerkleRoot(_) => {
                    <UnexpectedMerkleRoot as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UpdateAlreadyApplied(_) => {
                    <UpdateAlreadyApplied as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WithdrawalAlreadyFerried(_) => {
                    <WithdrawalAlreadyFerried as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroAdmin(_) => <ZeroAdmin as alloy_sol_types::SolError>::SELECTOR,
                Self::ZeroAmount(_) => <ZeroAmount as alloy_sol_types::SolError>::SELECTOR,
                Self::ZeroRecipient(_) => <ZeroRecipient as alloy_sol_types::SolError>::SELECTOR,
                Self::ZeroRootCount(_) => <ZeroRootCount as alloy_sol_types::SolError>::SELECTOR,
                Self::ZeroToken(_) => <ZeroToken as alloy_sol_types::SolError>::SELECTOR,
                Self::ZeroTransferAmount(_) => {
                    <ZeroTransferAmount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroUpdateRange(_) => {
                    <ZeroUpdateRange as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroUpdater(_) => <ZeroUpdater as alloy_sol_types::SolError>::SELECTOR,
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(&[u8], bool) -> alloy_sol_types::Result<RolldownErrors>] = &[
                {
                    fn ZeroAmount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownErrors> {
                        <ZeroAmount as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(RolldownErrors::ZeroAmount)
                    }
                    ZeroAmount
                },
                {
                    fn ZeroTransferAmount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownErrors> {
                        <ZeroTransferAmount as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownErrors::ZeroTransferAmount)
                    }
                    ZeroTransferAmount
                },
                {
                    fn RequestOutOfRange(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownErrors> {
                        <RequestOutOfRange as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownErrors::RequestOutOfRange)
                    }
                    RequestOutOfRange
                },
                {
                    fn UpdateAlreadyApplied(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownErrors> {
                        <UpdateAlreadyApplied as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownErrors::UpdateAlreadyApplied)
                    }
                    UpdateAlreadyApplied
                },
                {
                    fn ZeroRootCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownErrors> {
                        <ZeroRootCount as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(RolldownErrors::ZeroRootCount)
                    }
                    ZeroRootCount
                },
                {
                    fn ZeroUpdateRange(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownErrors> {
                        <ZeroUpdateRange as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownErrors::ZeroUpdateRange)
                    }
                    ZeroUpdateRange
                },
                {
                    fn ZeroAdmin(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownErrors> {
                        <ZeroAdmin as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(RolldownErrors::ZeroAdmin)
                    }
                    ZeroAdmin
                },
                {
                    fn FerryTipExceedsAmount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownErrors> {
                        <FerryTipExceedsAmount as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownErrors::FerryTipExceedsAmount)
                    }
                    FerryTipExceedsAmount
                },
                {
                    fn RequestRangeTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownErrors> {
                        <RequestRangeTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownErrors::RequestRangeTooLarge)
                    }
                    RequestRangeTooLarge
                },
                {
                    fn InvalidFerriedAmount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownErrors> {
                        <InvalidFerriedAmount as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownErrors::InvalidFerriedAmount)
                    }
                    InvalidFerriedAmount
                },
                {
                    fn InvalidRequestRange(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownErrors> {
                        <InvalidRequestRange as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownErrors::InvalidRequestRange)
                    }
                    InvalidRequestRange
                },
                {
                    fn ZeroToken(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownErrors> {
                        <ZeroToken as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(RolldownErrors::ZeroToken)
                    }
                    ZeroToken
                },
                {
                    fn InvalidRequestId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownErrors> {
                        <InvalidRequestId as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownErrors::InvalidRequestId)
                    }
                    InvalidRequestId
                },
                {
                    fn PreviousUpdateMissed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownErrors> {
                        <PreviousUpdateMissed as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownErrors::PreviousUpdateMissed)
                    }
                    PreviousUpdateMissed
                },
                {
                    fn ZeroRecipient(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownErrors> {
                        <ZeroRecipient as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(RolldownErrors::ZeroRecipient)
                    }
                    ZeroRecipient
                },
                {
                    fn UnexpectedMerkleRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownErrors> {
                        <UnexpectedMerkleRoot as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownErrors::UnexpectedMerkleRoot)
                    }
                    UnexpectedMerkleRoot
                },
                {
                    fn InvalidUpdateRange(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownErrors> {
                        <InvalidUpdateRange as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownErrors::InvalidUpdateRange)
                    }
                    InvalidUpdateRange
                },
                {
                    fn L2RequestAlreadyProcessed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownErrors> {
                        <L2RequestAlreadyProcessed as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownErrors::L2RequestAlreadyProcessed)
                    }
                    L2RequestAlreadyProcessed
                },
                {
                    fn InvalidRequestProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownErrors> {
                        <InvalidRequestProof as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownErrors::InvalidRequestProof)
                    }
                    InvalidRequestProof
                },
                {
                    fn WithdrawalAlreadyFerried(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownErrors> {
                        <WithdrawalAlreadyFerried as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RolldownErrors::WithdrawalAlreadyFerried)
                    }
                    WithdrawalAlreadyFerried
                },
                {
                    fn ZeroUpdater(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RolldownErrors> {
                        <ZeroUpdater as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(RolldownErrors::ZeroUpdater)
                    }
                    ZeroUpdater
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::FerryTipExceedsAmount(inner) => {
                    <FerryTipExceedsAmount as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidFerriedAmount(inner) => {
                    <InvalidFerriedAmount as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidRequestId(inner) => {
                    <InvalidRequestId as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidRequestProof(inner) => {
                    <InvalidRequestProof as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidRequestRange(inner) => {
                    <InvalidRequestRange as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidUpdateRange(inner) => {
                    <InvalidUpdateRange as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::L2RequestAlreadyProcessed(inner) => {
                    <L2RequestAlreadyProcessed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PreviousUpdateMissed(inner) => {
                    <PreviousUpdateMissed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::RequestOutOfRange(inner) => {
                    <RequestOutOfRange as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::RequestRangeTooLarge(inner) => {
                    <RequestRangeTooLarge as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::UnexpectedMerkleRoot(inner) => {
                    <UnexpectedMerkleRoot as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::UpdateAlreadyApplied(inner) => {
                    <UpdateAlreadyApplied as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::WithdrawalAlreadyFerried(inner) => {
                    <WithdrawalAlreadyFerried as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ZeroAdmin(inner) => {
                    <ZeroAdmin as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ZeroAmount(inner) => {
                    <ZeroAmount as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ZeroRecipient(inner) => {
                    <ZeroRecipient as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ZeroRootCount(inner) => {
                    <ZeroRootCount as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ZeroToken(inner) => {
                    <ZeroToken as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ZeroTransferAmount(inner) => {
                    <ZeroTransferAmount as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ZeroUpdateRange(inner) => {
                    <ZeroUpdateRange as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ZeroUpdater(inner) => {
                    <ZeroUpdater as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::FerryTipExceedsAmount(inner) => {
                    <FerryTipExceedsAmount as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidFerriedAmount(inner) => {
                    <InvalidFerriedAmount as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidRequestId(inner) => {
                    <InvalidRequestId as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidRequestProof(inner) => {
                    <InvalidRequestProof as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidRequestRange(inner) => {
                    <InvalidRequestRange as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidUpdateRange(inner) => {
                    <InvalidUpdateRange as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::L2RequestAlreadyProcessed(inner) => {
                    <L2RequestAlreadyProcessed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::PreviousUpdateMissed(inner) => {
                    <PreviousUpdateMissed as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::RequestOutOfRange(inner) => {
                    <RequestOutOfRange as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::RequestRangeTooLarge(inner) => {
                    <RequestRangeTooLarge as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::UnexpectedMerkleRoot(inner) => {
                    <UnexpectedMerkleRoot as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::UpdateAlreadyApplied(inner) => {
                    <UpdateAlreadyApplied as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::WithdrawalAlreadyFerried(inner) => {
                    <WithdrawalAlreadyFerried as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::ZeroAdmin(inner) => {
                    <ZeroAdmin as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ZeroAmount(inner) => {
                    <ZeroAmount as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ZeroRecipient(inner) => {
                    <ZeroRecipient as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ZeroRootCount(inner) => {
                    <ZeroRootCount as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ZeroToken(inner) => {
                    <ZeroToken as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ZeroTransferAmount(inner) => {
                    <ZeroTransferAmount as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ZeroUpdateRange(inner) => {
                    <ZeroUpdateRange as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ZeroUpdater(inner) => {
                    <ZeroUpdater as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`Rolldown`](self) events.
    pub enum RolldownEvents {
        DepositAcceptedIntoQueue(DepositAcceptedIntoQueue),
        DisputeResolutionAcceptedIntoQueue(DisputeResolutionAcceptedIntoQueue),
        ERC20TokensWithdrawn(ERC20TokensWithdrawn),
        FailedDepositResolutionClosed(FailedDepositResolutionClosed),
        FerriedWithdrawalClosed(FerriedWithdrawalClosed),
        Initialized(Initialized),
        L2UpdateAccepted(L2UpdateAccepted),
        NativeTokensWithdrawn(NativeTokensWithdrawn),
        NewUpdaterSet(NewUpdaterSet),
        Paused(Paused),
        RoleAdminChanged(RoleAdminChanged),
        RoleGranted(RoleGranted),
        RoleRevoked(RoleRevoked),
        Unpaused(Unpaused),
        WithdrawalClosed(WithdrawalClosed),
        WithdrawalFerried(WithdrawalFerried),
    }
    #[automatically_derived]
    impl RolldownEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                0u8, 231u8, 99u8, 247u8, 119u8, 139u8, 140u8, 238u8, 247u8, 39u8, 12u8, 137u8,
                183u8, 209u8, 223u8, 16u8, 8u8, 176u8, 228u8, 130u8, 218u8, 57u8, 196u8, 56u8,
                49u8, 65u8, 119u8, 51u8, 175u8, 150u8, 251u8, 13u8,
            ],
            [
                19u8, 117u8, 12u8, 115u8, 31u8, 135u8, 193u8, 82u8, 66u8, 135u8, 76u8, 231u8, 75u8,
                244u8, 100u8, 149u8, 2u8, 204u8, 142u8, 124u8, 130u8, 144u8, 103u8, 206u8, 132u8,
                101u8, 5u8, 172u8, 219u8, 150u8, 40u8, 157u8,
            ],
            [
                27u8, 15u8, 47u8, 80u8, 13u8, 245u8, 150u8, 180u8, 43u8, 115u8, 232u8, 13u8, 190u8,
                198u8, 161u8, 251u8, 87u8, 15u8, 1u8, 151u8, 138u8, 88u8, 103u8, 35u8, 249u8,
                136u8, 165u8, 252u8, 84u8, 215u8, 115u8, 161u8,
            ],
            [
                34u8, 83u8, 5u8, 236u8, 182u8, 111u8, 169u8, 185u8, 178u8, 159u8, 141u8, 234u8,
                217u8, 186u8, 234u8, 54u8, 90u8, 108u8, 34u8, 93u8, 99u8, 157u8, 253u8, 134u8,
                110u8, 120u8, 44u8, 203u8, 99u8, 226u8, 240u8, 91u8,
            ],
            [
                41u8, 150u8, 253u8, 84u8, 108u8, 55u8, 215u8, 76u8, 23u8, 4u8, 102u8, 234u8, 106u8,
                164u8, 163u8, 8u8, 227u8, 202u8, 45u8, 74u8, 166u8, 137u8, 230u8, 233u8, 227u8,
                41u8, 148u8, 219u8, 80u8, 57u8, 204u8, 14u8,
            ],
            [
                47u8, 135u8, 136u8, 17u8, 126u8, 126u8, 255u8, 29u8, 130u8, 233u8, 38u8, 236u8,
                121u8, 73u8, 1u8, 209u8, 124u8, 120u8, 2u8, 74u8, 80u8, 39u8, 9u8, 64u8, 48u8,
                69u8, 64u8, 167u8, 51u8, 101u8, 111u8, 13u8,
            ],
            [
                73u8, 193u8, 88u8, 212u8, 144u8, 219u8, 158u8, 6u8, 111u8, 1u8, 181u8, 212u8,
                241u8, 160u8, 148u8, 72u8, 90u8, 101u8, 152u8, 203u8, 108u8, 82u8, 150u8, 180u8,
                192u8, 126u8, 70u8, 193u8, 42u8, 29u8, 193u8, 28u8,
            ],
            [
                93u8, 185u8, 238u8, 10u8, 73u8, 91u8, 242u8, 230u8, 255u8, 156u8, 145u8, 167u8,
                131u8, 76u8, 27u8, 164u8, 253u8, 210u8, 68u8, 165u8, 232u8, 170u8, 78u8, 83u8,
                123u8, 211u8, 138u8, 234u8, 228u8, 176u8, 115u8, 170u8,
            ],
            [
                98u8, 231u8, 140u8, 234u8, 1u8, 190u8, 227u8, 32u8, 205u8, 78u8, 66u8, 2u8, 112u8,
                181u8, 234u8, 116u8, 0u8, 13u8, 17u8, 176u8, 201u8, 247u8, 71u8, 84u8, 235u8,
                219u8, 252u8, 84u8, 75u8, 5u8, 162u8, 88u8,
            ],
            [
                122u8, 154u8, 189u8, 158u8, 184u8, 107u8, 219u8, 202u8, 137u8, 203u8, 164u8, 6u8,
                154u8, 99u8, 44u8, 55u8, 217u8, 61u8, 184u8, 46u8, 62u8, 20u8, 173u8, 129u8, 25u8,
                163u8, 167u8, 129u8, 40u8, 20u8, 133u8, 62u8,
            ],
            [
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8, 56u8,
                82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8, 96u8,
                206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ],
            [
                147u8, 95u8, 38u8, 217u8, 75u8, 227u8, 25u8, 7u8, 8u8, 10u8, 167u8, 139u8, 62u8,
                110u8, 42u8, 198u8, 212u8, 138u8, 7u8, 42u8, 240u8, 150u8, 194u8, 2u8, 104u8, 56u8,
                134u8, 33u8, 187u8, 193u8, 23u8, 137u8,
            ],
            [
                158u8, 241u8, 19u8, 83u8, 175u8, 217u8, 125u8, 51u8, 154u8, 119u8, 115u8, 40u8,
                80u8, 183u8, 194u8, 39u8, 4u8, 101u8, 101u8, 88u8, 217u8, 186u8, 99u8, 204u8,
                126u8, 50u8, 30u8, 10u8, 196u8, 194u8, 10u8, 169u8,
            ],
            [
                189u8, 121u8, 184u8, 111u8, 254u8, 10u8, 184u8, 232u8, 119u8, 97u8, 81u8, 81u8,
                66u8, 23u8, 205u8, 124u8, 172u8, 213u8, 44u8, 144u8, 159u8, 102u8, 71u8, 92u8,
                58u8, 244u8, 78u8, 18u8, 159u8, 11u8, 0u8, 255u8,
            ],
            [
                224u8, 73u8, 83u8, 85u8, 193u8, 224u8, 76u8, 81u8, 37u8, 132u8, 82u8, 24u8, 84u8,
                210u8, 34u8, 210u8, 57u8, 164u8, 183u8, 130u8, 179u8, 154u8, 200u8, 167u8, 232u8,
                53u8, 163u8, 79u8, 94u8, 199u8, 193u8, 225u8,
            ],
            [
                246u8, 57u8, 31u8, 92u8, 50u8, 217u8, 198u8, 157u8, 42u8, 71u8, 234u8, 103u8, 11u8,
                68u8, 41u8, 116u8, 181u8, 57u8, 53u8, 209u8, 237u8, 199u8, 253u8, 100u8, 235u8,
                33u8, 224u8, 71u8, 168u8, 57u8, 23u8, 27u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for RolldownEvents {
        const NAME: &'static str = "RolldownEvents";
        const COUNT: usize = 16usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <DepositAcceptedIntoQueue as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DepositAcceptedIntoQueue as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::DepositAcceptedIntoQueue)
                }
                Some(
                    <DisputeResolutionAcceptedIntoQueue as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DisputeResolutionAcceptedIntoQueue as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::DisputeResolutionAcceptedIntoQueue)
                }
                Some(
                    <ERC20TokensWithdrawn as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ERC20TokensWithdrawn as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ERC20TokensWithdrawn)
                }
                Some(
                    <FailedDepositResolutionClosed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <FailedDepositResolutionClosed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::FailedDepositResolutionClosed)
                }
                Some(
                    <FerriedWithdrawalClosed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <FerriedWithdrawalClosed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::FerriedWithdrawalClosed)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Initialized)
                }
                Some(<L2UpdateAccepted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <L2UpdateAccepted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::L2UpdateAccepted)
                }
                Some(
                    <NativeTokensWithdrawn as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <NativeTokensWithdrawn as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::NativeTokensWithdrawn)
                }
                Some(<NewUpdaterSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NewUpdaterSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::NewUpdaterSet)
                }
                Some(<Paused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Paused as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Paused)
                }
                Some(<RoleAdminChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleAdminChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RoleAdminChanged)
                }
                Some(<RoleGranted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleGranted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RoleGranted)
                }
                Some(<RoleRevoked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleRevoked as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RoleRevoked)
                }
                Some(<Unpaused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Unpaused as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Unpaused)
                }
                Some(<WithdrawalClosed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <WithdrawalClosed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::WithdrawalClosed)
                }
                Some(
                    <WithdrawalFerried as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <WithdrawalFerried as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::WithdrawalFerried)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for RolldownEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::DepositAcceptedIntoQueue(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DisputeResolutionAcceptedIntoQueue(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ERC20TokensWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::FailedDepositResolutionClosed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::FerriedWithdrawalClosed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::L2UpdateAccepted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NativeTokensWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NewUpdaterSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::RoleAdminChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleGranted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleRevoked(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::WithdrawalClosed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::WithdrawalFerried(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::DepositAcceptedIntoQueue(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DisputeResolutionAcceptedIntoQueue(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ERC20TokensWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::FailedDepositResolutionClosed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::FerriedWithdrawalClosed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::L2UpdateAccepted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NativeTokensWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NewUpdaterSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
                Self::RoleAdminChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleGranted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleRevoked(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::WithdrawalClosed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::WithdrawalFerried(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`Rolldown`](self) contract instance.

    See the [wrapper's documentation](`RolldownInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> RolldownInstance<T, P, N> {
        RolldownInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

    Returns a new instance of the contract, if the deployment was successful.

    For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<RolldownInstance<T, P, N>>>
    {
        RolldownInstance::<T, P, N>::deploy(provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
    and constructor arguments, if any.

    This is a simple wrapper around creating a `RawCallBuilder` with the data set to
    the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        RolldownInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`Rolldown`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`Rolldown`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct RolldownInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for RolldownInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("RolldownInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > RolldownInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`Rolldown`](self) contract instance.

        See the [wrapper's documentation](`RolldownInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

        Returns a new instance of the contract, if the deployment was successful.

        For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(provider: P) -> alloy_contract::Result<RolldownInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
        and constructor arguments, if any.

        This is a simple wrapper around creating a `RawCallBuilder` with the data set to
        the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> RolldownInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> RolldownInstance<T, P, N> {
            RolldownInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > RolldownInstance<T, P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`CLOSED`] function.
        pub fn CLOSED(&self) -> alloy_contract::SolCallBuilder<T, &P, CLOSEDCall, N> {
            self.call_builder(&CLOSEDCall {})
        }
        ///Creates a new call builder for the [`DEFAULT_ADMIN_ROLE`] function.
        pub fn DEFAULT_ADMIN_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, DEFAULT_ADMIN_ROLECall, N> {
            self.call_builder(&DEFAULT_ADMIN_ROLECall {})
        }
        ///Creates a new call builder for the [`NATIVE_TOKEN_ADDRESS`] function.
        pub fn NATIVE_TOKEN_ADDRESS(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, NATIVE_TOKEN_ADDRESSCall, N> {
            self.call_builder(&NATIVE_TOKEN_ADDRESSCall {})
        }
        ///Creates a new call builder for the [`UPDATER_ROLE`] function.
        pub fn UPDATER_ROLE(&self) -> alloy_contract::SolCallBuilder<T, &P, UPDATER_ROLECall, N> {
            self.call_builder(&UPDATER_ROLECall {})
        }
        ///Creates a new call builder for the [`cancelResolutions`] function.
        pub fn cancelResolutions(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, cancelResolutionsCall, N> {
            self.call_builder(&cancelResolutionsCall { _0 })
        }
        ///Creates a new call builder for the [`chain`] function.
        pub fn chain(&self) -> alloy_contract::SolCallBuilder<T, &P, chainCall, N> {
            self.call_builder(&chainCall {})
        }
        ///Creates a new call builder for the [`closeCancel`] function.
        pub fn closeCancel(
            &self,
            cancel: <IRolldownPrimitives::Cancel as alloy::sol_types::SolType>::RustType,
            merkleRoot: alloy::sol_types::private::FixedBytes<32>,
            proof: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        ) -> alloy_contract::SolCallBuilder<T, &P, closeCancelCall, N> {
            self.call_builder(&closeCancelCall {
                cancel,
                merkleRoot,
                proof,
            })
        }
        ///Creates a new call builder for the [`closeDepositRefund`] function.
        pub fn closeDepositRefund(
            &self,
            failedDeposit: <IRolldownPrimitives::FailedDepositResolution as alloy::sol_types::SolType>::RustType,
            merkleRoot: alloy::sol_types::private::FixedBytes<32>,
            proof: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        ) -> alloy_contract::SolCallBuilder<T, &P, closeDepositRefundCall, N> {
            self.call_builder(&closeDepositRefundCall {
                failedDeposit,
                merkleRoot,
                proof,
            })
        }
        ///Creates a new call builder for the [`closeWithdrawal`] function.
        pub fn closeWithdrawal(
            &self,
            withdrawal: <IRolldownPrimitives::Withdrawal as alloy::sol_types::SolType>::RustType,
            merkleRoot: alloy::sol_types::private::FixedBytes<32>,
            proof: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        ) -> alloy_contract::SolCallBuilder<T, &P, closeWithdrawalCall, N> {
            self.call_builder(&closeWithdrawalCall {
                withdrawal,
                merkleRoot,
                proof,
            })
        }
        ///Creates a new call builder for the [`close_cancel`] function.
        pub fn close_cancel(
            &self,
            cancel: <IRolldownPrimitives::Cancel as alloy::sol_types::SolType>::RustType,
            merkleRoot: alloy::sol_types::private::FixedBytes<32>,
            proof: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        ) -> alloy_contract::SolCallBuilder<T, &P, close_cancelCall, N> {
            self.call_builder(&close_cancelCall {
                cancel,
                merkleRoot,
                proof,
            })
        }
        ///Creates a new call builder for the [`close_deposit_refund`] function.
        pub fn close_deposit_refund(
            &self,
            failedDeposit: <IRolldownPrimitives::FailedDepositResolution as alloy::sol_types::SolType>::RustType,
            merkleRoot: alloy::sol_types::private::FixedBytes<32>,
            proof: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        ) -> alloy_contract::SolCallBuilder<T, &P, close_deposit_refundCall, N> {
            self.call_builder(&close_deposit_refundCall {
                failedDeposit,
                merkleRoot,
                proof,
            })
        }
        ///Creates a new call builder for the [`close_withdrawal`] function.
        pub fn close_withdrawal(
            &self,
            withdrawal: <IRolldownPrimitives::Withdrawal as alloy::sol_types::SolType>::RustType,
            merkleRoot: alloy::sol_types::private::FixedBytes<32>,
            proof: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        ) -> alloy_contract::SolCallBuilder<T, &P, close_withdrawalCall, N> {
            self.call_builder(&close_withdrawalCall {
                withdrawal,
                merkleRoot,
                proof,
            })
        }
        ///Creates a new call builder for the [`counter`] function.
        pub fn counter(&self) -> alloy_contract::SolCallBuilder<T, &P, counterCall, N> {
            self.call_builder(&counterCall {})
        }
        ///Creates a new call builder for the [`deposit_0`] function.
        pub fn deposit_0(
            &self,
            tokenAddress: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
            ferryTip: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, deposit_0Call, N> {
            self.call_builder(&deposit_0Call {
                tokenAddress,
                amount,
                ferryTip,
            })
        }
        ///Creates a new call builder for the [`deposit_1`] function.
        pub fn deposit_1(
            &self,
            tokenAddress: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, deposit_1Call, N> {
            self.call_builder(&deposit_1Call {
                tokenAddress,
                amount,
            })
        }
        ///Creates a new call builder for the [`depositERC20_0`] function.
        pub fn depositERC20_0(
            &self,
            tokenAddress: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
            ferryTip: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, depositERC20_0Call, N> {
            self.call_builder(&depositERC20_0Call {
                tokenAddress,
                amount,
                ferryTip,
            })
        }
        ///Creates a new call builder for the [`depositERC20_1`] function.
        pub fn depositERC20_1(
            &self,
            tokenAddress: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, depositERC20_1Call, N> {
            self.call_builder(&depositERC20_1Call {
                tokenAddress,
                amount,
            })
        }
        ///Creates a new call builder for the [`depositNative_0`] function.
        pub fn depositNative_0(
            &self,
            ferryTip: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, depositNative_0Call, N> {
            self.call_builder(&depositNative_0Call { ferryTip })
        }
        ///Creates a new call builder for the [`depositNative_1`] function.
        pub fn depositNative_1(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, depositNative_1Call, N> {
            self.call_builder(&depositNative_1Call {})
        }
        ///Creates a new call builder for the [`deposit_erc20_0`] function.
        pub fn deposit_erc20_0(
            &self,
            tokenAddress: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
            ferryTip: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, deposit_erc20_0Call, N> {
            self.call_builder(&deposit_erc20_0Call {
                tokenAddress,
                amount,
                ferryTip,
            })
        }
        ///Creates a new call builder for the [`deposit_erc20_1`] function.
        pub fn deposit_erc20_1(
            &self,
            tokenAddress: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, deposit_erc20_1Call, N> {
            self.call_builder(&deposit_erc20_1Call {
                tokenAddress,
                amount,
            })
        }
        ///Creates a new call builder for the [`deposit_native_0`] function.
        pub fn deposit_native_0(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, deposit_native_0Call, N> {
            self.call_builder(&deposit_native_0Call {})
        }
        ///Creates a new call builder for the [`deposit_native_1`] function.
        pub fn deposit_native_1(
            &self,
            ferryTip: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, deposit_native_1Call, N> {
            self.call_builder(&deposit_native_1Call { ferryTip })
        }
        ///Creates a new call builder for the [`deposits`] function.
        pub fn deposits(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, depositsCall, N> {
            self.call_builder(&depositsCall { _0 })
        }
        ///Creates a new call builder for the [`ferryWithdrawal`] function.
        pub fn ferryWithdrawal(
            &self,
            withdrawal: <IRolldownPrimitives::Withdrawal as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, ferryWithdrawalCall, N> {
            self.call_builder(&ferryWithdrawalCall { withdrawal })
        }
        ///Creates a new call builder for the [`ferry_withdrawal`] function.
        pub fn ferry_withdrawal(
            &self,
            withdrawal: <IRolldownPrimitives::Withdrawal as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, ferry_withdrawalCall, N> {
            self.call_builder(&ferry_withdrawalCall { withdrawal })
        }
        ///Creates a new call builder for the [`findL2Batch`] function.
        pub fn findL2Batch(
            &self,
            requestId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, findL2BatchCall, N> {
            self.call_builder(&findL2BatchCall { requestId })
        }
        ///Creates a new call builder for the [`find_l2_batch`] function.
        pub fn find_l2_batch(
            &self,
            requestId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, find_l2_batchCall, N> {
            self.call_builder(&find_l2_batchCall { requestId })
        }
        ///Creates a new call builder for the [`getMerkleRootsLength`] function.
        pub fn getMerkleRootsLength(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMerkleRootsLengthCall, N> {
            self.call_builder(&getMerkleRootsLengthCall {})
        }
        ///Creates a new call builder for the [`getPendingRequests`] function.
        pub fn getPendingRequests(
            &self,
            start: alloy::sol_types::private::primitives::aliases::U256,
            end: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPendingRequestsCall, N> {
            self.call_builder(&getPendingRequestsCall { start, end })
        }
        ///Creates a new call builder for the [`getRoleAdmin`] function.
        pub fn getRoleAdmin(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRoleAdminCall, N> {
            self.call_builder(&getRoleAdminCall { role })
        }
        ///Creates a new call builder for the [`getUpdateForL2`] function.
        pub fn getUpdateForL2(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getUpdateForL2Call, N> {
            self.call_builder(&getUpdateForL2Call {})
        }
        ///Creates a new call builder for the [`grantRole`] function.
        pub fn grantRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, grantRoleCall, N> {
            self.call_builder(&grantRoleCall { role, account })
        }
        ///Creates a new call builder for the [`hasRole`] function.
        pub fn hasRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, hasRoleCall, N> {
            self.call_builder(&hasRoleCall { role, account })
        }
        ///Creates a new call builder for the [`hashCancel`] function.
        pub fn hashCancel(
            &self,
            cancel: <IRolldownPrimitives::Cancel as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, hashCancelCall, N> {
            self.call_builder(&hashCancelCall { cancel })
        }
        ///Creates a new call builder for the [`hashFailedDepositResolution`] function.
        pub fn hashFailedDepositResolution(
            &self,
            failedDeposit: <IRolldownPrimitives::FailedDepositResolution as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, hashFailedDepositResolutionCall, N> {
            self.call_builder(&hashFailedDepositResolutionCall { failedDeposit })
        }
        ///Creates a new call builder for the [`hashWithdrawal`] function.
        pub fn hashWithdrawal(
            &self,
            withdrawal: <IRolldownPrimitives::Withdrawal as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, hashWithdrawalCall, N> {
            self.call_builder(&hashWithdrawalCall { withdrawal })
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            admin: alloy::sol_types::private::Address,
            chainId: <IRolldownPrimitives::ChainId as alloy::sol_types::SolType>::RustType,
            updater: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall {
                admin,
                chainId,
                updater,
            })
        }
        ///Creates a new call builder for the [`lastProcessedUpdate_origin_l1`] function.
        pub fn lastProcessedUpdate_origin_l1(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, lastProcessedUpdate_origin_l1Call, N> {
            self.call_builder(&lastProcessedUpdate_origin_l1Call {})
        }
        ///Creates a new call builder for the [`lastProcessedUpdate_origin_l2`] function.
        pub fn lastProcessedUpdate_origin_l2(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, lastProcessedUpdate_origin_l2Call, N> {
            self.call_builder(&lastProcessedUpdate_origin_l2Call {})
        }
        ///Creates a new call builder for the [`merkleRootRange`] function.
        pub fn merkleRootRange(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, merkleRootRangeCall, N> {
            self.call_builder(&merkleRootRangeCall { _0 })
        }
        ///Creates a new call builder for the [`pause`] function.
        pub fn pause(&self) -> alloy_contract::SolCallBuilder<T, &P, pauseCall, N> {
            self.call_builder(&pauseCall {})
        }
        ///Creates a new call builder for the [`paused`] function.
        pub fn paused(&self) -> alloy_contract::SolCallBuilder<T, &P, pausedCall, N> {
            self.call_builder(&pausedCall {})
        }
        ///Creates a new call builder for the [`processedL2Requests`] function.
        pub fn processedL2Requests(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, processedL2RequestsCall, N> {
            self.call_builder(&processedL2RequestsCall { _0 })
        }
        ///Creates a new call builder for the [`renounceRole`] function.
        pub fn renounceRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceRoleCall, N> {
            self.call_builder(&renounceRoleCall { role, account })
        }
        ///Creates a new call builder for the [`revokeRole`] function.
        pub fn revokeRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, revokeRoleCall, N> {
            self.call_builder(&revokeRoleCall { role, account })
        }
        ///Creates a new call builder for the [`roots`] function.
        pub fn roots(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, rootsCall, N> {
            self.call_builder(&rootsCall { _0 })
        }
        ///Creates a new call builder for the [`setUpdater`] function.
        pub fn setUpdater(
            &self,
            updater: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setUpdaterCall, N> {
            self.call_builder(&setUpdaterCall { updater })
        }
        ///Creates a new call builder for the [`supportsInterface`] function.
        pub fn supportsInterface(
            &self,
            interfaceId: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::SolCallBuilder<T, &P, supportsInterfaceCall, N> {
            self.call_builder(&supportsInterfaceCall { interfaceId })
        }
        ///Creates a new call builder for the [`unpause`] function.
        pub fn unpause(&self) -> alloy_contract::SolCallBuilder<T, &P, unpauseCall, N> {
            self.call_builder(&unpauseCall {})
        }
        ///Creates a new call builder for the [`updateL1FromL2`] function.
        pub fn updateL1FromL2(
            &self,
            merkleRoot: alloy::sol_types::private::FixedBytes<32>,
            range: <IRolldownPrimitives::Range as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateL1FromL2Call, N> {
            self.call_builder(&updateL1FromL2Call { merkleRoot, range })
        }
        ///Creates a new call builder for the [`update_l1_from_l2`] function.
        pub fn update_l1_from_l2(
            &self,
            merkleRoot: alloy::sol_types::private::FixedBytes<32>,
            range: <IRolldownPrimitives::Range as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, update_l1_from_l2Call, N> {
            self.call_builder(&update_l1_from_l2Call { merkleRoot, range })
        }
        ///Creates a new call builder for the [`updaterAccount`] function.
        pub fn updaterAccount(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, updaterAccountCall, N> {
            self.call_builder(&updaterAccountCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > RolldownInstance<T, P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`DepositAcceptedIntoQueue`] event.
        pub fn DepositAcceptedIntoQueue_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, DepositAcceptedIntoQueue, N> {
            self.event_filter::<DepositAcceptedIntoQueue>()
        }
        ///Creates a new event filter for the [`DisputeResolutionAcceptedIntoQueue`] event.
        pub fn DisputeResolutionAcceptedIntoQueue_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, DisputeResolutionAcceptedIntoQueue, N> {
            self.event_filter::<DisputeResolutionAcceptedIntoQueue>()
        }
        ///Creates a new event filter for the [`ERC20TokensWithdrawn`] event.
        pub fn ERC20TokensWithdrawn_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ERC20TokensWithdrawn, N> {
            self.event_filter::<ERC20TokensWithdrawn>()
        }
        ///Creates a new event filter for the [`FailedDepositResolutionClosed`] event.
        pub fn FailedDepositResolutionClosed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, FailedDepositResolutionClosed, N> {
            self.event_filter::<FailedDepositResolutionClosed>()
        }
        ///Creates a new event filter for the [`FerriedWithdrawalClosed`] event.
        pub fn FerriedWithdrawalClosed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, FerriedWithdrawalClosed, N> {
            self.event_filter::<FerriedWithdrawalClosed>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`L2UpdateAccepted`] event.
        pub fn L2UpdateAccepted_filter(&self) -> alloy_contract::Event<T, &P, L2UpdateAccepted, N> {
            self.event_filter::<L2UpdateAccepted>()
        }
        ///Creates a new event filter for the [`NativeTokensWithdrawn`] event.
        pub fn NativeTokensWithdrawn_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, NativeTokensWithdrawn, N> {
            self.event_filter::<NativeTokensWithdrawn>()
        }
        ///Creates a new event filter for the [`NewUpdaterSet`] event.
        pub fn NewUpdaterSet_filter(&self) -> alloy_contract::Event<T, &P, NewUpdaterSet, N> {
            self.event_filter::<NewUpdaterSet>()
        }
        ///Creates a new event filter for the [`Paused`] event.
        pub fn Paused_filter(&self) -> alloy_contract::Event<T, &P, Paused, N> {
            self.event_filter::<Paused>()
        }
        ///Creates a new event filter for the [`RoleAdminChanged`] event.
        pub fn RoleAdminChanged_filter(&self) -> alloy_contract::Event<T, &P, RoleAdminChanged, N> {
            self.event_filter::<RoleAdminChanged>()
        }
        ///Creates a new event filter for the [`RoleGranted`] event.
        pub fn RoleGranted_filter(&self) -> alloy_contract::Event<T, &P, RoleGranted, N> {
            self.event_filter::<RoleGranted>()
        }
        ///Creates a new event filter for the [`RoleRevoked`] event.
        pub fn RoleRevoked_filter(&self) -> alloy_contract::Event<T, &P, RoleRevoked, N> {
            self.event_filter::<RoleRevoked>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<T, &P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
        ///Creates a new event filter for the [`WithdrawalClosed`] event.
        pub fn WithdrawalClosed_filter(&self) -> alloy_contract::Event<T, &P, WithdrawalClosed, N> {
            self.event_filter::<WithdrawalClosed>()
        }
        ///Creates a new event filter for the [`WithdrawalFerried`] event.
        pub fn WithdrawalFerried_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, WithdrawalFerried, N> {
            self.event_filter::<WithdrawalFerried>()
        }
    }
}
