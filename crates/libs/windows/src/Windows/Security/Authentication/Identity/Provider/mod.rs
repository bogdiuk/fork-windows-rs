#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorAuthentication(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISecondaryAuthenticationFactorAuthentication {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x020a16e5_6a25_40a3_8c00_50a023f619d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthenticationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicehmac: ::windows::core::RawPtr, sessionhmac: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorlogmessage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorAuthenticationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISecondaryAuthenticationFactorAuthenticationResult {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cbb5987_ef6d_4bc2_bf49_4617515a0f9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthenticationResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SecondaryAuthenticationFactorAuthenticationStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4a5ee56_7291_4073_bc1f_ccb8f5afdf96);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorAuthenticationStageInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISecondaryAuthenticationFactorAuthenticationStageInfo {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationStageInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56fec28b_e8aa_4c0f_8e4c_a559e73add88);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthenticationStageInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SecondaryAuthenticationFactorAuthenticationStage) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SecondaryAuthenticationFactorAuthenticationScenario) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorAuthenticationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISecondaryAuthenticationFactorAuthenticationStatics {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f582656_28f8_4e0f_ae8c_5898b9ae2469);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthenticationStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: SecondaryAuthenticationFactorAuthenticationMessage, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, serviceauthenticationnonce: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics {
    type Vtable = ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90499a19_7ef2_4523_951c_a417a24acf93);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, deviceinstancepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, monitoringmode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, deviceinstancepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, monitoringmode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode, devicefriendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, devicemodelnumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, deviceconfigurationdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISecondaryAuthenticationFactorInfo {
    type Vtable = ISecondaryAuthenticationFactorInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e2ba861_8533_4fce_839b_ecb72410ac14);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISecondaryAuthenticationFactorInfo2 {
    type Vtable = ISecondaryAuthenticationFactorInfo2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14d981a3_fc26_4ff7_abc3_48e82a512a0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorInfo2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SecondaryAuthenticationFactorDevicePresenceMonitoringMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presencestate: SecondaryAuthenticationFactorDevicePresence, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorRegistration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISecondaryAuthenticationFactorRegistration {
    type Vtable = ISecondaryAuthenticationFactorRegistrationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f4cbbb4_8cba_48b0_840d_dbb22a54c678);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorRegistrationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceconfigurationdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorlogmessage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorRegistrationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISecondaryAuthenticationFactorRegistrationResult {
    type Vtable = ISecondaryAuthenticationFactorRegistrationResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4fe35f0_ade3_4981_af6b_ec195921682a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorRegistrationResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SecondaryAuthenticationFactorRegistrationStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorRegistrationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISecondaryAuthenticationFactorRegistrationStatics {
    type Vtable = ISecondaryAuthenticationFactorRegistrationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1adf0f65_e3b7_4155_997f_b756ef65beba);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorRegistrationStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, capabilities: SecondaryAuthenticationFactorDeviceCapabilities, devicefriendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, devicemodelnumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, devicekey: ::windows::core::RawPtr, mutualauthenticationkey: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querytype: SecondaryAuthenticationFactorDeviceFindScope, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, deviceconfigurationdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthentication(::windows::core::IUnknown);
impl SecondaryAuthenticationFactorAuthentication {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage_Streams")]
    pub fn ServiceAuthenticationHmac(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage_Streams")]
    pub fn SessionNonce(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage_Streams")]
    pub fn DeviceNonce(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage_Streams")]
    pub fn DeviceConfigurationData(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn FinishAuthenticationAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Storage::Streams::IBuffer>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Storage::Streams::IBuffer>>(&self, devicehmac: Param0, sessionhmac: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorFinishAuthenticationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), devicehmac.into_param().abi(), sessionhmac.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorFinishAuthenticationStatus>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn AbortAuthenticationAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, errorlogmessage: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), errorlogmessage.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn ShowNotificationMessageAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(devicename: Param0, message: SecondaryAuthenticationFactorAuthenticationMessage) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::ISecondaryAuthenticationFactorAuthenticationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), devicename.into_param().abi(), message, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn StartAuthenticationAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Storage::Streams::IBuffer>>(deviceid: Param0, serviceauthenticationnonce: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationResult>> {
        Self::ISecondaryAuthenticationFactorAuthenticationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), serviceauthenticationnonce.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationResult>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticationStageChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventHandler<SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        Self::ISecondaryAuthenticationFactorAuthenticationStatics(|this| unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAuthenticationStageChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::ISecondaryAuthenticationFactorAuthenticationStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn GetAuthenticationStageInfoAsync() -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationStageInfo>> {
        Self::ISecondaryAuthenticationFactorAuthenticationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationStageInfo>>(result__)
        })
    }
    pub fn ISecondaryAuthenticationFactorAuthenticationStatics<R, F: FnOnce(&ISecondaryAuthenticationFactorAuthenticationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SecondaryAuthenticationFactorAuthentication, ISecondaryAuthenticationFactorAuthenticationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SecondaryAuthenticationFactorAuthentication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorAuthentication {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecondaryAuthenticationFactorAuthentication {}
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorAuthentication {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthentication;{020a16e5-6a25-40a3-8c00-50a023f619d1})");
}
unsafe impl ::windows::core::Interface for SecondaryAuthenticationFactorAuthentication {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x020a16e5_6a25_40a3_8c00_50a023f619d1);
}
impl ::windows::core::RuntimeName for SecondaryAuthenticationFactorAuthentication {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthentication";
}
impl ::core::convert::From<SecondaryAuthenticationFactorAuthentication> for ::windows::core::IUnknown {
    fn from(value: SecondaryAuthenticationFactorAuthentication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthentication> for ::windows::core::IUnknown {
    fn from(value: &SecondaryAuthenticationFactorAuthentication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SecondaryAuthenticationFactorAuthentication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SecondaryAuthenticationFactorAuthentication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SecondaryAuthenticationFactorAuthentication> for ::windows::core::IInspectable {
    fn from(value: SecondaryAuthenticationFactorAuthentication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthentication> for ::windows::core::IInspectable {
    fn from(value: &SecondaryAuthenticationFactorAuthentication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SecondaryAuthenticationFactorAuthentication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SecondaryAuthenticationFactorAuthentication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SecondaryAuthenticationFactorAuthentication {}
unsafe impl ::core::marker::Sync for SecondaryAuthenticationFactorAuthentication {}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationMessage(pub i32);
impl SecondaryAuthenticationFactorAuthenticationMessage {
    pub const Invalid: Self = Self(0i32);
    pub const SwipeUpWelcome: Self = Self(1i32);
    pub const TapWelcome: Self = Self(2i32);
    pub const DeviceNeedsAttention: Self = Self(3i32);
    pub const LookingForDevice: Self = Self(4i32);
    pub const LookingForDevicePluggedin: Self = Self(5i32);
    pub const BluetoothIsDisabled: Self = Self(6i32);
    pub const NfcIsDisabled: Self = Self(7i32);
    pub const WiFiIsDisabled: Self = Self(8i32);
    pub const ExtraTapIsRequired: Self = Self(9i32);
    pub const DisabledByPolicy: Self = Self(10i32);
    pub const TapOnDeviceRequired: Self = Self(11i32);
    pub const HoldFinger: Self = Self(12i32);
    pub const ScanFinger: Self = Self(13i32);
    pub const UnauthorizedUser: Self = Self(14i32);
    pub const ReregisterRequired: Self = Self(15i32);
    pub const TryAgain: Self = Self(16i32);
    pub const SayPassphrase: Self = Self(17i32);
    pub const ReadyToSignIn: Self = Self(18i32);
    pub const UseAnotherSignInOption: Self = Self(19i32);
    pub const ConnectionRequired: Self = Self(20i32);
    pub const TimeLimitExceeded: Self = Self(21i32);
    pub const CanceledByUser: Self = Self(22i32);
    pub const CenterHand: Self = Self(23i32);
    pub const MoveHandCloser: Self = Self(24i32);
    pub const MoveHandFarther: Self = Self(25i32);
    pub const PlaceHandAbove: Self = Self(26i32);
    pub const RecognitionFailed: Self = Self(27i32);
    pub const DeviceUnavailable: Self = Self(28i32);
}
impl ::core::marker::Copy for SecondaryAuthenticationFactorAuthenticationMessage {}
impl ::core::clone::Clone for SecondaryAuthenticationFactorAuthenticationMessage {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecondaryAuthenticationFactorAuthenticationMessage {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorAuthenticationMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecondaryAuthenticationFactorAuthenticationMessage {}
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorAuthenticationMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationMessage;i4)");
}
impl ::windows::core::DefaultType for SecondaryAuthenticationFactorAuthenticationMessage {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationResult(::windows::core::IUnknown);
impl SecondaryAuthenticationFactorAuthenticationResult {
    #[cfg(feature = "deprecated")]
    pub fn Status(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__: SecondaryAuthenticationFactorAuthenticationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryAuthenticationFactorAuthenticationStatus>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Authentication(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthentication> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryAuthenticationFactorAuthentication>(result__)
        }
    }
}
impl ::core::clone::Clone for SecondaryAuthenticationFactorAuthenticationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorAuthenticationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecondaryAuthenticationFactorAuthenticationResult {}
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorAuthenticationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationResult;{9cbb5987-ef6d-4bc2-bf49-4617515a0f9a})");
}
unsafe impl ::windows::core::Interface for SecondaryAuthenticationFactorAuthenticationResult {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cbb5987_ef6d_4bc2_bf49_4617515a0f9a);
}
impl ::windows::core::RuntimeName for SecondaryAuthenticationFactorAuthenticationResult {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationResult";
}
impl ::core::convert::From<SecondaryAuthenticationFactorAuthenticationResult> for ::windows::core::IUnknown {
    fn from(value: SecondaryAuthenticationFactorAuthenticationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthenticationResult> for ::windows::core::IUnknown {
    fn from(value: &SecondaryAuthenticationFactorAuthenticationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SecondaryAuthenticationFactorAuthenticationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SecondaryAuthenticationFactorAuthenticationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SecondaryAuthenticationFactorAuthenticationResult> for ::windows::core::IInspectable {
    fn from(value: SecondaryAuthenticationFactorAuthenticationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthenticationResult> for ::windows::core::IInspectable {
    fn from(value: &SecondaryAuthenticationFactorAuthenticationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SecondaryAuthenticationFactorAuthenticationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SecondaryAuthenticationFactorAuthenticationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SecondaryAuthenticationFactorAuthenticationResult {}
unsafe impl ::core::marker::Sync for SecondaryAuthenticationFactorAuthenticationResult {}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationScenario(pub i32);
impl SecondaryAuthenticationFactorAuthenticationScenario {
    pub const SignIn: Self = Self(0i32);
    pub const CredentialPrompt: Self = Self(1i32);
}
impl ::core::marker::Copy for SecondaryAuthenticationFactorAuthenticationScenario {}
impl ::core::clone::Clone for SecondaryAuthenticationFactorAuthenticationScenario {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecondaryAuthenticationFactorAuthenticationScenario {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorAuthenticationScenario {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecondaryAuthenticationFactorAuthenticationScenario {}
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorAuthenticationScenario {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationScenario;i4)");
}
impl ::windows::core::DefaultType for SecondaryAuthenticationFactorAuthenticationScenario {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationStage(pub i32);
impl SecondaryAuthenticationFactorAuthenticationStage {
    pub const NotStarted: Self = Self(0i32);
    pub const WaitingForUserConfirmation: Self = Self(1i32);
    pub const CollectingCredential: Self = Self(2i32);
    pub const SuspendingAuthentication: Self = Self(3i32);
    pub const CredentialCollected: Self = Self(4i32);
    pub const CredentialAuthenticated: Self = Self(5i32);
    pub const StoppingAuthentication: Self = Self(6i32);
    pub const ReadyForLock: Self = Self(7i32);
    pub const CheckingDevicePresence: Self = Self(8i32);
}
impl ::core::marker::Copy for SecondaryAuthenticationFactorAuthenticationStage {}
impl ::core::clone::Clone for SecondaryAuthenticationFactorAuthenticationStage {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecondaryAuthenticationFactorAuthenticationStage {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorAuthenticationStage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecondaryAuthenticationFactorAuthenticationStage {}
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorAuthenticationStage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStage;i4)");
}
impl ::windows::core::DefaultType for SecondaryAuthenticationFactorAuthenticationStage {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs(::windows::core::IUnknown);
impl SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn StageInfo(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthenticationStageInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryAuthenticationFactorAuthenticationStageInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {}
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs;{d4a5ee56-7291-4073-bc1f-ccb8f5afdf96})");
}
unsafe impl ::windows::core::Interface for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4a5ee56_7291_4073_bc1f_ccb8f5afdf96);
}
impl ::windows::core::RuntimeName for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs";
}
impl ::core::convert::From<SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {}
unsafe impl ::core::marker::Sync for SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationStageInfo(::windows::core::IUnknown);
impl SecondaryAuthenticationFactorAuthenticationStageInfo {
    #[cfg(feature = "deprecated")]
    pub fn Stage(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthenticationStage> {
        let this = self;
        unsafe {
            let mut result__: SecondaryAuthenticationFactorAuthenticationStage = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryAuthenticationFactorAuthenticationStage>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Scenario(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthenticationScenario> {
        let this = self;
        unsafe {
            let mut result__: SecondaryAuthenticationFactorAuthenticationScenario = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryAuthenticationFactorAuthenticationScenario>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for SecondaryAuthenticationFactorAuthenticationStageInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorAuthenticationStageInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecondaryAuthenticationFactorAuthenticationStageInfo {}
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorAuthenticationStageInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageInfo;{56fec28b-e8aa-4c0f-8e4c-a559e73add88})");
}
unsafe impl ::windows::core::Interface for SecondaryAuthenticationFactorAuthenticationStageInfo {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationStageInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56fec28b_e8aa_4c0f_8e4c_a559e73add88);
}
impl ::windows::core::RuntimeName for SecondaryAuthenticationFactorAuthenticationStageInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageInfo";
}
impl ::core::convert::From<SecondaryAuthenticationFactorAuthenticationStageInfo> for ::windows::core::IUnknown {
    fn from(value: SecondaryAuthenticationFactorAuthenticationStageInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthenticationStageInfo> for ::windows::core::IUnknown {
    fn from(value: &SecondaryAuthenticationFactorAuthenticationStageInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SecondaryAuthenticationFactorAuthenticationStageInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SecondaryAuthenticationFactorAuthenticationStageInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SecondaryAuthenticationFactorAuthenticationStageInfo> for ::windows::core::IInspectable {
    fn from(value: SecondaryAuthenticationFactorAuthenticationStageInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorAuthenticationStageInfo> for ::windows::core::IInspectable {
    fn from(value: &SecondaryAuthenticationFactorAuthenticationStageInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SecondaryAuthenticationFactorAuthenticationStageInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SecondaryAuthenticationFactorAuthenticationStageInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SecondaryAuthenticationFactorAuthenticationStageInfo {}
unsafe impl ::core::marker::Sync for SecondaryAuthenticationFactorAuthenticationStageInfo {}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationStatus(pub i32);
impl SecondaryAuthenticationFactorAuthenticationStatus {
    pub const Failed: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const UnknownDevice: Self = Self(2i32);
    pub const DisabledByPolicy: Self = Self(3i32);
    pub const InvalidAuthenticationStage: Self = Self(4i32);
}
impl ::core::marker::Copy for SecondaryAuthenticationFactorAuthenticationStatus {}
impl ::core::clone::Clone for SecondaryAuthenticationFactorAuthenticationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecondaryAuthenticationFactorAuthenticationStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorAuthenticationStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecondaryAuthenticationFactorAuthenticationStatus {}
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorAuthenticationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStatus;i4)");
}
impl ::windows::core::DefaultType for SecondaryAuthenticationFactorAuthenticationStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDeviceCapabilities(pub u32);
impl SecondaryAuthenticationFactorDeviceCapabilities {
    pub const None: Self = Self(0u32);
    pub const SecureStorage: Self = Self(1u32);
    pub const StoreKeys: Self = Self(2u32);
    pub const ConfirmUserIntentToAuthenticate: Self = Self(4u32);
    pub const SupportSecureUserPresenceCheck: Self = Self(8u32);
    pub const TransmittedDataIsEncrypted: Self = Self(16u32);
    pub const HMacSha256: Self = Self(32u32);
    pub const CloseRangeDataTransmission: Self = Self(64u32);
}
impl ::core::marker::Copy for SecondaryAuthenticationFactorDeviceCapabilities {}
impl ::core::clone::Clone for SecondaryAuthenticationFactorDeviceCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecondaryAuthenticationFactorDeviceCapabilities {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorDeviceCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecondaryAuthenticationFactorDeviceCapabilities {}
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorDeviceCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDeviceCapabilities;u4)");
}
impl ::windows::core::DefaultType for SecondaryAuthenticationFactorDeviceCapabilities {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDeviceFindScope(pub i32);
impl SecondaryAuthenticationFactorDeviceFindScope {
    pub const User: Self = Self(0i32);
    pub const AllUsers: Self = Self(1i32);
}
impl ::core::marker::Copy for SecondaryAuthenticationFactorDeviceFindScope {}
impl ::core::clone::Clone for SecondaryAuthenticationFactorDeviceFindScope {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecondaryAuthenticationFactorDeviceFindScope {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorDeviceFindScope {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecondaryAuthenticationFactorDeviceFindScope {}
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorDeviceFindScope {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDeviceFindScope;i4)");
}
impl ::windows::core::DefaultType for SecondaryAuthenticationFactorDeviceFindScope {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDevicePresence(pub i32);
impl SecondaryAuthenticationFactorDevicePresence {
    pub const Absent: Self = Self(0i32);
    pub const Present: Self = Self(1i32);
}
impl ::core::marker::Copy for SecondaryAuthenticationFactorDevicePresence {}
impl ::core::clone::Clone for SecondaryAuthenticationFactorDevicePresence {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecondaryAuthenticationFactorDevicePresence {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorDevicePresence {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecondaryAuthenticationFactorDevicePresence {}
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorDevicePresence {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDevicePresence;i4)");
}
impl ::windows::core::DefaultType for SecondaryAuthenticationFactorDevicePresence {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDevicePresenceMonitoringMode(pub i32);
impl SecondaryAuthenticationFactorDevicePresenceMonitoringMode {
    pub const Unsupported: Self = Self(0i32);
    pub const AppManaged: Self = Self(1i32);
    pub const SystemManaged: Self = Self(2i32);
}
impl ::core::marker::Copy for SecondaryAuthenticationFactorDevicePresenceMonitoringMode {}
impl ::core::clone::Clone for SecondaryAuthenticationFactorDevicePresenceMonitoringMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecondaryAuthenticationFactorDevicePresenceMonitoringMode {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorDevicePresenceMonitoringMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecondaryAuthenticationFactorDevicePresenceMonitoringMode {}
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorDevicePresenceMonitoringMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDevicePresenceMonitoringMode;i4)");
}
impl ::windows::core::DefaultType for SecondaryAuthenticationFactorDevicePresenceMonitoringMode {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus(pub i32);
impl SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {
    pub const Unsupported: Self = Self(0i32);
    pub const Succeeded: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
}
impl ::core::marker::Copy for SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {}
impl ::core::clone::Clone for SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {}
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus;i4)");
}
impl ::windows::core::DefaultType for SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorFinishAuthenticationStatus(pub i32);
impl SecondaryAuthenticationFactorFinishAuthenticationStatus {
    pub const Failed: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const NonceExpired: Self = Self(2i32);
}
impl ::core::marker::Copy for SecondaryAuthenticationFactorFinishAuthenticationStatus {}
impl ::core::clone::Clone for SecondaryAuthenticationFactorFinishAuthenticationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecondaryAuthenticationFactorFinishAuthenticationStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorFinishAuthenticationStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecondaryAuthenticationFactorFinishAuthenticationStatus {}
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorFinishAuthenticationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorFinishAuthenticationStatus;i4)");
}
impl ::windows::core::DefaultType for SecondaryAuthenticationFactorFinishAuthenticationStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorInfo(::windows::core::IUnknown);
impl SecondaryAuthenticationFactorInfo {
    #[cfg(feature = "deprecated")]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn DeviceFriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn DeviceModelNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage_Streams")]
    pub fn DeviceConfigurationData(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn PresenceMonitoringMode(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorDevicePresenceMonitoringMode> {
        let this = &::windows::core::Interface::cast::<ISecondaryAuthenticationFactorInfo2>(self)?;
        unsafe {
            let mut result__: SecondaryAuthenticationFactorDevicePresenceMonitoringMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryAuthenticationFactorDevicePresenceMonitoringMode>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn UpdateDevicePresenceAsync(&self, presencestate: SecondaryAuthenticationFactorDevicePresence) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<ISecondaryAuthenticationFactorInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), presencestate, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn IsAuthenticationSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISecondaryAuthenticationFactorInfo2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for SecondaryAuthenticationFactorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecondaryAuthenticationFactorInfo {}
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorInfo;{1e2ba861-8533-4fce-839b-ecb72410ac14})");
}
unsafe impl ::windows::core::Interface for SecondaryAuthenticationFactorInfo {
    type Vtable = ISecondaryAuthenticationFactorInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e2ba861_8533_4fce_839b_ecb72410ac14);
}
impl ::windows::core::RuntimeName for SecondaryAuthenticationFactorInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorInfo";
}
impl ::core::convert::From<SecondaryAuthenticationFactorInfo> for ::windows::core::IUnknown {
    fn from(value: SecondaryAuthenticationFactorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorInfo> for ::windows::core::IUnknown {
    fn from(value: &SecondaryAuthenticationFactorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SecondaryAuthenticationFactorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SecondaryAuthenticationFactorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SecondaryAuthenticationFactorInfo> for ::windows::core::IInspectable {
    fn from(value: SecondaryAuthenticationFactorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorInfo> for ::windows::core::IInspectable {
    fn from(value: &SecondaryAuthenticationFactorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SecondaryAuthenticationFactorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SecondaryAuthenticationFactorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SecondaryAuthenticationFactorInfo {}
unsafe impl ::core::marker::Sync for SecondaryAuthenticationFactorInfo {}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorRegistration(::windows::core::IUnknown);
impl SecondaryAuthenticationFactorRegistration {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RegisterDevicePresenceMonitoringAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0, deviceinstancepath: Param1, monitoringmode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>> {
        Self::ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), deviceinstancepath.into_param().abi(), monitoringmode, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn RegisterDevicePresenceMonitoringWithNewDeviceAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param5: ::windows::core::IntoParam<'a, super::super::super::super::Storage::Streams::IBuffer>>(deviceid: Param0, deviceinstancepath: Param1, monitoringmode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode, devicefriendlyname: Param3, devicemodelnumber: Param4, deviceconfigurationdata: Param5) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>> {
        Self::ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), deviceinstancepath.into_param().abi(), monitoringmode, devicefriendlyname.into_param().abi(), devicemodelnumber.into_param().abi(), deviceconfigurationdata.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn UnregisterDevicePresenceMonitoringAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn IsDevicePresenceMonitoringSupported() -> ::windows::core::Result<bool> {
        Self::ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn FinishRegisteringDeviceAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Storage::Streams::IBuffer>>(&self, deviceconfigurationdata: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceconfigurationdata.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn AbortRegisteringDeviceAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, errorlogmessage: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), errorlogmessage.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn RequestStartRegisteringDeviceAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, super::super::super::super::Storage::Streams::IBuffer>, Param5: ::windows::core::IntoParam<'a, super::super::super::super::Storage::Streams::IBuffer>>(deviceid: Param0, capabilities: SecondaryAuthenticationFactorDeviceCapabilities, devicefriendlyname: Param2, devicemodelnumber: Param3, devicekey: Param4, mutualauthenticationkey: Param5) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorRegistrationResult>> {
        Self::ISecondaryAuthenticationFactorRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), capabilities, devicefriendlyname.into_param().abi(), devicemodelnumber.into_param().abi(), devicekey.into_param().abi(), mutualauthenticationkey.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorRegistrationResult>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindAllRegisteredDeviceInfoAsync(querytype: SecondaryAuthenticationFactorDeviceFindScope) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<SecondaryAuthenticationFactorInfo>>> {
        Self::ISecondaryAuthenticationFactorRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), querytype, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<SecondaryAuthenticationFactorInfo>>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn UnregisterDeviceAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::ISecondaryAuthenticationFactorRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn UpdateDeviceConfigurationDataAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Storage::Streams::IBuffer>>(deviceid: Param0, deviceconfigurationdata: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::ISecondaryAuthenticationFactorRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), deviceconfigurationdata.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics<R, F: FnOnce(&ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SecondaryAuthenticationFactorRegistration, ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISecondaryAuthenticationFactorRegistrationStatics<R, F: FnOnce(&ISecondaryAuthenticationFactorRegistrationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SecondaryAuthenticationFactorRegistration, ISecondaryAuthenticationFactorRegistrationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SecondaryAuthenticationFactorRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecondaryAuthenticationFactorRegistration {}
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorRegistration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistration;{9f4cbbb4-8cba-48b0-840d-dbb22a54c678})");
}
unsafe impl ::windows::core::Interface for SecondaryAuthenticationFactorRegistration {
    type Vtable = ISecondaryAuthenticationFactorRegistrationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f4cbbb4_8cba_48b0_840d_dbb22a54c678);
}
impl ::windows::core::RuntimeName for SecondaryAuthenticationFactorRegistration {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistration";
}
impl ::core::convert::From<SecondaryAuthenticationFactorRegistration> for ::windows::core::IUnknown {
    fn from(value: SecondaryAuthenticationFactorRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorRegistration> for ::windows::core::IUnknown {
    fn from(value: &SecondaryAuthenticationFactorRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SecondaryAuthenticationFactorRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SecondaryAuthenticationFactorRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SecondaryAuthenticationFactorRegistration> for ::windows::core::IInspectable {
    fn from(value: SecondaryAuthenticationFactorRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorRegistration> for ::windows::core::IInspectable {
    fn from(value: &SecondaryAuthenticationFactorRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SecondaryAuthenticationFactorRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SecondaryAuthenticationFactorRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SecondaryAuthenticationFactorRegistration {}
unsafe impl ::core::marker::Sync for SecondaryAuthenticationFactorRegistration {}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorRegistrationResult(::windows::core::IUnknown);
impl SecondaryAuthenticationFactorRegistrationResult {
    #[cfg(feature = "deprecated")]
    pub fn Status(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorRegistrationStatus> {
        let this = self;
        unsafe {
            let mut result__: SecondaryAuthenticationFactorRegistrationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryAuthenticationFactorRegistrationStatus>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Registration(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorRegistration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryAuthenticationFactorRegistration>(result__)
        }
    }
}
impl ::core::clone::Clone for SecondaryAuthenticationFactorRegistrationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorRegistrationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecondaryAuthenticationFactorRegistrationResult {}
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorRegistrationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistrationResult;{a4fe35f0-ade3-4981-af6b-ec195921682a})");
}
unsafe impl ::windows::core::Interface for SecondaryAuthenticationFactorRegistrationResult {
    type Vtable = ISecondaryAuthenticationFactorRegistrationResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4fe35f0_ade3_4981_af6b_ec195921682a);
}
impl ::windows::core::RuntimeName for SecondaryAuthenticationFactorRegistrationResult {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistrationResult";
}
impl ::core::convert::From<SecondaryAuthenticationFactorRegistrationResult> for ::windows::core::IUnknown {
    fn from(value: SecondaryAuthenticationFactorRegistrationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorRegistrationResult> for ::windows::core::IUnknown {
    fn from(value: &SecondaryAuthenticationFactorRegistrationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SecondaryAuthenticationFactorRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SecondaryAuthenticationFactorRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SecondaryAuthenticationFactorRegistrationResult> for ::windows::core::IInspectable {
    fn from(value: SecondaryAuthenticationFactorRegistrationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SecondaryAuthenticationFactorRegistrationResult> for ::windows::core::IInspectable {
    fn from(value: &SecondaryAuthenticationFactorRegistrationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SecondaryAuthenticationFactorRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SecondaryAuthenticationFactorRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SecondaryAuthenticationFactorRegistrationResult {}
unsafe impl ::core::marker::Sync for SecondaryAuthenticationFactorRegistrationResult {}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorRegistrationStatus(pub i32);
impl SecondaryAuthenticationFactorRegistrationStatus {
    pub const Failed: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const CanceledByUser: Self = Self(2i32);
    pub const PinSetupRequired: Self = Self(3i32);
    pub const DisabledByPolicy: Self = Self(4i32);
}
impl ::core::marker::Copy for SecondaryAuthenticationFactorRegistrationStatus {}
impl ::core::clone::Clone for SecondaryAuthenticationFactorRegistrationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecondaryAuthenticationFactorRegistrationStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorRegistrationStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecondaryAuthenticationFactorRegistrationStatus {}
unsafe impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorRegistrationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistrationStatus;i4)");
}
impl ::windows::core::DefaultType for SecondaryAuthenticationFactorRegistrationStatus {
    type DefaultType = Self;
}