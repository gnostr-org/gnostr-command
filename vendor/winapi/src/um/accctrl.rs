// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::guiddef::GUID;
use shared::minwindef::{DWORD, ULONG};
use um::winbase::LocalFree;
use um::winnt::{HANDLE, LONG, LPSTR, LPWSTR, PVOID, SID};
#[inline]
pub unsafe fn AccFree(p: PVOID) -> PVOID {
    LocalFree(p)
}
ENUM!{enum SE_OBJECT_TYPE {
    SE_UNKNOWN_OBJECT_TYPE = 0,
    SE_FILE_OBJECT,
    SE_SERVICE,
    SE_PRINTER,
    SE_REGISTRY_KEY,
    SE_LMSHARE,
    SE_KERNEL_OBJECT,
    SE_WINDOW_OBJECT,
    SE_DS_OBJECT,
    SE_DS_OBJECT_ALL,
    SE_PROVIDER_DEFINED_OBJECT,
    SE_WMIGUID_OBJECT,
    SE_REGISTRY_WOW64_32KEY,
    SE_REGISTRY_WOW64_64KEY,
}}
ENUM!{enum TRUSTEE_TYPE {
    TRUSTEE_IS_UNKNOWN,
    TRUSTEE_IS_USER,
    TRUSTEE_IS_GROUP,
    TRUSTEE_IS_DOMAIN,
    TRUSTEE_IS_ALIAS,
    TRUSTEE_IS_WELL_KNOWN_GROUP,
    TRUSTEE_IS_DELETED,
    TRUSTEE_IS_INVALID,
    TRUSTEE_IS_COMPUTER,
}}
ENUM!{enum TRUSTEE_FORM {
    TRUSTEE_IS_SID,
    TRUSTEE_IS_NAME,
    TRUSTEE_BAD_FORM,
    TRUSTEE_IS_OBJECTS_AND_SID,
    TRUSTEE_IS_OBJECTS_AND_NAME,
}}
ENUM!{enum MULTIPLE_TRUSTEE_OPERATION {
    NO_MULTIPLE_TRUSTEE,
    TRUSTEE_IS_IMPERSONATE,
}}
STRUCT!{struct OBJECTS_AND_SID {
    ObjectsPresent: DWORD,
    ObjectTypeGuid: GUID,
    InheritedObjectTypeGuid: GUID,
    pSid: *mut SID,
}}
pub type POBJECTS_AND_SID = *mut OBJECTS_AND_SID;
STRUCT!{struct OBJECTS_AND_NAME_A {
    ObjectsPresent: DWORD,
    ObjectType: SE_OBJECT_TYPE,
    ObjectTypeName: LPSTR,
    InheritedObjectTypeName: LPSTR,
    ptstrName: LPSTR,
}}
pub type POBJECTS_AND_NAME_A = *mut OBJECTS_AND_NAME_A;
STRUCT!{struct OBJECTS_AND_NAME_W {
    ObjectsPresent: DWORD,
    ObjectType: SE_OBJECT_TYPE,
    ObjectTypeName: LPWSTR,
    InheritedObjectTypeName: LPWSTR,
    ptstrName: LPWSTR,
}}
pub type POBJECTS_AND_NAME_W = *mut OBJECTS_AND_NAME_W;
STRUCT!{struct TRUSTEE_A {
    pMultipleTrustee: *mut TRUSTEE_A,
    MultipleTrusteeOperation: MULTIPLE_TRUSTEE_OPERATION,
    TrusteeForm: TRUSTEE_FORM,
    TrusteeType: TRUSTEE_TYPE,
    ptstrName: LPSTR,
}}
pub type PTRUSTEE_A = *mut TRUSTEE_A;
pub type TRUSTEEA = TRUSTEE_A;
pub type PTRUSTEEA = PTRUSTEE_A;
STRUCT!{struct TRUSTEE_W {
    pMultipleTrustee: *mut TRUSTEE_W,
    MultipleTrusteeOperation: MULTIPLE_TRUSTEE_OPERATION,
    TrusteeForm: TRUSTEE_FORM,
    TrusteeType: TRUSTEE_TYPE,
    ptstrName: LPWSTR,
}}
pub type PTRUSTEE_W = *mut TRUSTEE_W;
pub type TRUSTEEW = TRUSTEE_W;
pub type PTRUSTEEW = PTRUSTEE_W;
ENUM!{enum ACCESS_MODE {
    NOT_USED_ACCESS = 0,
    GRANT_ACCESS,
    SET_ACCESS,
    DENY_ACCESS,
    REVOKE_ACCESS,
    SET_AUDIT_SUCCESS,
    SET_AUDIT_FAILURE,
}}
pub const NO_INHERITANCE: DWORD = 0x0;
pub const SUB_OBJECTS_ONLY_INHERIT: DWORD = 0x1;
pub const SUB_CONTAINERS_ONLY_INHERIT: DWORD = 0x2;
pub const SUB_CONTAINERS_AND_OBJECTS_INHERIT: DWORD = 0x3;
pub const INHERIT_NO_PROPAGATE: DWORD = 0x4;
pub const INHERIT_ONLY: DWORD = 0x8;
pub const INHERITED_ACCESS_ENTRY: DWORD = 0x10;
pub const INHERITED_PARENT: DWORD = 0x10000000;
pub const INHERITED_GRANDPARENT: DWORD = 0x20000000;
STRUCT!{struct EXPLICIT_ACCESS_A {
    grfAccessPermissions: DWORD,
    grfAccessMode: ACCESS_MODE,
    grfInheritance: DWORD,
    Trustee: TRUSTEE_A,
}}
pub type PEXPLICIT_ACCESS_A = *mut EXPLICIT_ACCESS_A;
pub type EXPLICIT_ACCESSA = EXPLICIT_ACCESS_A;
pub type PEXPLICIT_ACCESSA = PEXPLICIT_ACCESS_A;
STRUCT!{struct EXPLICIT_ACCESS_W {
    grfAccessPermissions: DWORD,
    grfAccessMode: ACCESS_MODE,
    grfInheritance: DWORD,
    Trustee: TRUSTEE_W,
}}
pub type PEXPLICIT_ACCESS_W = *mut EXPLICIT_ACCESS_W;
pub type EXPLICIT_ACCESSW = EXPLICIT_ACCESS_W;
pub type PEXPLICIT_ACCESSW = PEXPLICIT_ACCESS_W;
pub const ACCCTRL_DEFAULT_PROVIDER: &'static str = "Windows NT Access Provider";
pub type ACCESS_RIGHTS = ULONG;
pub type PACCESS_RIGHTS = *mut ACCESS_RIGHTS;
pub type INHERIT_FLAGS = ULONG;
pub type PINHERIT_FLAGS = *mut INHERIT_FLAGS;
STRUCT!{struct ACTRL_ACCESS_ENTRYA {
    Trustee: TRUSTEE_A,
    fAccessFlags: ULONG,
    Access: ACCESS_RIGHTS,
    ProvSpecificAccess: ACCESS_RIGHTS,
    Inheritance: INHERIT_FLAGS,
    lpInheritProperty: LPSTR,
}}
pub type PACTRL_ACCESS_ENTRYA = *mut ACTRL_ACCESS_ENTRYA;
STRUCT!{struct ACTRL_ACCESS_ENTRYW {
    Trustee: TRUSTEE_W,
    fAccessFlags: ULONG,
    Access: ACCESS_RIGHTS,
    ProvSpecificAccess: ACCESS_RIGHTS,
    Inheritance: INHERIT_FLAGS,
    lpInheritProperty: LPWSTR,
}}
pub type PACTRL_ACCESS_ENTRYW = *mut ACTRL_ACCESS_ENTRYW;
STRUCT!{struct ACTRL_ACCESS_ENTRY_LISTA {
    cEntries: ULONG,
    pAccessList: *mut ACTRL_ACCESS_ENTRYA,
}}
pub type PACTRL_ACCESS_ENTRY_LISTA = *mut ACTRL_ACCESS_ENTRY_LISTA;
STRUCT!{struct ACTRL_ACCESS_ENTRY_LISTW {
    cEntries: ULONG,
    pAccessList: *mut ACTRL_ACCESS_ENTRYW,
}}
pub type PACTRL_ACCESS_ENTRY_LISTW = *mut ACTRL_ACCESS_ENTRY_LISTW;
STRUCT!{struct ACTRL_PROPERTY_ENTRYA {
    lpProperty: LPSTR,
    pAccessEntryList: PACTRL_ACCESS_ENTRY_LISTA,
    fListFlags: ULONG,
}}
pub type PACTRL_PROPERTY_ENTRYA = *mut ACTRL_PROPERTY_ENTRYA;
STRUCT!{struct ACTRL_PROPERTY_ENTRYW {
    lpProperty: LPWSTR,
    pAccessEntryList: PACTRL_ACCESS_ENTRY_LISTW,
    fListFlags: ULONG,
}}
pub type PACTRL_PROPERTY_ENTRYW = *mut ACTRL_PROPERTY_ENTRYW;
STRUCT!{struct ACTRL_ACCESSA {
    cEntries: ULONG,
    pPropertyAccessList: PACTRL_PROPERTY_ENTRYA,
}}
pub type PACTRL_ACCESSA = *mut ACTRL_ACCESSA;
pub type ACTRL_AUDITA = ACTRL_ACCESSA;
pub type PACTRL_AUDITA = *mut ACTRL_AUDITA;
STRUCT!{struct ACTRL_ACCESSW {
    cEntries: ULONG,
    pPropertyAccessList: PACTRL_PROPERTY_ENTRYW,
}}
pub type PACTRL_ACCESSW = *mut ACTRL_ACCESSW;
pub type ACTRL_AUDITW = ACTRL_ACCESSW;
pub type PACTRL_AUDITW = *mut ACTRL_AUDITW;
pub const TRUSTEE_ACCESS_ALLOWED: ULONG = 0x00000001;
pub const TRUSTEE_ACCESS_READ: ULONG = 0x00000002;
pub const TRUSTEE_ACCESS_WRITE: ULONG = 0x00000004;
pub const TRUSTEE_ACCESS_EXPLICIT: ULONG = 0x00000001;
pub const TRUSTEE_ACCESS_READ_WRITE: ULONG = TRUSTEE_ACCESS_READ | TRUSTEE_ACCESS_WRITE;
pub const TRUSTEE_ACCESS_ALL: ULONG = 0xFFFFFFFF;
STRUCT!{struct TRUSTEE_ACCESSA {
    lpProperty: LPSTR,
    Access: ACCESS_RIGHTS,
    fAccessFlags: ULONG,
    fReturnedAccess: ULONG,
}}
pub type PTRUSTEE_ACCESSA = *mut TRUSTEE_ACCESSA;
STRUCT!{struct TRUSTEE_ACCESSW {
    lpProperty: LPWSTR,
    Access: ACCESS_RIGHTS,
    fAccessFlags: ULONG,
    fReturnedAccess: ULONG,
}}
pub type PTRUSTEE_ACCESSW = *mut TRUSTEE_ACCESSW;
pub const ACTRL_RESERVED: ULONG = 0x00000000;
pub const ACTRL_PERM_1: ULONG = 0x00000001;
pub const ACTRL_PERM_2: ULONG = 0x00000002;
pub const ACTRL_PERM_3: ULONG = 0x00000004;
pub const ACTRL_PERM_4: ULONG = 0x00000008;
pub const ACTRL_PERM_5: ULONG = 0x00000010;
pub const ACTRL_PERM_6: ULONG = 0x00000020;
pub const ACTRL_PERM_7: ULONG = 0x00000040;
pub const ACTRL_PERM_8: ULONG = 0x00000080;
pub const ACTRL_PERM_9: ULONG = 0x00000100;
pub const ACTRL_PERM_10: ULONG = 0x00000200;
pub const ACTRL_PERM_11: ULONG = 0x00000400;
pub const ACTRL_PERM_12: ULONG = 0x00000800;
pub const ACTRL_PERM_13: ULONG = 0x00001000;
pub const ACTRL_PERM_14: ULONG = 0x00002000;
pub const ACTRL_PERM_15: ULONG = 0x00004000;
pub const ACTRL_PERM_16: ULONG = 0x00008000;
pub const ACTRL_PERM_17: ULONG = 0x00010000;
pub const ACTRL_PERM_18: ULONG = 0x00020000;
pub const ACTRL_PERM_19: ULONG = 0x00040000;
pub const ACTRL_PERM_20: ULONG = 0x00080000;
pub const ACTRL_ACCESS_ALLOWED: ULONG = 0x00000001;
pub const ACTRL_ACCESS_DENIED: ULONG = 0x00000002;
pub const ACTRL_AUDIT_SUCCESS: ULONG = 0x00000004;
pub const ACTRL_AUDIT_FAILURE: ULONG = 0x00000008;
pub const ACTRL_ACCESS_PROTECTED: ULONG = 0x00000001;
pub const ACTRL_SYSTEM_ACCESS: ULONG = 0x04000000;
pub const ACTRL_DELETE: ULONG = 0x08000000;
pub const ACTRL_READ_CONTROL: ULONG = 0x10000000;
pub const ACTRL_CHANGE_ACCESS: ULONG = 0x20000000;
pub const ACTRL_CHANGE_OWNER: ULONG = 0x40000000;
pub const ACTRL_SYNCHRONIZE: ULONG = 0x80000000;
pub const ACTRL_STD_RIGHTS_ALL: ULONG = 0xf8000000;
pub const ACTRL_STD_RIGHT_REQUIRED: ULONG = ACTRL_STD_RIGHTS_ALL & !ACTRL_SYNCHRONIZE;
pub const ACTRL_DS_OPEN: ULONG = ACTRL_RESERVED;
pub const ACTRL_DS_CREATE_CHILD: ULONG = ACTRL_PERM_1;
pub const ACTRL_DS_DELETE_CHILD: ULONG = ACTRL_PERM_2;
pub const ACTRL_DS_LIST: ULONG = ACTRL_PERM_3;
pub const ACTRL_DS_SELF: ULONG = ACTRL_PERM_4;
pub const ACTRL_DS_READ_PROP: ULONG = ACTRL_PERM_5;
pub const ACTRL_DS_WRITE_PROP: ULONG = ACTRL_PERM_6;
pub const ACTRL_DS_DELETE_TREE: ULONG = ACTRL_PERM_7;
pub const ACTRL_DS_LIST_OBJECT: ULONG = ACTRL_PERM_8;
pub const ACTRL_DS_CONTROL_ACCESS: ULONG = ACTRL_PERM_9;
pub const ACTRL_FILE_READ: ULONG = ACTRL_PERM_1;
pub const ACTRL_FILE_WRITE: ULONG = ACTRL_PERM_2;
pub const ACTRL_FILE_APPEND: ULONG = ACTRL_PERM_3;
pub const ACTRL_FILE_READ_PROP: ULONG = ACTRL_PERM_4;
pub const ACTRL_FILE_WRITE_PROP: ULONG = ACTRL_PERM_5;
pub const ACTRL_FILE_EXECUTE: ULONG = ACTRL_PERM_6;
pub const ACTRL_FILE_READ_ATTRIB: ULONG = ACTRL_PERM_8;
pub const ACTRL_FILE_WRITE_ATTRIB: ULONG = ACTRL_PERM_9;
pub const ACTRL_FILE_CREATE_PIPE: ULONG = ACTRL_PERM_10;
pub const ACTRL_DIR_LIST: ULONG = ACTRL_PERM_1;
pub const ACTRL_DIR_CREATE_OBJECT: ULONG = ACTRL_PERM_2;
pub const ACTRL_DIR_CREATE_CHILD: ULONG = ACTRL_PERM_3;
pub const ACTRL_DIR_DELETE_CHILD: ULONG = ACTRL_PERM_7;
pub const ACTRL_DIR_TRAVERSE: ULONG = ACTRL_PERM_6;
pub const ACTRL_KERNEL_TERMINATE: ULONG = ACTRL_PERM_1;
pub const ACTRL_KERNEL_THREAD: ULONG = ACTRL_PERM_2;
pub const ACTRL_KERNEL_VM: ULONG = ACTRL_PERM_3;
pub const ACTRL_KERNEL_VM_READ: ULONG = ACTRL_PERM_4;
pub const ACTRL_KERNEL_VM_WRITE: ULONG = ACTRL_PERM_5;
pub const ACTRL_KERNEL_DUP_HANDLE: ULONG = ACTRL_PERM_6;
pub const ACTRL_KERNEL_PROCESS: ULONG = ACTRL_PERM_7;
pub const ACTRL_KERNEL_SET_INFO: ULONG = ACTRL_PERM_8;
pub const ACTRL_KERNEL_GET_INFO: ULONG = ACTRL_PERM_9;
pub const ACTRL_KERNEL_CONTROL: ULONG = ACTRL_PERM_10;
pub const ACTRL_KERNEL_ALERT: ULONG = ACTRL_PERM_11;
pub const ACTRL_KERNEL_GET_CONTEXT: ULONG = ACTRL_PERM_12;
pub const ACTRL_KERNEL_SET_CONTEXT: ULONG = ACTRL_PERM_13;
pub const ACTRL_KERNEL_TOKEN: ULONG = ACTRL_PERM_14;
pub const ACTRL_KERNEL_IMPERSONATE: ULONG = ACTRL_PERM_15;
pub const ACTRL_KERNEL_DIMPERSONATE: ULONG = ACTRL_PERM_16;
pub const ACTRL_PRINT_SADMIN: ULONG = ACTRL_PERM_1;
pub const ACTRL_PRINT_SLIST: ULONG = ACTRL_PERM_2;
pub const ACTRL_PRINT_PADMIN: ULONG = ACTRL_PERM_3;
pub const ACTRL_PRINT_PUSE: ULONG = ACTRL_PERM_4;
pub const ACTRL_PRINT_JADMIN: ULONG = ACTRL_PERM_5;
pub const ACTRL_SVC_GET_INFO: ULONG = ACTRL_PERM_1;
pub const ACTRL_SVC_SET_INFO: ULONG = ACTRL_PERM_2;
pub const ACTRL_SVC_STATUS: ULONG = ACTRL_PERM_3;
pub const ACTRL_SVC_LIST: ULONG = ACTRL_PERM_4;
pub const ACTRL_SVC_START: ULONG = ACTRL_PERM_5;
pub const ACTRL_SVC_STOP: ULONG = ACTRL_PERM_6;
pub const ACTRL_SVC_PAUSE: ULONG = ACTRL_PERM_7;
pub const ACTRL_SVC_INTERROGATE: ULONG = ACTRL_PERM_8;
pub const ACTRL_SVC_UCONTROL: ULONG = ACTRL_PERM_9;
pub const ACTRL_REG_QUERY: ULONG = ACTRL_PERM_1;
pub const ACTRL_REG_SET: ULONG = ACTRL_PERM_2;
pub const ACTRL_REG_CREATE_CHILD: ULONG = ACTRL_PERM_3;
pub const ACTRL_REG_LIST: ULONG = ACTRL_PERM_4;
pub const ACTRL_REG_NOTIFY: ULONG = ACTRL_PERM_5;
pub const ACTRL_REG_LINK: ULONG = ACTRL_PERM_6;
pub const ACTRL_WIN_CLIPBRD: ULONG = ACTRL_PERM_1;
pub const ACTRL_WIN_GLOBAL_ATOMS: ULONG = ACTRL_PERM_2;
pub const ACTRL_WIN_CREATE: ULONG = ACTRL_PERM_3;
pub const ACTRL_WIN_LIST_DESK: ULONG = ACTRL_PERM_4;
pub const ACTRL_WIN_LIST: ULONG = ACTRL_PERM_5;
pub const ACTRL_WIN_READ_ATTRIBS: ULONG = ACTRL_PERM_6;
pub const ACTRL_WIN_WRITE_ATTRIBS: ULONG = ACTRL_PERM_7;
pub const ACTRL_WIN_SCREEN: ULONG = ACTRL_PERM_8;
pub const ACTRL_WIN_EXIT: ULONG = ACTRL_PERM_9;
UNION!{union ACTRL_OVERLAPPED_u {
    [u32; 1] [u64; 1],
    Provider Provider_mut: PVOID,
    Reserved1 Reserved1_mut: ULONG,
}}
STRUCT!{struct ACTRL_OVERLAPPED {
    u: ACTRL_OVERLAPPED_u,
    Reserved2: ULONG,
    hEvent: HANDLE,
}}
pub type PACTRL_OVERLAPPED = *mut ACTRL_OVERLAPPED;
STRUCT!{struct ACTRL_ACCESS_INFOA {
    fAccessPermission: ULONG,
    lpAccessPermissionName: LPSTR,
}}
pub type PACTRL_ACCESS_INFOA = *mut ACTRL_ACCESS_INFOA;
STRUCT!{struct ACTRL_ACCESS_INFOW {
    fAccessPermission: ULONG,
    lpAccessPermissionName: LPWSTR,
}}
pub type PACTRL_ACCESS_INFOW = *mut ACTRL_ACCESS_INFOW;
STRUCT!{struct ACTRL_CONTROL_INFOA {
    lpControlId: LPSTR,
    lpControlName: LPSTR,
}}
pub type PACTRL_CONTROL_INFOA = *mut ACTRL_CONTROL_INFOA;
STRUCT!{struct ACTRL_CONTROL_INFOW {
    lpControlId: LPWSTR,
    lpControlName: LPWSTR,
}}
pub type PACTRL_CONTROL_INFOW = *mut ACTRL_CONTROL_INFOW;
pub const ACTRL_ACCESS_NO_OPTIONS: DWORD = 0x00000000;
pub const ACTRL_ACCESS_SUPPORTS_OBJECT_ENTRIES: DWORD = 0x00000001;
pub const TREE_SEC_INFO_SET: DWORD = 0x00000001;
pub const TREE_SEC_INFO_RESET: DWORD = 0x00000002;
pub const TREE_SEC_INFO_RESET_KEEP_EXPLICIT: DWORD = 0x00000003;
ENUM!{enum PROG_INVOKE_SETTING {
    ProgressInvokeNever = 1,
    ProgressInvokeEveryObject,
    ProgressInvokeOnError,
    ProgressCancelOperation,
    ProgressRetryOperation,
    ProgressInvokePrePostError,
}}
pub type PPROG_INVOKE_SETTING = *mut PROG_INVOKE_SETTING;
STRUCT!{struct FN_OBJECT_MGR_FUNCTS {
    Placeholder: ULONG,
}}
pub type PFN_OBJECT_MGR_FUNCTS = *mut FN_OBJECT_MGR_FUNCTS;
STRUCT!{struct INHERITED_FROMA {
    GenerationGap: LONG,
    AncestorName: LPSTR,
}}
pub type PINHERITED_FROMA = *mut INHERITED_FROMA;
STRUCT!{struct INHERITED_FROMW {
    GenerationGap: LONG,
    AncestorName: LPWSTR,
}}
pub type PINHERITED_FROMW = *mut INHERITED_FROMW;
