#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    pub moduleName: super::super::Foundation::PWSTR,
    pub friendlyModuleName: super::super::Foundation::PWSTR,
    pub eventCount: u32,
    pub uploadSizeBytes: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAGNOSTIC_DATA_EVENT_BINARY_STATS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    pub id: i32,
    pub name: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    pub name: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    pub privacyTag: i32,
    pub name: super::super::Foundation::PWSTR,
    pub description: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    pub privacyTag: i32,
    pub eventCount: u32,
}
impl ::core::marker::Copy for DIAGNOSTIC_DATA_EVENT_TAG_STATS {}
impl ::core::clone::Clone for DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAGNOSTIC_DATA_EVENT_TAG_STATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_EVENT_TAG_STATS {}
impl ::core::default::Default for DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    pub hoursOfHistoryToKeep: u32,
    pub maxStoreMegabytes: u32,
    pub requestedMaxStoreMegabytes: u32,
}
impl ::core::marker::Copy for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {}
impl ::core::clone::Clone for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {}
impl ::core::default::Default for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIAGNOSTIC_DATA_GENERAL_STATS {
    pub optInLevel: u32,
    pub transcriptSizeBytes: u64,
    pub oldestEventTimestamp: u64,
    pub totalEventCountLast24Hours: u32,
    pub averageDailyEvents: f32,
}
impl ::core::marker::Copy for DIAGNOSTIC_DATA_GENERAL_STATS {}
impl ::core::clone::Clone for DIAGNOSTIC_DATA_GENERAL_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DIAGNOSTIC_DATA_GENERAL_STATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_GENERAL_STATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAGNOSTIC_DATA_GENERAL_STATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_GENERAL_STATS {}
impl ::core::default::Default for DIAGNOSTIC_DATA_GENERAL_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DIAGNOSTIC_DATA_RECORD {
    pub rowId: i64,
    pub timestamp: u64,
    pub eventKeywords: u64,
    pub fullEventName: super::super::Foundation::PWSTR,
    pub providerGroupGuid: super::super::Foundation::PWSTR,
    pub producerName: super::super::Foundation::PWSTR,
    pub privacyTags: *mut i32,
    pub privacyTagCount: u32,
    pub categoryIds: *mut i32,
    pub categoryIdCount: u32,
    pub isCoreData: super::super::Foundation::BOOL,
    pub extra1: super::super::Foundation::PWSTR,
    pub extra2: super::super::Foundation::PWSTR,
    pub extra3: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIAGNOSTIC_DATA_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIAGNOSTIC_DATA_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DIAGNOSTIC_DATA_RECORD {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAGNOSTIC_DATA_RECORD>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIAGNOSTIC_DATA_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    pub producerNames: *mut super::super::Foundation::PWSTR,
    pub producerNameCount: u32,
    pub textToMatch: super::super::Foundation::PWSTR,
    pub categoryIds: *mut i32,
    pub categoryIdCount: u32,
    pub privacyTags: *mut i32,
    pub privacyTagCount: u32,
    pub coreDataOnly: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIAGNOSTIC_DATA_SEARCH_CRITERIA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAGNOSTIC_DATA_SEARCH_CRITERIA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_SEARCH_CRITERIA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DIAGNOSTIC_REPORT_DATA {
    pub signature: DIAGNOSTIC_REPORT_SIGNATURE,
    pub bucketId: ::windows::core::GUID,
    pub reportId: ::windows::core::GUID,
    pub creationTime: super::super::Foundation::FILETIME,
    pub sizeInBytes: u64,
    pub cabId: super::super::Foundation::PWSTR,
    pub reportStatus: u32,
    pub reportIntegratorId: ::windows::core::GUID,
    pub fileNames: *mut super::super::Foundation::PWSTR,
    pub fileCount: u32,
    pub friendlyEventName: super::super::Foundation::PWSTR,
    pub applicationName: super::super::Foundation::PWSTR,
    pub applicationPath: super::super::Foundation::PWSTR,
    pub description: super::super::Foundation::PWSTR,
    pub bucketIdString: super::super::Foundation::PWSTR,
    pub legacyBucketId: u64,
    pub reportKey: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIAGNOSTIC_REPORT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIAGNOSTIC_REPORT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DIAGNOSTIC_REPORT_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIAGNOSTIC_REPORT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAGNOSTIC_REPORT_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIAGNOSTIC_REPORT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIAGNOSTIC_REPORT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIAGNOSTIC_REPORT_PARAMETER {
    pub name: [u16; 129],
    pub value: [u16; 260],
}
impl ::core::marker::Copy for DIAGNOSTIC_REPORT_PARAMETER {}
impl ::core::clone::Clone for DIAGNOSTIC_REPORT_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DIAGNOSTIC_REPORT_PARAMETER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DIAGNOSTIC_REPORT_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAGNOSTIC_REPORT_PARAMETER>()) == 0 }
    }
}
impl ::core::cmp::Eq for DIAGNOSTIC_REPORT_PARAMETER {}
impl ::core::default::Default for DIAGNOSTIC_REPORT_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIAGNOSTIC_REPORT_SIGNATURE {
    pub eventName: [u16; 65],
    pub parameters: [DIAGNOSTIC_REPORT_PARAMETER; 10],
}
impl ::core::marker::Copy for DIAGNOSTIC_REPORT_SIGNATURE {}
impl ::core::clone::Clone for DIAGNOSTIC_REPORT_SIGNATURE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DIAGNOSTIC_REPORT_SIGNATURE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DIAGNOSTIC_REPORT_SIGNATURE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAGNOSTIC_REPORT_SIGNATURE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DIAGNOSTIC_REPORT_SIGNATURE {}
impl ::core::default::Default for DIAGNOSTIC_REPORT_SIGNATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DdqAccessLevel = i32;
pub const NoData: DdqAccessLevel = 0i32;
pub const CurrentUserData: DdqAccessLevel = 1i32;
pub const AllUserData: DdqAccessLevel = 2i32;
#[inline]
pub unsafe fn DdqCancelDiagnosticRecordOperation<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqCancelDiagnosticRecordOperation(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION) -> ::windows::core::HRESULT;
        }
        DdqCancelDiagnosticRecordOperation(hsession.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdqCloseSession<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqCloseSession(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION) -> ::windows::core::HRESULT;
        }
        DdqCloseSession(hsession.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdqCreateSession(accesslevel: DdqAccessLevel) -> ::windows::core::Result<super::HDIAGNOSTIC_DATA_QUERY_SESSION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqCreateSession(accesslevel: DdqAccessLevel, hsession: *mut super::HDIAGNOSTIC_DATA_QUERY_SESSION) -> ::windows::core::HRESULT;
        }
        let mut result__: super::HDIAGNOSTIC_DATA_QUERY_SESSION = ::core::mem::zeroed();
        DdqCreateSession(::core::mem::transmute(accesslevel), ::core::mem::transmute(&mut result__)).from_abi::<super::HDIAGNOSTIC_DATA_QUERY_SESSION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqExtractDiagnosticReport<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hsession: Param0, reportstoretype: u32, reportkey: Param2, destinationpath: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqExtractDiagnosticReport(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype: u32, reportkey: super::super::Foundation::PWSTR, destinationpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        DdqExtractDiagnosticReport(hsession.into_param().abi(), ::core::mem::transmute(reportstoretype), reportkey.into_param().abi(), destinationpath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdqFreeDiagnosticRecordLocaleTags<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>>(htagdescription: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqFreeDiagnosticRecordLocaleTags(htagdescription: super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION) -> ::windows::core::HRESULT;
        }
        DdqFreeDiagnosticRecordLocaleTags(htagdescription.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdqFreeDiagnosticRecordPage<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_RECORD>>(hrecord: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqFreeDiagnosticRecordPage(hrecord: super::HDIAGNOSTIC_RECORD) -> ::windows::core::HRESULT;
        }
        DdqFreeDiagnosticRecordPage(hrecord.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdqFreeDiagnosticRecordProducerCategories<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>>(hcategorydescription: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqFreeDiagnosticRecordProducerCategories(hcategorydescription: super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION) -> ::windows::core::HRESULT;
        }
        DdqFreeDiagnosticRecordProducerCategories(hcategorydescription.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdqFreeDiagnosticRecordProducers<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>>(hproducerdescription: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqFreeDiagnosticRecordProducers(hproducerdescription: super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION) -> ::windows::core::HRESULT;
        }
        DdqFreeDiagnosticRecordProducers(hproducerdescription.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdqFreeDiagnosticReport<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_REPORT>>(hreport: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqFreeDiagnosticReport(hreport: super::HDIAGNOSTIC_REPORT) -> ::windows::core::HRESULT;
        }
        DdqFreeDiagnosticReport(hreport.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdqGetDiagnosticDataAccessLevelAllowed() -> ::windows::core::Result<DdqAccessLevel> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticDataAccessLevelAllowed(accesslevel: *mut DdqAccessLevel) -> ::windows::core::HRESULT;
        }
        let mut result__: DdqAccessLevel = ::core::mem::zeroed();
        DdqGetDiagnosticDataAccessLevelAllowed(::core::mem::transmute(&mut result__)).from_abi::<DdqAccessLevel>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordAtIndex<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_RECORD>>(hrecord: Param0, index: u32) -> ::windows::core::Result<DIAGNOSTIC_DATA_RECORD> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordAtIndex(hrecord: super::HDIAGNOSTIC_RECORD, index: u32, record: *mut DIAGNOSTIC_DATA_RECORD) -> ::windows::core::HRESULT;
        }
        let mut result__: DIAGNOSTIC_DATA_RECORD = ::core::mem::zeroed();
        DdqGetDiagnosticRecordAtIndex(hrecord.into_param().abi(), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<DIAGNOSTIC_DATA_RECORD>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordBinaryDistribution<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, producernames: *const super::super::Foundation::PWSTR, producernamecount: u32, topnbinaries: u32, binarystats: *mut *mut DIAGNOSTIC_DATA_EVENT_BINARY_STATS, statcount: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordBinaryDistribution(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, producernames: *const super::super::Foundation::PWSTR, producernamecount: u32, topnbinaries: u32, binarystats: *mut *mut DIAGNOSTIC_DATA_EVENT_BINARY_STATS, statcount: *mut u32) -> ::windows::core::HRESULT;
        }
        DdqGetDiagnosticRecordBinaryDistribution(hsession.into_param().abi(), ::core::mem::transmute(producernames), ::core::mem::transmute(producernamecount), ::core::mem::transmute(topnbinaries), ::core::mem::transmute(binarystats), ::core::mem::transmute(statcount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordCategoryAtIndex<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>>(hcategorydescription: Param0, index: u32) -> ::windows::core::Result<DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordCategoryAtIndex(hcategorydescription: super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION, index: u32, categorydescription: *mut DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION) -> ::windows::core::HRESULT;
        }
        let mut result__: DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION = ::core::mem::zeroed();
        DdqGetDiagnosticRecordCategoryAtIndex(hcategorydescription.into_param().abi(), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordCategoryCount<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>>(hcategorydescription: Param0) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordCategoryCount(hcategorydescription: super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION, categorydescriptioncount: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        DdqGetDiagnosticRecordCategoryCount(hcategorydescription.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordCount<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_RECORD>>(hrecord: Param0) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordCount(hrecord: super::HDIAGNOSTIC_RECORD, recordcount: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        DdqGetDiagnosticRecordCount(hrecord.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordLocaleTagAtIndex<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>>(htagdescription: Param0, index: u32) -> ::windows::core::Result<DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordLocaleTagAtIndex(htagdescription: super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION, index: u32, tagdescription: *mut DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION) -> ::windows::core::HRESULT;
        }
        let mut result__: DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION = ::core::mem::zeroed();
        DdqGetDiagnosticRecordLocaleTagAtIndex(htagdescription.into_param().abi(), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordLocaleTagCount<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>>(htagdescription: Param0) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordLocaleTagCount(htagdescription: super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION, tagdescriptioncount: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        DdqGetDiagnosticRecordLocaleTagCount(htagdescription.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordLocaleTags<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hsession: Param0, locale: Param1) -> ::windows::core::Result<super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordLocaleTags(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, locale: super::super::Foundation::PWSTR, htagdescription: *mut super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION) -> ::windows::core::HRESULT;
        }
        let mut result__: super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION = ::core::mem::zeroed();
        DdqGetDiagnosticRecordLocaleTags(hsession.into_param().abi(), locale.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordPage<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, offset: u32, pagerecordcount: u32, baserowid: i64) -> ::windows::core::Result<super::HDIAGNOSTIC_RECORD> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordPage(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, offset: u32, pagerecordcount: u32, baserowid: i64, hrecord: *mut super::HDIAGNOSTIC_RECORD) -> ::windows::core::HRESULT;
        }
        let mut result__: super::HDIAGNOSTIC_RECORD = ::core::mem::zeroed();
        DdqGetDiagnosticRecordPage(hsession.into_param().abi(), ::core::mem::transmute(searchcriteria), ::core::mem::transmute(offset), ::core::mem::transmute(pagerecordcount), ::core::mem::transmute(baserowid), ::core::mem::transmute(&mut result__)).from_abi::<super::HDIAGNOSTIC_RECORD>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordPayload<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, rowid: i64) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordPayload(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, rowid: i64, payload: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        DdqGetDiagnosticRecordPayload(hsession.into_param().abi(), ::core::mem::transmute(rowid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordProducerAtIndex<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>>(hproducerdescription: Param0, index: u32) -> ::windows::core::Result<DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordProducerAtIndex(hproducerdescription: super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION, index: u32, producerdescription: *mut DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION) -> ::windows::core::HRESULT;
        }
        let mut result__: DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION = ::core::mem::zeroed();
        DdqGetDiagnosticRecordProducerAtIndex(hproducerdescription.into_param().abi(), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordProducerCategories<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hsession: Param0, producername: Param1) -> ::windows::core::Result<super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordProducerCategories(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, producername: super::super::Foundation::PWSTR, hcategorydescription: *mut super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION) -> ::windows::core::HRESULT;
        }
        let mut result__: super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION = ::core::mem::zeroed();
        DdqGetDiagnosticRecordProducerCategories(hsession.into_param().abi(), producername.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordProducerCount<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>>(hproducerdescription: Param0) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordProducerCount(hproducerdescription: super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION, producerdescriptioncount: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        DdqGetDiagnosticRecordProducerCount(hproducerdescription.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdqGetDiagnosticRecordProducers<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0) -> ::windows::core::Result<super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordProducers(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, hproducerdescription: *mut super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION) -> ::windows::core::HRESULT;
        }
        let mut result__: super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION = ::core::mem::zeroed();
        DdqGetDiagnosticRecordProducers(hsession.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordStats<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, recordcount: *mut u32, minrowid: *mut i64, maxrowid: *mut i64) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordStats(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, recordcount: *mut u32, minrowid: *mut i64, maxrowid: *mut i64) -> ::windows::core::HRESULT;
        }
        DdqGetDiagnosticRecordStats(hsession.into_param().abi(), ::core::mem::transmute(searchcriteria), ::core::mem::transmute(recordcount), ::core::mem::transmute(minrowid), ::core::mem::transmute(maxrowid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordSummary<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, producernames: *const super::super::Foundation::PWSTR, producernamecount: u32) -> ::windows::core::Result<DIAGNOSTIC_DATA_GENERAL_STATS> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordSummary(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, producernames: *const super::super::Foundation::PWSTR, producernamecount: u32, generalstats: *mut DIAGNOSTIC_DATA_GENERAL_STATS) -> ::windows::core::HRESULT;
        }
        let mut result__: DIAGNOSTIC_DATA_GENERAL_STATS = ::core::mem::zeroed();
        DdqGetDiagnosticRecordSummary(hsession.into_param().abi(), ::core::mem::transmute(producernames), ::core::mem::transmute(producernamecount), ::core::mem::transmute(&mut result__)).from_abi::<DIAGNOSTIC_DATA_GENERAL_STATS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordTagDistribution<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, producernames: *const super::super::Foundation::PWSTR, producernamecount: u32, tagstats: *mut *mut DIAGNOSTIC_DATA_EVENT_TAG_STATS, statcount: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordTagDistribution(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, producernames: *const super::super::Foundation::PWSTR, producernamecount: u32, tagstats: *mut *mut DIAGNOSTIC_DATA_EVENT_TAG_STATS, statcount: *mut u32) -> ::windows::core::HRESULT;
        }
        DdqGetDiagnosticRecordTagDistribution(hsession.into_param().abi(), ::core::mem::transmute(producernames), ::core::mem::transmute(producernamecount), ::core::mem::transmute(tagstats), ::core::mem::transmute(statcount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdqGetDiagnosticReport<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, reportstoretype: u32) -> ::windows::core::Result<super::HDIAGNOSTIC_REPORT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticReport(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype: u32, hreport: *mut super::HDIAGNOSTIC_REPORT) -> ::windows::core::HRESULT;
        }
        let mut result__: super::HDIAGNOSTIC_REPORT = ::core::mem::zeroed();
        DdqGetDiagnosticReport(hsession.into_param().abi(), ::core::mem::transmute(reportstoretype), ::core::mem::transmute(&mut result__)).from_abi::<super::HDIAGNOSTIC_REPORT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticReportAtIndex<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_REPORT>>(hreport: Param0, index: u32) -> ::windows::core::Result<DIAGNOSTIC_REPORT_DATA> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticReportAtIndex(hreport: super::HDIAGNOSTIC_REPORT, index: u32, report: *mut DIAGNOSTIC_REPORT_DATA) -> ::windows::core::HRESULT;
        }
        let mut result__: DIAGNOSTIC_REPORT_DATA = ::core::mem::zeroed();
        DdqGetDiagnosticReportAtIndex(hreport.into_param().abi(), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<DIAGNOSTIC_REPORT_DATA>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdqGetDiagnosticReportCount<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_REPORT>>(hreport: Param0) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticReportCount(hreport: super::HDIAGNOSTIC_REPORT, reportcount: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        DdqGetDiagnosticReportCount(hreport.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdqGetDiagnosticReportStoreReportCount<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, reportstoretype: u32) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticReportStoreReportCount(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype: u32, reportcount: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        DdqGetDiagnosticReportStoreReportCount(hsession.into_param().abi(), ::core::mem::transmute(reportstoretype), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdqGetSessionAccessLevel<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0) -> ::windows::core::Result<DdqAccessLevel> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetSessionAccessLevel(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, accesslevel: *mut DdqAccessLevel) -> ::windows::core::HRESULT;
        }
        let mut result__: DdqAccessLevel = ::core::mem::zeroed();
        DdqGetSessionAccessLevel(hsession.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<DdqAccessLevel>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdqGetTranscriptConfiguration<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0) -> ::windows::core::Result<DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetTranscriptConfiguration(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, currentconfig: *mut DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION) -> ::windows::core::HRESULT;
        }
        let mut result__: DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION = ::core::mem::zeroed();
        DdqGetTranscriptConfiguration(hsession.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqIsDiagnosticRecordSampledIn<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hsession: Param0, providergroup: *const ::windows::core::GUID, providerid: *const ::windows::core::GUID, providername: Param3, eventid: *const u32, eventname: Param5, eventversion: *const u32, eventkeywords: *const u64) -> ::windows::core::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqIsDiagnosticRecordSampledIn(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, providergroup: *const ::windows::core::GUID, providerid: *const ::windows::core::GUID, providername: super::super::Foundation::PWSTR, eventid: *const u32, eventname: super::super::Foundation::PWSTR, eventversion: *const u32, eventkeywords: *const u64, issampledin: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        DdqIsDiagnosticRecordSampledIn(hsession.into_param().abi(), ::core::mem::transmute(providergroup), ::core::mem::transmute(providerid), providername.into_param().abi(), ::core::mem::transmute(eventid), eventname.into_param().abi(), ::core::mem::transmute(eventversion), ::core::mem::transmute(eventkeywords), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdqSetTranscriptConfiguration<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, desiredconfig: *const DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqSetTranscriptConfiguration(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, desiredconfig: *const DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION) -> ::windows::core::HRESULT;
        }
        DdqSetTranscriptConfiguration(hsession.into_param().abi(), ::core::mem::transmute(desiredconfig)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}