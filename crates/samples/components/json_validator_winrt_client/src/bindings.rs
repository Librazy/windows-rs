// Bindings generated by `windows-bindgen` 0.52.0

#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
::windows_core::imp::com_interface!(
    IJsonValidator,
    IJsonValidator_Vtbl,
    0xe09cb12b_b13c_5139_8c99_6140bf80deb9
);
#[repr(C)]
pub struct IJsonValidator_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Validate: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IJsonValidatorFactory,
    IJsonValidatorFactory_Vtbl,
    0x1cf4464e_ae9e_55d5_9539_0af4d8fc35aa
);
#[repr(C)]
pub struct IJsonValidatorFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct JsonValidator(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(
    JsonValidator,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl JsonValidator {
    pub fn Validate(
        &self,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Validate)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
                &mut result__,
            )
            .and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance(
        schema: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<JsonValidator> {
        Self::IJsonValidatorFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(schema),
                &mut result__,
            )
            .and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IJsonValidatorFactory<
        R,
        F: FnOnce(&IJsonValidatorFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<JsonValidator, IJsonValidatorFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for JsonValidator {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for JsonValidator {
    type Vtable = IJsonValidator_Vtbl;
    const IID: ::windows_core::GUID = <IJsonValidator as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for JsonValidator {
    const NAME: &'static str = "Sample.JsonValidator";
}
unsafe impl ::core::marker::Send for JsonValidator {}
unsafe impl ::core::marker::Sync for JsonValidator {}
