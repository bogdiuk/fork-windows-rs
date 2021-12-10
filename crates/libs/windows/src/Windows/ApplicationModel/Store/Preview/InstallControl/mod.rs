#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(transparent)]
pub struct AppInstallItem(::windows::core::IUnknown);
impl AppInstallItem {
    pub fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn InstallType(&self) -> ::windows::core::Result<AppInstallType> {
        let this = self;
        unsafe {
            let mut result__: AppInstallType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallType>(result__)
        }
    }
    pub fn IsUserInitiated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn GetCurrentStatus(&self) -> ::windows::core::Result<AppInstallStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallStatus>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Pause(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Restart(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Completed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<AppInstallItem, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<AppInstallItem, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn CancelWithTelemetry<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, correlationvector: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), correlationvector.into_param().abi()).ok() }
    }
    pub fn PauseWithTelemetry<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, correlationvector: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), correlationvector.into_param().abi()).ok() }
    }
    pub fn RestartWithTelemetry<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, correlationvector: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), correlationvector.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>(result__)
        }
    }
    pub fn ItemOperationsMightAffectOtherItems(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn LaunchAfterInstall(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetLaunchAfterInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PinToDesktopAfterInstall(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetPinToDesktopAfterInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PinToStartAfterInstall(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetPinToStartAfterInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PinToTaskbarAfterInstall(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetPinToTaskbarAfterInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CompletedInstallToastNotificationMode(&self) -> ::windows::core::Result<AppInstallationToastNotificationMode> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__: AppInstallationToastNotificationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallationToastNotificationMode>(result__)
        }
    }
    pub fn SetCompletedInstallToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn InstallInProgressToastNotificationMode(&self) -> ::windows::core::Result<AppInstallationToastNotificationMode> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__: AppInstallationToastNotificationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallationToastNotificationMode>(result__)
        }
    }
    pub fn SetInstallInProgressToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for AppInstallItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstallItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallItem {}
unsafe impl ::windows::core::RuntimeType for AppInstallItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem;{49d3dfab-168a-4cbf-a93a-9e448c82737d})");
}
unsafe impl ::windows::core::Interface for AppInstallItem {
    type Vtable = IAppInstallItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49d3dfab_168a_4cbf_a93a_9e448c82737d);
}
impl ::windows::core::RuntimeName for AppInstallItem {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem";
}
impl ::core::convert::From<AppInstallItem> for ::windows::core::IUnknown {
    fn from(value: AppInstallItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallItem> for ::windows::core::IUnknown {
    fn from(value: &AppInstallItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppInstallItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppInstallItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppInstallItem> for ::windows::core::IInspectable {
    fn from(value: AppInstallItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallItem> for ::windows::core::IInspectable {
    fn from(value: &AppInstallItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppInstallItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppInstallItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppInstallItem {}
unsafe impl ::core::marker::Sync for AppInstallItem {}
#[repr(transparent)]
pub struct AppInstallManager(::windows::core::IUnknown);
impl AppInstallManager {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppInstallManager, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppInstallItems(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>(result__)
        }
    }
    pub fn Cancel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), productid.into_param().abi()).ok() }
    }
    pub fn Pause<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), productid.into_param().abi()).ok() }
    }
    pub fn Restart<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), productid.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ItemCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<AppInstallManager, AppInstallManagerItemEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ItemStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<AppInstallManager, AppInstallManagerItemEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn AutoUpdateSetting(&self) -> ::windows::core::Result<AutoUpdateSetting> {
        let this = self;
        unsafe {
            let mut result__: AutoUpdateSetting = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutoUpdateSetting>(result__)
        }
    }
    pub fn SetAutoUpdateSetting(&self, value: AutoUpdateSetting) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AcquisitionIdentity(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAcquisitionIdentity<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetIsApplicableAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0, skuid: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), productid.into_param().abi(), skuid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartAppInstallAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0, skuid: Param1, repair: bool, forceuseofnonremovablestorage: bool) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), productid.into_param().abi(), skuid.into_param().abi(), repair, forceuseofnonremovablestorage, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UpdateAppByPackageFamilyNameAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagefamilyname: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SearchForUpdatesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0, skuid: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), productid.into_param().abi(), skuid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SearchForAllUpdatesAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn IsStoreBlockedByPolicyAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, storeclientname: Param0, storeclientpublisher: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), storeclientname.into_param().abi(), storeclientpublisher.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetIsAppAllowedToInstallAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), productid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartAppInstallWithTelemetryAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param5: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param6: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0, skuid: Param1, repair: bool, forceuseofnonremovablestorage: bool, catalogid: Param4, bundleid: Param5, correlationvector: Param6) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), productid.into_param().abi(), skuid.into_param().abi(), repair, forceuseofnonremovablestorage, catalogid.into_param().abi(), bundleid.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UpdateAppByPackageFamilyNameWithTelemetryAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, packagefamilyname: Param0, correlationvector: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SearchForUpdatesWithTelemetryAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0, skuid: Param1, catalogid: Param2, correlationvector: Param3) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), productid.into_param().abi(), skuid.into_param().abi(), catalogid.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SearchForAllUpdatesWithTelemetryAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, correlationvector: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetIsAppAllowedToInstallWithTelemetryAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0, skuid: Param1, catalogid: Param2, correlationvector: Param3) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), productid.into_param().abi(), skuid.into_param().abi(), catalogid.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn CancelWithTelemetry<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0, correlationvector: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), productid.into_param().abi(), correlationvector.into_param().abi()).ok() }
    }
    pub fn PauseWithTelemetry<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0, correlationvector: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), productid.into_param().abi(), correlationvector.into_param().abi()).ok() }
    }
    pub fn RestartWithTelemetry<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0, correlationvector: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), productid.into_param().abi(), correlationvector.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Management_Deployment"))]
    pub fn StartProductInstallAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param6: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param7: ::windows::core::IntoParam<'a, super::super::super::super::Management::Deployment::PackageVolume>>(
        &self,
        productid: Param0,
        catalogid: Param1,
        flightid: Param2,
        clientid: Param3,
        repair: bool,
        forceuseofnonremovablestorage: bool,
        correlationvector: Param6,
        targetvolume: Param7,
    ) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), productid.into_param().abi(), catalogid.into_param().abi(), flightid.into_param().abi(), clientid.into_param().abi(), repair, forceuseofnonremovablestorage, correlationvector.into_param().abi(), targetvolume.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System"))]
    pub fn StartProductInstallForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param7: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param8: ::windows::core::IntoParam<'a, super::super::super::super::Management::Deployment::PackageVolume>>(
        &self,
        user: Param0,
        productid: Param1,
        catalogid: Param2,
        flightid: Param3,
        clientid: Param4,
        repair: bool,
        forceuseofnonremovablestorage: bool,
        correlationvector: Param7,
        targetvolume: Param8,
    ) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), user.into_param().abi(), productid.into_param().abi(), catalogid.into_param().abi(), flightid.into_param().abi(), clientid.into_param().abi(), repair, forceuseofnonremovablestorage, correlationvector.into_param().abi(), targetvolume.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn UpdateAppByPackageFamilyNameForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, user: Param0, packagefamilyname: Param1, correlationvector: Param2) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), user.into_param().abi(), packagefamilyname.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn SearchForUpdatesForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, user: Param0, productid: Param1, skuid: Param2, catalogid: Param3, correlationvector: Param4) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), user.into_param().abi(), productid.into_param().abi(), skuid.into_param().abi(), catalogid.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System"))]
    pub fn SearchForAllUpdatesForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, user: Param0, correlationvector: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), user.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetIsAppAllowedToInstallForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, user: Param0, productid: Param1, skuid: Param2, catalogid: Param3, correlationvector: Param4) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), user.into_param().abi(), productid.into_param().abi(), skuid.into_param().abi(), catalogid.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetIsApplicableForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, user: Param0, productid: Param1, skuid: Param2) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), user.into_param().abi(), productid.into_param().abi(), skuid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn MoveToFrontOfDownloadQueue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, productid: Param0, correlationvector: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), productid.into_param().abi(), correlationvector.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetFreeUserEntitlementAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, storeid: Param0, campaignid: Param1, correlationvector: Param2) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), storeid.into_param().abi(), campaignid.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetFreeUserEntitlementForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, user: Param0, storeid: Param1, campaignid: Param2, correlationvector: Param3) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), user.into_param().abi(), storeid.into_param().abi(), campaignid.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetFreeDeviceEntitlementAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, storeid: Param0, campaignid: Param1, correlationvector: Param2) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), storeid.into_param().abi(), campaignid.into_param().abi(), correlationvector.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppInstallItemsWithGroupSupport(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SearchForAllUpdatesWithUpdateOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, AppUpdateOptions>>(&self, correlationvector: Param0, clientid: Param1, updateoptions: Param2) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), correlationvector.into_param().abi(), clientid.into_param().abi(), updateoptions.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System"))]
    pub fn SearchForAllUpdatesWithUpdateOptionsForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, AppUpdateOptions>>(&self, user: Param0, correlationvector: Param1, clientid: Param2, updateoptions: Param3) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), user.into_param().abi(), correlationvector.into_param().abi(), clientid.into_param().abi(), updateoptions.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SearchForUpdatesWithUpdateOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, AppUpdateOptions>>(&self, productid: Param0, skuid: Param1, correlationvector: Param2, clientid: Param3, updateoptions: Param4) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), productid.into_param().abi(), skuid.into_param().abi(), correlationvector.into_param().abi(), clientid.into_param().abi(), updateoptions.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn SearchForUpdatesWithUpdateOptionsForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param5: ::windows::core::IntoParam<'a, AppUpdateOptions>>(&self, user: Param0, productid: Param1, skuid: Param2, correlationvector: Param3, clientid: Param4, updateoptions: Param5) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), user.into_param().abi(), productid.into_param().abi(), skuid.into_param().abi(), correlationvector.into_param().abi(), clientid.into_param().abi(), updateoptions.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn StartProductInstallWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, AppInstallOptions>>(&self, productid: Param0, flightid: Param1, clientid: Param2, correlationvector: Param3, installoptions: Param4) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), productid.into_param().abi(), flightid.into_param().abi(), clientid.into_param().abi(), correlationvector.into_param().abi(), installoptions.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System"))]
    pub fn StartProductInstallWithOptionsForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param5: ::windows::core::IntoParam<'a, AppInstallOptions>>(&self, user: Param0, productid: Param1, flightid: Param2, clientid: Param3, correlationvector: Param4, installoptions: Param5) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), user.into_param().abi(), productid.into_param().abi(), flightid.into_param().abi(), clientid.into_param().abi(), correlationvector.into_param().abi(), installoptions.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetIsPackageIdentityAllowedToInstallAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, correlationvector: Param0, packageidentityname: Param1, publishercertificatename: Param2) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), correlationvector.into_param().abi(), packageidentityname.into_param().abi(), publishercertificatename.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetIsPackageIdentityAllowedToInstallForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, user: Param0, correlationvector: Param1, packageidentityname: Param2, publishercertificatename: Param3) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), user.into_param().abi(), correlationvector.into_param().abi(), packageidentityname.into_param().abi(), publishercertificatename.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn CanInstallForAllUsers(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager7>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AppInstallManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstallManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallManager {}
