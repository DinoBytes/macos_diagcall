# macos_diagcall
Rust wrappers for making diagnostic system calls on macOS

## Enabling Diagnostic Syscalls

The functions in this library will fail without diagnostic calls being enabled.

Modify the `Kernel Flags` portion of "/Library/Preferences/SystemConfiguration/com.apple.Boot.plist" to look like this:

```
<key>Kernel Flags</key>
 <string>diag=0x00000008</string> 
```

Reboot.

Other boot args may be present and other configurations are possible. The important part is that the bit above be set. (https://sethc23.github.io/wiki/OSX/Mac_OS_X_and_iOS_Internals_To_the_Apple's_Core.pdf page 294)
