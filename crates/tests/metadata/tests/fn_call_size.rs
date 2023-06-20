use metadata::*;

#[test]
fn size() {
    // Note: you can double check these export names from a Visual Studio x86 command prompt as follows:
    // dumpbin /exports kernel32.lib | findstr /i RtmConvertIpv6AddressAndLengthToNetAddress

    let files = tool_lib::default_metadata();
    let reader = &Reader::new(&files);

    assert_eq!(
        struct_size(reader, "Windows.Win32.System.Com", "VARIANT"),
        16
    );
    assert_eq!(
        struct_size(
            reader,
            "Windows.Win32.Devices.AllJoyn",
            "alljoyn_interfacedescription_property"
        ),
        16
    );
    assert_eq!(
        struct_size(reader, "Windows.Win32.Networking.WinSock", "IN6_ADDR"),
        16
    );
    assert_eq!(
        struct_size(
            reader,
            "Windows.Win32.Devices.BiometricFramework",
            "WINBIO_IDENTITY"
        ),
        76
    );

    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Devices.AllJoyn",
            "alljoyn_interfacedescription_property_eql"
        ),
        32
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Devices.AllJoyn",
            "alljoyn_interfacedescription_property_getannotation"
        ),
        28
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Devices.AllJoyn",
            "alljoyn_interfacedescription_property_getannotationatindex"
        ),
        36
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Devices.AllJoyn",
            "alljoyn_interfacedescription_property_getannotationscount"
        ),
        16
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Devices.BiometricFramework",
            "WinBioGetCredentialState"
        ),
        84
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Devices.BiometricFramework",
            "WinBioRemoveCredential"
        ),
        80
    );
    assert_eq!(
        function_size(reader, "Windows.Win32.Graphics.Gdi", "AlphaBlend"),
        44
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Networking.Clustering",
            "RegisterClusterNotifyV2"
        ),
        28
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.NetworkManagement.Rras",
            "RtmConvertIpv6AddressAndLengthToNetAddress"
        ),
        28
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.NetworkManagement.WiFi",
            "WlanSetProfileEapUserData"
        ),
        44
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Security.Authentication.Identity",
            "AcceptSecurityContext"
        ),
        36
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Security.Authentication.Identity",
            "InitializeSecurityContextA"
        ),
        48
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Security.Authentication.Identity",
            "InitializeSecurityContextW"
        ),
        48
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Security.Authentication.Identity",
            "SaslAcceptSecurityContext"
        ),
        36
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Security.Authentication.Identity",
            "SaslInitializeSecurityContextA"
        ),
        48
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Security.Authentication.Identity",
            "SaslInitializeSecurityContextW"
        ),
        48
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Security.ExtensibleAuthenticationProtocol",
            "EapHostPeerBeginSession"
        ),
        68
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Security.ExtensibleAuthenticationProtocol",
            "EapHostPeerConfigBlob2Xml"
        ),
        36
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Security.ExtensibleAuthenticationProtocol",
            "EapHostPeerGetIdentity"
        ),
        68
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Security.ExtensibleAuthenticationProtocol",
            "EapHostPeerGetMethodProperties"
        ),
        52
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Security.ExtensibleAuthenticationProtocol",
            "EapHostPeerInvokeConfigUI"
        ),
        44
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Security.ExtensibleAuthenticationProtocol",
            "EapHostPeerInvokeIdentityUI"
        ),
        64
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Security.ExtensibleAuthenticationProtocol",
            "EapHostPeerQueryCredentialInputFields"
        ),
        40
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Security.ExtensibleAuthenticationProtocol",
            "EapHostPeerQueryUserBlobFromCredentialInputFields"
        ),
        48
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Storage.CloudFilters",
            "CfDisconnectSyncRoot"
        ),
        8
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Storage.CloudFilters",
            "CfQuerySyncProviderStatus"
        ),
        12
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Storage.CloudFilters",
            "CfReportProviderProgress"
        ),
        32
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Storage.CloudFilters",
            "CfReportProviderProgress2"
        ),
        44
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.Storage.CloudFilters",
            "CfUpdateSyncProviderStatus"
        ),
        12
    );
    assert_eq!(
        function_size(reader, "Windows.Win32.System.Com", "GetErrorInfo"),
        8
    );
    assert_eq!(
        function_size(reader, "Windows.Win32.System.Console", "ReadConsoleOutputA"),
        20
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.System.Console",
            "ReadConsoleOutputAttribute"
        ),
        20
    );
    assert_eq!(
        function_size(reader, "Windows.Win32.System.Ole", "VarI2FromCy"),
        12
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.UI.Accessibility",
            "ItemContainerPattern_FindItemByProperty"
        ),
        32
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.UI.Accessibility",
            "TextRange_FindAttribute"
        ),
        32
    );
    assert_eq!(
        function_size(
            reader,
            "Windows.Win32.UI.Accessibility",
            "UiaRaiseAutomationPropertyChangedEvent"
        ),
        40
    );
}

fn function_size(reader: &Reader, namespace: &str, name: &str) -> usize {
    for method in reader.namespace_functions(namespace) {
        if reader.method_def_name(method) == name {
            return reader.method_def_size(method);
        }
    }
    0
}

fn struct_size(reader: &Reader, namespace: &str, name: &str) -> usize {
    for def in reader.get(TypeName::new(namespace, name)) {
        return reader.type_def_size(def);
    }
    0
}
