::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsAddRef ( r#ref : *const ::core::ffi::c_void , count : *mut u32 ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsBoolToBoolean ( value : u8 , booleanvalue : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsBooleanToBool ( value : *const ::core::ffi::c_void , boolvalue : *mut bool ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsCallFunction ( function : *const ::core::ffi::c_void , arguments : *const *const ::core::ffi::c_void , argumentcount : u16 , result : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsCollectGarbage ( runtime : *const ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsConstructObject ( function : *const ::core::ffi::c_void , arguments : *const *const ::core::ffi::c_void , argumentcount : u16 , result : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsConvertValueToBoolean ( value : *const ::core::ffi::c_void , booleanvalue : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsConvertValueToNumber ( value : *const ::core::ffi::c_void , numbervalue : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsConvertValueToObject ( value : *const ::core::ffi::c_void , object : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsConvertValueToString ( value : *const ::core::ffi::c_void , stringvalue : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsCreateArray ( length : u32 , result : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`, `\"Win32_System_Diagnostics_Debug\"`*"] fn JsCreateContext ( runtime : *const ::core::ffi::c_void , debugapplication : super::Diagnostics::Debug:: IDebugApplication64 , newcontext : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`, `\"Win32_System_Diagnostics_Debug\"`*"] fn JsCreateContext ( runtime : *const ::core::ffi::c_void , debugapplication : super::Diagnostics::Debug:: IDebugApplication32 , newcontext : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsCreateError ( message : *const ::core::ffi::c_void , error : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsCreateExternalObject ( data : *const ::core::ffi::c_void , finalizecallback : JsFinalizeCallback , object : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsCreateFunction ( nativefunction : JsNativeFunction , callbackstate : *const ::core::ffi::c_void , function : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsCreateObject ( object : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsCreateRangeError ( message : *const ::core::ffi::c_void , error : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsCreateReferenceError ( message : *const ::core::ffi::c_void , error : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsCreateRuntime ( attributes : JsRuntimeAttributes , runtimeversion : JsRuntimeVersion , threadservice : JsThreadServiceCallback , runtime : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsCreateSyntaxError ( message : *const ::core::ffi::c_void , error : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsCreateTypeError ( message : *const ::core::ffi::c_void , error : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsCreateURIError ( message : *const ::core::ffi::c_void , error : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsDefineProperty ( object : *const ::core::ffi::c_void , propertyid : *const ::core::ffi::c_void , propertydescriptor : *const ::core::ffi::c_void , result : *mut bool ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsDeleteIndexedProperty ( object : *const ::core::ffi::c_void , index : *const ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsDeleteProperty ( object : *const ::core::ffi::c_void , propertyid : *const ::core::ffi::c_void , usestrictrules : u8 , result : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsDisableRuntimeExecution ( runtime : *const ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsDisposeRuntime ( runtime : *const ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsDoubleToNumber ( doublevalue : f64 , value : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsEnableRuntimeExecution ( runtime : *const ::core::ffi::c_void ) -> JsErrorCode );
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`, `\"Win32_System_Diagnostics_Debug\"`*"] fn JsEnumerateHeap ( enumerator : *mut super::Diagnostics::Debug:: IActiveScriptProfilerHeapEnum ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsEquals ( object1 : *const ::core::ffi::c_void , object2 : *const ::core::ffi::c_void , result : *mut bool ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsGetAndClearException ( exception : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsGetCurrentContext ( currentcontext : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsGetExtensionAllowed ( object : *const ::core::ffi::c_void , value : *mut bool ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsGetExternalData ( object : *const ::core::ffi::c_void , externaldata : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsGetFalseValue ( falsevalue : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsGetGlobalObject ( globalobject : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsGetIndexedProperty ( object : *const ::core::ffi::c_void , index : *const ::core::ffi::c_void , result : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsGetNullValue ( nullvalue : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsGetOwnPropertyDescriptor ( object : *const ::core::ffi::c_void , propertyid : *const ::core::ffi::c_void , propertydescriptor : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsGetOwnPropertyNames ( object : *const ::core::ffi::c_void , propertynames : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsGetProperty ( object : *const ::core::ffi::c_void , propertyid : *const ::core::ffi::c_void , value : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsGetPropertyIdFromName ( name : ::windows_sys::core::PCWSTR , propertyid : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsGetPropertyNameFromId ( propertyid : *const ::core::ffi::c_void , name : *mut *mut u16 ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsGetPrototype ( object : *const ::core::ffi::c_void , prototypeobject : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsGetRuntime ( context : *const ::core::ffi::c_void , runtime : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsGetRuntimeMemoryLimit ( runtime : *const ::core::ffi::c_void , memorylimit : *mut usize ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsGetRuntimeMemoryUsage ( runtime : *const ::core::ffi::c_void , memoryusage : *mut usize ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsGetStringLength ( stringvalue : *const ::core::ffi::c_void , length : *mut i32 ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsGetTrueValue ( truevalue : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsGetUndefinedValue ( undefinedvalue : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsGetValueType ( value : *const ::core::ffi::c_void , r#type : *mut JsValueType ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsHasException ( hasexception : *mut bool ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsHasExternalData ( object : *const ::core::ffi::c_void , value : *mut bool ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsHasIndexedProperty ( object : *const ::core::ffi::c_void , index : *const ::core::ffi::c_void , result : *mut bool ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsHasProperty ( object : *const ::core::ffi::c_void , propertyid : *const ::core::ffi::c_void , hasproperty : *mut bool ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsIdle ( nextidletick : *mut u32 ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsIntToNumber ( intvalue : i32 , value : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsIsEnumeratingHeap ( isenumeratingheap : *mut bool ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsIsRuntimeExecutionDisabled ( runtime : *const ::core::ffi::c_void , isdisabled : *mut bool ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsNumberToDouble ( value : *const ::core::ffi::c_void , doublevalue : *mut f64 ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsParseScript ( script : ::windows_sys::core::PCWSTR , sourcecontext : usize , sourceurl : ::windows_sys::core::PCWSTR , result : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsParseSerializedScript ( script : ::windows_sys::core::PCWSTR , buffer : *const u8 , sourcecontext : usize , sourceurl : ::windows_sys::core::PCWSTR , result : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsPointerToString ( stringvalue : ::windows_sys::core::PCWSTR , stringlength : usize , value : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsPreventExtension ( object : *const ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsRelease ( r#ref : *const ::core::ffi::c_void , count : *mut u32 ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsRunScript ( script : ::windows_sys::core::PCWSTR , sourcecontext : usize , sourceurl : ::windows_sys::core::PCWSTR , result : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsRunSerializedScript ( script : ::windows_sys::core::PCWSTR , buffer : *const u8 , sourcecontext : usize , sourceurl : ::windows_sys::core::PCWSTR , result : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsSerializeScript ( script : ::windows_sys::core::PCWSTR , buffer : *mut u8 , buffersize : *mut u32 ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsSetCurrentContext ( context : *const ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsSetException ( exception : *const ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsSetExternalData ( object : *const ::core::ffi::c_void , externaldata : *const ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsSetIndexedProperty ( object : *const ::core::ffi::c_void , index : *const ::core::ffi::c_void , value : *const ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsSetProperty ( object : *const ::core::ffi::c_void , propertyid : *const ::core::ffi::c_void , value : *const ::core::ffi::c_void , usestrictrules : u8 ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsSetPrototype ( object : *const ::core::ffi::c_void , prototypeobject : *const ::core::ffi::c_void ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsSetRuntimeBeforeCollectCallback ( runtime : *const ::core::ffi::c_void , callbackstate : *const ::core::ffi::c_void , beforecollectcallback : JsBeforeCollectCallback ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsSetRuntimeMemoryAllocationCallback ( runtime : *const ::core::ffi::c_void , callbackstate : *const ::core::ffi::c_void , allocationcallback : JsMemoryAllocationCallback ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsSetRuntimeMemoryLimit ( runtime : *const ::core::ffi::c_void , memorylimit : usize ) -> JsErrorCode );
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`, `\"Win32_System_Diagnostics_Debug\"`*"] fn JsStartDebugging ( debugapplication : super::Diagnostics::Debug:: IDebugApplication64 ) -> JsErrorCode );
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`, `\"Win32_System_Diagnostics_Debug\"`*"] fn JsStartDebugging ( debugapplication : super::Diagnostics::Debug:: IDebugApplication32 ) -> JsErrorCode );
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`, `\"Win32_System_Diagnostics_Debug\"`*"] fn JsStartProfiling ( callback : super::Diagnostics::Debug:: IActiveScriptProfilerCallback , eventmask : super::Diagnostics::Debug:: PROFILER_EVENT_MASK , context : u32 ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsStopProfiling ( reason : ::windows_sys::core::HRESULT ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsStrictEquals ( object1 : *const ::core::ffi::c_void , object2 : *const ::core::ffi::c_void , result : *mut bool ) -> JsErrorCode );
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`*"] fn JsStringToPointer ( value : *const ::core::ffi::c_void , stringvalue : *mut *mut u16 , stringlength : *mut usize ) -> JsErrorCode );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"] fn JsValueToVariant ( object : *const ::core::ffi::c_void , variant : *mut super::Com:: VARIANT ) -> JsErrorCode );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
::windows_sys::core::link ! ( "chakra.dll""system" #[doc = "*Required features: `\"Win32_System_Js\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"] fn JsVariantToValue ( variant : *const super::Com:: VARIANT , value : *mut *mut ::core::ffi::c_void ) -> JsErrorCode );
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JS_SOURCE_CONTEXT_NONE: u64 = 18446744073709551615u64;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsErrorCode = u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsNoError: JsErrorCode = 0u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorCategoryUsage: JsErrorCode = 65536u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorInvalidArgument: JsErrorCode = 65537u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorNullArgument: JsErrorCode = 65538u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorNoCurrentContext: JsErrorCode = 65539u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorInExceptionState: JsErrorCode = 65540u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorNotImplemented: JsErrorCode = 65541u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorWrongThread: JsErrorCode = 65542u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorRuntimeInUse: JsErrorCode = 65543u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorBadSerializedScript: JsErrorCode = 65544u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorInDisabledState: JsErrorCode = 65545u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorCannotDisableExecution: JsErrorCode = 65546u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorHeapEnumInProgress: JsErrorCode = 65547u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorArgumentNotObject: JsErrorCode = 65548u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorInProfileCallback: JsErrorCode = 65549u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorInThreadServiceCallback: JsErrorCode = 65550u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorCannotSerializeDebugScript: JsErrorCode = 65551u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorAlreadyDebuggingContext: JsErrorCode = 65552u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorAlreadyProfilingContext: JsErrorCode = 65553u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorIdleNotEnabled: JsErrorCode = 65554u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorCategoryEngine: JsErrorCode = 131072u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorOutOfMemory: JsErrorCode = 131073u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorCategoryScript: JsErrorCode = 196608u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorScriptException: JsErrorCode = 196609u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorScriptCompile: JsErrorCode = 196610u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorScriptTerminated: JsErrorCode = 196611u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorScriptEvalDisabled: JsErrorCode = 196612u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorCategoryFatal: JsErrorCode = 262144u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsErrorFatal: JsErrorCode = 262145u32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsMemoryEventType = i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsMemoryAllocate: JsMemoryEventType = 0i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsMemoryFree: JsMemoryEventType = 1i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsMemoryFailure: JsMemoryEventType = 2i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsRuntimeAttributes = i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeAttributeNone: JsRuntimeAttributes = 0i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeAttributeDisableBackgroundWork: JsRuntimeAttributes = 1i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeAttributeAllowScriptInterrupt: JsRuntimeAttributes = 2i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeAttributeEnableIdleProcessing: JsRuntimeAttributes = 4i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeAttributeDisableNativeCodeGeneration: JsRuntimeAttributes = 8i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeAttributeDisableEval: JsRuntimeAttributes = 16i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsRuntimeVersion = i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeVersion10: JsRuntimeVersion = 0i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeVersion11: JsRuntimeVersion = 1i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsRuntimeVersionEdge: JsRuntimeVersion = -1i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsValueType = i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsUndefined: JsValueType = 0i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsNull: JsValueType = 1i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsNumber: JsValueType = 2i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsString: JsValueType = 3i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsBoolean: JsValueType = 4i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsObject: JsValueType = 5i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsFunction: JsValueType = 6i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsError: JsValueType = 7i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub const JsArray: JsValueType = 8i32;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsBackgroundWorkItemCallback = ::core::option::Option<unsafe extern "system" fn(callbackstate: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsBeforeCollectCallback = ::core::option::Option<unsafe extern "system" fn(callbackstate: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsFinalizeCallback = ::core::option::Option<unsafe extern "system" fn(data: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsMemoryAllocationCallback = ::core::option::Option<unsafe extern "system" fn(callbackstate: *const ::core::ffi::c_void, allocationevent: JsMemoryEventType, allocationsize: usize) -> bool>;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsNativeFunction = ::core::option::Option<unsafe extern "system" fn(callee: *const ::core::ffi::c_void, isconstructcall: bool, arguments: *const *const ::core::ffi::c_void, argumentcount: u16, callbackstate: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_System_Js\"`*"]
pub type JsThreadServiceCallback = ::core::option::Option<unsafe extern "system" fn(callback: JsBackgroundWorkItemCallback, callbackstate: *const ::core::ffi::c_void) -> bool>;