unsafe impl ::windows::core::RuntimeType for AppInstallManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager;{9353e170-8441-4b45-bd72-7c2fa925beee})");
}
unsafe impl ::windows::core::Interface for AppInstallManager {
    type Vtable = IAppInstallManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9353e170_8441_4b45_bd72_7c2fa925beee);
}
impl ::windows::core::RuntimeName for AppInstallManager {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager";
}
impl ::core::convert::From<AppInstallManager> for ::windows::core::IUnknown {
    fn from(value: AppInstallManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallManager> for ::windows::core::IUnknown {
    fn from(value: &AppInstallManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppInstallManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppInstallManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppInstallManager> for ::windows::core::IInspectable {
    fn from(value: AppInstallManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallManager> for ::windows::core::IInspectable {
    fn from(value: &AppInstallManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppInstallManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppInstallManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppInstallManager {}
unsafe impl ::core::marker::Sync for AppInstallManager {}
#[repr(transparent)]
pub struct AppInstallManagerItemEventArgs(::windows::core::IUnknown);
impl AppInstallManagerItemEventArgs {
    pub fn Item(&self) -> ::windows::core::Result<AppInstallItem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallItem>(result__)
        }
    }
}
impl ::core::clone::Clone for AppInstallManagerItemEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstallManagerItemEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallManagerItemEventArgs {}
unsafe impl ::windows::core::RuntimeType for AppInstallManagerItemEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManagerItemEventArgs;{bc505743-4674-4dd1-957e-c25682086a14})");
}
unsafe impl ::windows::core::Interface for AppInstallManagerItemEventArgs {
    type Vtable = IAppInstallManagerItemEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc505743_4674_4dd1_957e_c25682086a14);
}
impl ::windows::core::RuntimeName for AppInstallManagerItemEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManagerItemEventArgs";
}
impl ::core::convert::From<AppInstallManagerItemEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppInstallManagerItemEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallManagerItemEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppInstallManagerItemEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppInstallManagerItemEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppInstallManagerItemEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppInstallManagerItemEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppInstallManagerItemEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallManagerItemEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppInstallManagerItemEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppInstallManagerItemEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppInstallManagerItemEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppInstallManagerItemEventArgs {}
unsafe impl ::core::marker::Sync for AppInstallManagerItemEventArgs {}
#[repr(transparent)]
pub struct AppInstallOptions(::windows::core::IUnknown);
impl AppInstallOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppInstallOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CatalogId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCatalogId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ForceUseOfNonRemovableStorage(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceUseOfNonRemovableStorage(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AllowForcedAppRestart(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowForcedAppRestart(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Repair(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetRepair(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Management_Deployment")]
    pub fn TargetVolume(&self) -> ::windows::core::Result<super::super::super::super::Management::Deployment::PackageVolume> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Management::Deployment::PackageVolume>(result__)
        }
    }
    #[cfg(feature = "Management_Deployment")]
    pub fn SetTargetVolume<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Management::Deployment::PackageVolume>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn LaunchAfterInstall(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetLaunchAfterInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PinToDesktopAfterInstall(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetPinToDesktopAfterInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PinToStartAfterInstall(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetPinToStartAfterInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PinToTaskbarAfterInstall(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetPinToTaskbarAfterInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CompletedInstallToastNotificationMode(&self) -> ::windows::core::Result<AppInstallationToastNotificationMode> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: AppInstallationToastNotificationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallationToastNotificationMode>(result__)
        }
    }
    pub fn SetCompletedInstallToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn InstallInProgressToastNotificationMode(&self) -> ::windows::core::Result<AppInstallationToastNotificationMode> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: AppInstallationToastNotificationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallationToastNotificationMode>(result__)
        }
    }
    pub fn SetInstallInProgressToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn InstallForAllUsers(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetInstallForAllUsers(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn StageButDoNotInstall(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetStageButDoNotInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CampaignId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCampaignId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ExtendedCampaignId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetExtendedCampaignId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppInstallOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstallOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallOptions {}
unsafe impl ::windows::core::RuntimeType for AppInstallOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallOptions;{c9808300-1cb8-4eb6-8c9f-6a30c64a5b51})");
}
unsafe impl ::windows::core::Interface for AppInstallOptions {
    type Vtable = IAppInstallOptionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9808300_1cb8_4eb6_8c9f_6a30c64a5b51);
}
impl ::windows::core::RuntimeName for AppInstallOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallOptions";
}
impl ::core::convert::From<AppInstallOptions> for ::windows::core::IUnknown {
    fn from(value: AppInstallOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallOptions> for ::windows::core::IUnknown {
    fn from(value: &AppInstallOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppInstallOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppInstallOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppInstallOptions> for ::windows::core::IInspectable {
    fn from(value: AppInstallOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallOptions> for ::windows::core::IInspectable {
    fn from(value: &AppInstallOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppInstallOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppInstallOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppInstallOptions {}
unsafe impl ::core::marker::Sync for AppInstallOptions {}
#[repr(transparent)]
pub struct AppInstallState(pub i32);
impl AppInstallState {
    pub const Pending: Self = Self(0i32);
    pub const Starting: Self = Self(1i32);
    pub const AcquiringLicense: Self = Self(2i32);
    pub const Downloading: Self = Self(3i32);
    pub const RestoringData: Self = Self(4i32);
    pub const Installing: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
    pub const Canceled: Self = Self(7i32);
    pub const Paused: Self = Self(8i32);
    pub const Error: Self = Self(9i32);
    pub const PausedLowBattery: Self = Self(10i32);
    pub const PausedWiFiRecommended: Self = Self(11i32);
    pub const PausedWiFiRequired: Self = Self(12i32);
    pub const ReadyToDownload: Self = Self(13i32);
}
impl ::core::marker::Copy for AppInstallState {}
impl ::core::clone::Clone for AppInstallState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AppInstallState {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AppInstallState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallState {}
unsafe impl ::windows::core::RuntimeType for AppInstallState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallState;i4)");
}
impl ::windows::core::DefaultType for AppInstallState {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct AppInstallStatus(::windows::core::IUnknown);
impl AppInstallStatus {
    pub fn InstallState(&self) -> ::windows::core::Result<AppInstallState> {
        let this = self;
        unsafe {
            let mut result__: AppInstallState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppInstallState>(result__)
        }
    }
    pub fn DownloadSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn BytesDownloaded(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn PercentComplete(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IAppInstallStatus2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::System::User>(result__)
        }
    }
    pub fn ReadyForLaunch(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallStatus2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsStaged(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallStatus3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AppInstallStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstallStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallStatus {}
unsafe impl ::windows::core::RuntimeType for AppInstallStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallStatus;{936dccfa-2450-4126-88b1-6127a644dd5c})");
}
unsafe impl ::windows::core::Interface for AppInstallStatus {
    type Vtable = IAppInstallStatusVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x936dccfa_2450_4126_88b1_6127a644dd5c);
}
impl ::windows::core::RuntimeName for AppInstallStatus {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallStatus";
}
impl ::core::convert::From<AppInstallStatus> for ::windows::core::IUnknown {
    fn from(value: AppInstallStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallStatus> for ::windows::core::IUnknown {
    fn from(value: &AppInstallStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppInstallStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppInstallStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppInstallStatus> for ::windows::core::IInspectable {
    fn from(value: AppInstallStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallStatus> for ::windows::core::IInspectable {
    fn from(value: &AppInstallStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppInstallStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppInstallStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppInstallStatus {}
unsafe impl ::core::marker::Sync for AppInstallStatus {}
#[repr(transparent)]
pub struct AppInstallType(pub i32);
impl AppInstallType {
    pub const Install: Self = Self(0i32);
    pub const Update: Self = Self(1i32);
    pub const Repair: Self = Self(2i32);
}
impl ::core::marker::Copy for AppInstallType {}
impl ::core::clone::Clone for AppInstallType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AppInstallType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AppInstallType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallType {}
unsafe impl ::windows::core::RuntimeType for AppInstallType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallType;i4)");
}
impl ::windows::core::DefaultType for AppInstallType {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct AppInstallationToastNotificationMode(pub i32);
impl AppInstallationToastNotificationMode {
    pub const Default: Self = Self(0i32);
    pub const Toast: Self = Self(1i32);
    pub const ToastWithoutPopup: Self = Self(2i32);
    pub const NoToast: Self = Self(3i32);
}
impl ::core::marker::Copy for AppInstallationToastNotificationMode {}
impl ::core::clone::Clone for AppInstallationToastNotificationMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AppInstallationToastNotificationMode {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AppInstallationToastNotificationMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallationToastNotificationMode {}
unsafe impl ::windows::core::RuntimeType for AppInstallationToastNotificationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallationToastNotificationMode;i4)");
}
impl ::windows::core::DefaultType for AppInstallationToastNotificationMode {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct AppUpdateOptions(::windows::core::IUnknown);
impl AppUpdateOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppUpdateOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CatalogId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCatalogId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn AllowForcedAppRestart(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowForcedAppRestart(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AutomaticallyDownloadAndInstallUpdateIfFound(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppUpdateOptions2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutomaticallyDownloadAndInstallUpdateIfFound(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppUpdateOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for AppUpdateOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppUpdateOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppUpdateOptions {}
unsafe impl ::windows::core::RuntimeType for AppUpdateOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppUpdateOptions;{26f0b02f-c2f3-4aea-af8c-6308dd9db85f})");
}
unsafe impl ::windows::core::Interface for AppUpdateOptions {
    type Vtable = IAppUpdateOptionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26f0b02f_c2f3_4aea_af8c_6308dd9db85f);
}
impl ::windows::core::RuntimeName for AppUpdateOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppUpdateOptions";
}
impl ::core::convert::From<AppUpdateOptions> for ::windows::core::IUnknown {
    fn from(value: AppUpdateOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppUpdateOptions> for ::windows::core::IUnknown {
    fn from(value: &AppUpdateOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppUpdateOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppUpdateOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppUpdateOptions> for ::windows::core::IInspectable {
    fn from(value: AppUpdateOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppUpdateOptions> for ::windows::core::IInspectable {
    fn from(value: &AppUpdateOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppUpdateOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppUpdateOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppUpdateOptions {}
unsafe impl ::core::marker::Sync for AppUpdateOptions {}
#[repr(transparent)]
pub struct AutoUpdateSetting(pub i32);
impl AutoUpdateSetting {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
    pub const EnabledByPolicy: Self = Self(3i32);
}
impl ::core::marker::Copy for AutoUpdateSetting {}
impl ::core::clone::Clone for AutoUpdateSetting {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AutoUpdateSetting {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AutoUpdateSetting {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutoUpdateSetting {}
unsafe impl ::windows::core::RuntimeType for AutoUpdateSetting {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AutoUpdateSetting;i4)");
}
impl ::windows::core::DefaultType for AutoUpdateSetting {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct GetEntitlementResult(::windows::core::IUnknown);
impl GetEntitlementResult {
    pub fn Status(&self) -> ::windows::core::Result<GetEntitlementStatus> {
        let this = self;
        unsafe {
            let mut result__: GetEntitlementStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GetEntitlementStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for GetEntitlementResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GetEntitlementResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GetEntitlementResult {}
unsafe impl ::windows::core::RuntimeType for GetEntitlementResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementResult;{74fc843f-1a9e-4609-8e4d-819086d08a3d})");
}
unsafe impl ::windows::core::Interface for GetEntitlementResult {
    type Vtable = IGetEntitlementResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74fc843f_1a9e_4609_8e4d_819086d08a3d);
}
impl ::windows::core::RuntimeName for GetEntitlementResult {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementResult";
}
impl ::core::convert::From<GetEntitlementResult> for ::windows::core::IUnknown {
    fn from(value: GetEntitlementResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GetEntitlementResult> for ::windows::core::IUnknown {
    fn from(value: &GetEntitlementResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GetEntitlementResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GetEntitlementResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GetEntitlementResult> for ::windows::core::IInspectable {
    fn from(value: GetEntitlementResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GetEntitlementResult> for ::windows::core::IInspectable {
    fn from(value: &GetEntitlementResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GetEntitlementResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GetEntitlementResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GetEntitlementResult {}
unsafe impl ::core::marker::Sync for GetEntitlementResult {}
#[repr(transparent)]
pub struct GetEntitlementStatus(pub i32);
impl GetEntitlementStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const NoStoreAccount: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
}
impl ::core::marker::Copy for GetEntitlementStatus {}
impl ::core::clone::Clone for GetEntitlementStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GetEntitlementStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GetEntitlementStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GetEntitlementStatus {}
unsafe impl ::windows::core::RuntimeType for GetEntitlementStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementStatus;i4)");
}
impl ::windows::core::DefaultType for GetEntitlementStatus {
    type DefaultType = Self;
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallItem {
    type Vtable = IAppInstallItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49d3dfab_168a_4cbf_a93a_9e448c82737d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItemVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallItem2 {
    type Vtable = IAppInstallItem2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3972af8_40c0_4fd7_aa6c_0aa13ca6188c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallItem3 {
    type Vtable = IAppInstallItem3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f3dc998_dd47_433c_9234_560172d67a45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallItem4 {
    type Vtable = IAppInstallItem4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2d1ce12_71ff_4fc8_b540_453d4b37e1d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallItem5 {
    type Vtable = IAppInstallItem5Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5510e7cc_4076_4a0b_9472_c21d9d380e55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem5Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallManager {
    type Vtable = IAppInstallManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9353e170_8441_4b45_bd72_7c2fa925beee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AutoUpdateSetting) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AutoUpdateSetting) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeclientname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, storeclientpublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallManager2 {
    type Vtable = IAppInstallManager2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16937851_ed37_480d_8314_52e27c03f04a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, bundleid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallManager3 {
    type Vtable = IAppInstallManager3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95b24b17_e96a_4d0e_84e1_c8cb417a0178);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Management_Deployment"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, flightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetvolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Management_Deployment")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, flightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetvolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallManager4 {
    type Vtable = IAppInstallManager4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x260a2a16_5a9e_4ebd_b944_f2ba75c31159);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, campaignid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, storeid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, campaignid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, campaignid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallManager5 {
    type Vtable = IAppInstallManager5Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3cd7be4c_1be9_4f7f_b675_aa1d64a529b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager5Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager6(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallManager6 {
    type Vtable = IAppInstallManager6Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9e7d408_f27a_4471_b2f4_e76efcbebcca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager6Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, updateoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, updateoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, updateoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, updateoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, flightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, installoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, flightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, installoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packageidentityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, publishercertificatename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packageidentityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, publishercertificatename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager7(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallManager7 {
    type Vtable = IAppInstallManager7Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5ee7b30_d5e4_49a3_9853_3db03203321d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager7Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManagerItemEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallManagerItemEventArgs {
    type Vtable = IAppInstallManagerItemEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc505743_4674_4dd1_957e_c25682086a14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManagerItemEventArgsVtbl(
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
pub struct IAppInstallOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallOptions {
    type Vtable = IAppInstallOptionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9808300_1cb8_4eb6_8c9f_6a30c64a5b51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallOptionsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Management_Deployment")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Management_Deployment"))] usize,
    #[cfg(feature = "Management_Deployment")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Management_Deployment"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallOptions2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallOptions2 {
    type Vtable = IAppInstallOptions2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a04c0d7_c94b_425e_95b4_bf27faeaee89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallOptions2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallStatus(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallStatus {
    type Vtable = IAppInstallStatusVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x936dccfa_2450_4126_88b1_6127a644dd5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallStatusVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallStatus2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallStatus2 {
    type Vtable = IAppInstallStatus2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96e7818a_5e92_4aa9_8edc_58fed4b87e00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallStatus2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallStatus3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstallStatus3 {
    type Vtable = IAppInstallStatus3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb880c56_837b_4b4c_9ebb_6d44a0a96307);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallStatus3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUpdateOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppUpdateOptions {
    type Vtable = IAppUpdateOptionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26f0b02f_c2f3_4aea_af8c_6308dd9db85f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUpdateOptionsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUpdateOptions2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppUpdateOptions2 {
    type Vtable = IAppUpdateOptions2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4646e08_ed26_4bf9_9679_48f628e53df8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUpdateOptions2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGetEntitlementResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGetEntitlementResult {
    type Vtable = IGetEntitlementResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74fc843f_1a9e_4609_8e4d_819086d08a3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetEntitlementResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GetEntitlementStatus) -> ::windows::core::HRESULT,
);