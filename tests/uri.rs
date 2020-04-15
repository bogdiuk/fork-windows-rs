winrt::import!(
    dependencies
        "os"
    modules
        "windows.foundation"
);

#[test]
fn uri() -> winrt::Result<()> {
    use windows::foundation::Uri;

    assert!(<Uri as winrt::TypeName>::type_name() == "Windows.Foundation.Uri");

    assert!(
        <Uri as winrt::TypeGuid>::type_guid()
            == &winrt::Guid::from("9E365E57-48B2-4160-956F-C7385120BBFC") // IUriRuntimeClass
    );

    let _uri = Uri::default();

    //     let uri = &Uri::create_uri("http://kennykerr.ca")?;
    //     assert!(uri.domain()? == "kennykerr.ca");
    //     assert!(uri.port()? == 80);

    //     // Calls QueryInterface followed by IStringable::ToString under the hood
    //     assert!(uri.to_string()? == "http://kennykerr.ca/");

    //     let default: IUriRuntimeClass = uri.into();
    //     assert!(default.domain()? == uri.domain()?);

    //     let stringable: IStringable = uri.into();
    //     assert!(stringable.to_string()? == uri.to_string()?);

    Ok(())
}