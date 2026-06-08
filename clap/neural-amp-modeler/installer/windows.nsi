; NSIS installer for Neural Amp Modeler CLAP plugin
;
; Defines passed from CI (all required):
;   PLUGIN_FILE  — absolute path to the .clap file
;   DISPLAY_NAME — human-readable name shown in the installer
;   VERSION      — version string, e.g. "1.0.0"
;   OUTFILE      — absolute path for the output .exe

!ifndef DISPLAY_NAME
  !define DISPLAY_NAME "Neural Amp Modeler"
!endif
!ifndef VERSION
  !define VERSION "0.0.0"
!endif
!ifndef OUTFILE
  !define OUTFILE "installer.exe"
!endif

Name "${DISPLAY_NAME} ${VERSION}"
OutFile "${OUTFILE}"

; C:\Program Files\Common Files\CLAP  (64-bit)
InstallDir "$COMMONFILES64\CLAP"

; Requires UAC elevation to write to Program Files
RequestExecutionLevel admin

!include "MUI2.nsh"

!define MUI_ABORTWARNING

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_LANGUAGE "English"

Section "CLAP Plugin"
  SetOutPath "$INSTDIR"
  File "${PLUGIN_FILE}"
SectionEnd
