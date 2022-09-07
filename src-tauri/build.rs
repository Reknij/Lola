fn main() {
    tauri_build::build();
    if cfg!(windows) {
        let mut res = winres::WindowsResource::new();
        res.set_icon("./icons/icon.ico").set_manifest(
            r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
<security>
    <requestedPrivileges>
        <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
    </requestedPrivileges>
</security>
</trustInfo>
</assembly>
"#,
        );
        res.compile().expect("Compile manifest failed!");
    }
}
