!include "tauri-utils.nsi"

!define TAURI_PRODUCT_NAME "OpenCode"
!define TAURI_VERSION "1.17.17"
!define TAURI_COMPANY_NAME "OpenCode"
!define TAURI_COPYRIGHT "Copyright © 2024 OpenCode"
!define TAURI_MAIN_EXE "opencode-desktop.exe"

Function .onInit
  Call tauri_utils_onInit
FunctionEnd

Function .onInstSuccess
  Call tauri_utils_onInstSuccess
FunctionEnd

Function un.onUninstSuccess
  Call tauri_utils_onUninstSuccess
FunctionEnd

; Include node-dist folder in the installer
Section "node-dist" SEC_NODE_DIST
  SetOutPath "$INSTDIR\node-dist"
  File /r "..\..\opencode\dist\node\*.*"
SectionEnd

!insertmacro TAURI_MAIN_SECTION
  SectionIn 1
  Call tauri_utils_mainSection
  ; Ensure node-dist is installed
  SectionIn 1
  Section "node-dist-files" SEC_NODE_DIST_FILES
    SetOutPath "$INSTDIR\node-dist"
    File /r "..\..\opencode\dist\node\*.*"
  SectionEnd
!insertmacro TAURI_MAIN_SECTION_END