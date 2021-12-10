#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub type HCN_NOTIFICATIONS = i32;
pub const HcnNotificationInvalid: HCN_NOTIFICATIONS = 0i32;
pub const HcnNotificationNetworkPreCreate: HCN_NOTIFICATIONS = 1i32;
pub const HcnNotificationNetworkCreate: HCN_NOTIFICATIONS = 2i32;
pub const HcnNotificationNetworkPreDelete: HCN_NOTIFICATIONS = 3i32;
pub const HcnNotificationNetworkDelete: HCN_NOTIFICATIONS = 4i32;
pub const HcnNotificationNamespaceCreate: HCN_NOTIFICATIONS = 5i32;
pub const HcnNotificationNamespaceDelete: HCN_NOTIFICATIONS = 6i32;
pub const HcnNotificationGuestNetworkServiceCreate: HCN_NOTIFICATIONS = 7i32;
pub const HcnNotificationGuestNetworkServiceDelete: HCN_NOTIFICATIONS = 8i32;
pub const HcnNotificationNetworkEndpointAttached: HCN_NOTIFICATIONS = 9i32;
pub const HcnNotificationNetworkEndpointDetached: HCN_NOTIFICATIONS = 16i32;
pub const HcnNotificationGuestNetworkServiceStateChanged: HCN_NOTIFICATIONS = 17i32;
pub const HcnNotificationGuestNetworkServiceInterfaceStateChanged: HCN_NOTIFICATIONS = 18i32;
pub const HcnNotificationServiceDisconnect: HCN_NOTIFICATIONS = 16777216i32;
pub const HcnNotificationFlagsReserved: HCN_NOTIFICATIONS = -268435456i32;
#[cfg(feature = "Win32_Foundation")]
pub type HCN_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(notificationtype: u32, context: *const ::core::ffi::c_void, notificationstatus: ::windows::core::HRESULT, notificationdata: super::super::Foundation::PWSTR)>;
pub type HCN_PORT_ACCESS = i32;
pub const HCN_PORT_ACCESS_EXCLUSIVE: HCN_PORT_ACCESS = 1i32;
pub const HCN_PORT_ACCESS_SHARED: HCN_PORT_ACCESS = 2i32;
pub type HCN_PORT_PROTOCOL = i32;
pub const HCN_PORT_PROTOCOL_TCP: HCN_PORT_PROTOCOL = 1i32;
pub const HCN_PORT_PROTOCOL_UDP: HCN_PORT_PROTOCOL = 2i32;
pub const HCN_PORT_PROTOCOL_BOTH: HCN_PORT_PROTOCOL = 3i32;
#[repr(C)]
pub struct HCN_PORT_RANGE_ENTRY {
    pub OwningPartitionId: ::windows::core::GUID,
    pub TargetPartitionId: ::windows::core::GUID,
    pub Protocol: HCN_PORT_PROTOCOL,
    pub Priority: u64,
    pub ReservationType: u32,
    pub SharingFlags: u32,
    pub DeliveryMode: u32,
    pub StartingPort: u16,
    pub EndingPort: u16,
}
impl ::core::marker::Copy for HCN_PORT_RANGE_ENTRY {}
impl ::core::clone::Clone for HCN_PORT_RANGE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HCN_PORT_RANGE_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HCN_PORT_RANGE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HCN_PORT_RANGE_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for HCN_PORT_RANGE_ENTRY {}
impl ::core::default::Default for HCN_PORT_RANGE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct HCN_PORT_RANGE_RESERVATION {
    pub startingPort: u16,
    pub endingPort: u16,
}
impl ::core::marker::Copy for HCN_PORT_RANGE_RESERVATION {}
impl ::core::clone::Clone for HCN_PORT_RANGE_RESERVATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HCN_PORT_RANGE_RESERVATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HCN_PORT_RANGE_RESERVATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HCN_PORT_RANGE_RESERVATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for HCN_PORT_RANGE_RESERVATION {}
impl ::core::default::Default for HCN_PORT_RANGE_RESERVATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn HcnCloseEndpoint(endpoint: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnCloseEndpoint(endpoint: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        HcnCloseEndpoint(::core::mem::transmute(endpoint)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HcnCloseGuestNetworkService(guestnetworkservice: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnCloseGuestNetworkService(guestnetworkservice: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        HcnCloseGuestNetworkService(::core::mem::transmute(guestnetworkservice)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HcnCloseLoadBalancer(loadbalancer: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnCloseLoadBalancer(loadbalancer: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        HcnCloseLoadBalancer(::core::mem::transmute(loadbalancer)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HcnCloseNamespace(namespace: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnCloseNamespace(namespace: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        HcnCloseNamespace(::core::mem::transmute(namespace)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HcnCloseNetwork(network: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnCloseNetwork(network: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        HcnCloseNetwork(::core::mem::transmute(network)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnCreateEndpoint<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(network: *const ::core::ffi::c_void, id: *const ::windows::core::GUID, settings: Param2, endpoint: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnCreateEndpoint(network: *const ::core::ffi::c_void, id: *const ::windows::core::GUID, settings: super::super::Foundation::PWSTR, endpoint: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcnCreateEndpoint(::core::mem::transmute(network), ::core::mem::transmute(id), settings.into_param().abi(), ::core::mem::transmute(endpoint), ::core::mem::transmute(errorrecord)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnCreateGuestNetworkService<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(id: *const ::windows::core::GUID, settings: Param1, guestnetworkservice: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnCreateGuestNetworkService(id: *const ::windows::core::GUID, settings: super::super::Foundation::PWSTR, guestnetworkservice: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcnCreateGuestNetworkService(::core::mem::transmute(id), settings.into_param().abi(), ::core::mem::transmute(guestnetworkservice), ::core::mem::transmute(errorrecord)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnCreateLoadBalancer<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(id: *const ::windows::core::GUID, settings: Param1, loadbalancer: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnCreateLoadBalancer(id: *const ::windows::core::GUID, settings: super::super::Foundation::PWSTR, loadbalancer: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcnCreateLoadBalancer(::core::mem::transmute(id), settings.into_param().abi(), ::core::mem::transmute(loadbalancer), ::core::mem::transmute(errorrecord)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnCreateNamespace<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(id: *const ::windows::core::GUID, settings: Param1, namespace: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnCreateNamespace(id: *const ::windows::core::GUID, settings: super::super::Foundation::PWSTR, namespace: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcnCreateNamespace(::core::mem::transmute(id), settings.into_param().abi(), ::core::mem::transmute(namespace), ::core::mem::transmute(errorrecord)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnCreateNetwork<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(id: *const ::windows::core::GUID, settings: Param1, network: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnCreateNetwork(id: *const ::windows::core::GUID, settings: super::super::Foundation::PWSTR, network: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcnCreateNetwork(::core::mem::transmute(id), settings.into_param().abi(), ::core::mem::transmute(network), ::core::mem::transmute(errorrecord)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnDeleteEndpoint(id: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnDeleteEndpoint(id: *const ::windows::core::GUID, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        HcnDeleteEndpoint(::core::mem::transmute(id), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnDeleteGuestNetworkService(id: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnDeleteGuestNetworkService(id: *const ::windows::core::GUID, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        HcnDeleteGuestNetworkService(::core::mem::transmute(id), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnDeleteLoadBalancer(id: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnDeleteLoadBalancer(id: *const ::windows::core::GUID, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        HcnDeleteLoadBalancer(::core::mem::transmute(id), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnDeleteNamespace(id: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnDeleteNamespace(id: *const ::windows::core::GUID, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        HcnDeleteNamespace(::core::mem::transmute(id), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnDeleteNetwork(id: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnDeleteNetwork(id: *const ::windows::core::GUID, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        HcnDeleteNetwork(::core::mem::transmute(id), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnEnumerateEndpoints<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(query: Param0, endpoints: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnEnumerateEndpoints(query: super::super::Foundation::PWSTR, endpoints: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcnEnumerateEndpoints(query.into_param().abi(), ::core::mem::transmute(endpoints), ::core::mem::transmute(errorrecord)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HcnEnumerateGuestNetworkPortReservations(returncount: *mut u32, portentries: *mut *mut HCN_PORT_RANGE_ENTRY) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnEnumerateGuestNetworkPortReservations(returncount: *mut u32, portentries: *mut *mut HCN_PORT_RANGE_ENTRY) -> ::windows::core::HRESULT;
        }
        HcnEnumerateGuestNetworkPortReservations(::core::mem::transmute(returncount), ::core::mem::transmute(portentries)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnEnumerateLoadBalancers<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(query: Param0, loadbalancer: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnEnumerateLoadBalancers(query: super::super::Foundation::PWSTR, loadbalancer: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcnEnumerateLoadBalancers(query.into_param().abi(), ::core::mem::transmute(loadbalancer), ::core::mem::transmute(errorrecord)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnEnumerateNamespaces<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(query: Param0, namespaces: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnEnumerateNamespaces(query: super::super::Foundation::PWSTR, namespaces: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcnEnumerateNamespaces(query.into_param().abi(), ::core::mem::transmute(namespaces), ::core::mem::transmute(errorrecord)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnEnumerateNetworks<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(query: Param0, networks: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnEnumerateNetworks(query: super::super::Foundation::PWSTR, networks: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcnEnumerateNetworks(query.into_param().abi(), ::core::mem::transmute(networks), ::core::mem::transmute(errorrecord)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HcnFreeGuestNetworkPortReservations(portentries: *mut HCN_PORT_RANGE_ENTRY) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnFreeGuestNetworkPortReservations(portentries: *mut HCN_PORT_RANGE_ENTRY);
        }
        HcnFreeGuestNetworkPortReservations(::core::mem::transmute(portentries))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnModifyEndpoint<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(endpoint: *const ::core::ffi::c_void, settings: Param1) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnModifyEndpoint(endpoint: *const ::core::ffi::c_void, settings: super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        HcnModifyEndpoint(::core::mem::transmute(endpoint), settings.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnModifyGuestNetworkService<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(guestnetworkservice: *const ::core::ffi::c_void, settings: Param1) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnModifyGuestNetworkService(guestnetworkservice: *const ::core::ffi::c_void, settings: super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        HcnModifyGuestNetworkService(::core::mem::transmute(guestnetworkservice), settings.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnModifyLoadBalancer<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(loadbalancer: *const ::core::ffi::c_void, settings: Param1) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnModifyLoadBalancer(loadbalancer: *const ::core::ffi::c_void, settings: super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        HcnModifyLoadBalancer(::core::mem::transmute(loadbalancer), settings.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnModifyNamespace<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(namespace: *const ::core::ffi::c_void, settings: Param1) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnModifyNamespace(namespace: *const ::core::ffi::c_void, settings: super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        HcnModifyNamespace(::core::mem::transmute(namespace), settings.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnModifyNetwork<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(network: *const ::core::ffi::c_void, settings: Param1) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnModifyNetwork(network: *const ::core::ffi::c_void, settings: super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        HcnModifyNetwork(::core::mem::transmute(network), settings.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnOpenEndpoint(id: *const ::windows::core::GUID, endpoint: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnOpenEndpoint(id: *const ::windows::core::GUID, endpoint: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcnOpenEndpoint(::core::mem::transmute(id), ::core::mem::transmute(endpoint), ::core::mem::transmute(errorrecord)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnOpenLoadBalancer(id: *const ::windows::core::GUID, loadbalancer: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnOpenLoadBalancer(id: *const ::windows::core::GUID, loadbalancer: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcnOpenLoadBalancer(::core::mem::transmute(id), ::core::mem::transmute(loadbalancer), ::core::mem::transmute(errorrecord)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnOpenNamespace(id: *const ::windows::core::GUID, namespace: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnOpenNamespace(id: *const ::windows::core::GUID, namespace: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcnOpenNamespace(::core::mem::transmute(id), ::core::mem::transmute(namespace), ::core::mem::transmute(errorrecord)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnOpenNetwork(id: *const ::windows::core::GUID, network: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnOpenNetwork(id: *const ::windows::core::GUID, network: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcnOpenNetwork(::core::mem::transmute(id), ::core::mem::transmute(network), ::core::mem::transmute(errorrecord)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnQueryEndpointProperties<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(endpoint: *const ::core::ffi::c_void, query: Param1, properties: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnQueryEndpointProperties(endpoint: *const ::core::ffi::c_void, query: super::super::Foundation::PWSTR, properties: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcnQueryEndpointProperties(::core::mem::transmute(endpoint), query.into_param().abi(), ::core::mem::transmute(properties), ::core::mem::transmute(errorrecord)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnQueryLoadBalancerProperties<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(loadbalancer: *const ::core::ffi::c_void, query: Param1, properties: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnQueryLoadBalancerProperties(loadbalancer: *const ::core::ffi::c_void, query: super::super::Foundation::PWSTR, properties: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcnQueryLoadBalancerProperties(::core::mem::transmute(loadbalancer), query.into_param().abi(), ::core::mem::transmute(properties), ::core::mem::transmute(errorrecord)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnQueryNamespaceProperties<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(namespace: *const ::core::ffi::c_void, query: Param1, properties: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnQueryNamespaceProperties(namespace: *const ::core::ffi::c_void, query: super::super::Foundation::PWSTR, properties: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcnQueryNamespaceProperties(::core::mem::transmute(namespace), query.into_param().abi(), ::core::mem::transmute(properties), ::core::mem::transmute(errorrecord)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnQueryNetworkProperties<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(network: *const ::core::ffi::c_void, query: Param1, properties: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnQueryNetworkProperties(network: *const ::core::ffi::c_void, query: super::super::Foundation::PWSTR, properties: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcnQueryNetworkProperties(::core::mem::transmute(network), query.into_param().abi(), ::core::mem::transmute(properties), ::core::mem::transmute(errorrecord)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnRegisterGuestNetworkServiceCallback(guestnetworkservice: *const ::core::ffi::c_void, callback: HCN_NOTIFICATION_CALLBACK, context: *const ::core::ffi::c_void, callbackhandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnRegisterGuestNetworkServiceCallback(guestnetworkservice: *const ::core::ffi::c_void, callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, callbackhandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        HcnRegisterGuestNetworkServiceCallback(::core::mem::transmute(guestnetworkservice), ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(callbackhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnRegisterServiceCallback(callback: HCN_NOTIFICATION_CALLBACK, context: *const ::core::ffi::c_void, callbackhandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnRegisterServiceCallback(callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, callbackhandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        HcnRegisterServiceCallback(::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(callbackhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnReleaseGuestNetworkServicePortReservationHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(portreservationhandle: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnReleaseGuestNetworkServicePortReservationHandle(portreservationhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        HcnReleaseGuestNetworkServicePortReservationHandle(portreservationhandle.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnReserveGuestNetworkServicePort(guestnetworkservice: *const ::core::ffi::c_void, protocol: HCN_PORT_PROTOCOL, access: HCN_PORT_ACCESS, port: u16) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnReserveGuestNetworkServicePort(guestnetworkservice: *const ::core::ffi::c_void, protocol: HCN_PORT_PROTOCOL, access: HCN_PORT_ACCESS, port: u16, portreservationhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::HANDLE = ::core::mem::zeroed();
        HcnReserveGuestNetworkServicePort(::core::mem::transmute(guestnetworkservice), ::core::mem::transmute(protocol), ::core::mem::transmute(access), ::core::mem::transmute(port), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnReserveGuestNetworkServicePortRange(guestnetworkservice: *const ::core::ffi::c_void, portcount: u16, portrangereservation: *mut HCN_PORT_RANGE_RESERVATION, portreservationhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnReserveGuestNetworkServicePortRange(guestnetworkservice: *const ::core::ffi::c_void, portcount: u16, portrangereservation: *mut HCN_PORT_RANGE_RESERVATION, portreservationhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        HcnReserveGuestNetworkServicePortRange(::core::mem::transmute(guestnetworkservice), ::core::mem::transmute(portcount), ::core::mem::transmute(portrangereservation), ::core::mem::transmute(portreservationhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HcnUnregisterGuestNetworkServiceCallback(callbackhandle: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnUnregisterGuestNetworkServiceCallback(callbackhandle: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        HcnUnregisterGuestNetworkServiceCallback(::core::mem::transmute(callbackhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HcnUnregisterServiceCallback(callbackhandle: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcnUnregisterServiceCallback(callbackhandle: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        HcnUnregisterServiceCallback(::core::mem::transmute(callbackhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}