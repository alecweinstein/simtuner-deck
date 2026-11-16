; Always create the desktop shortcut. The finish page checkbox is optional and unchecked by default.
!macro NSIS_HOOK_POSTINSTALL
  Call CreateOrUpdateDesktopShortcut
!macroend

; 2.14.2 used product name "Sim Tuner Deck", binary simtuner-deck.exe, and uninstall key
; "Sim Tuner Deck". 2.14.3 is "Sim Deck" / sim-deck.exe, so the keys differ.
; Remove the previous install, its shortcuts, and its autostart entry before installing.
!macro NSIS_HOOK_PREINSTALL
  ExecWait 'taskkill /F /IM simtuner-deck.exe' $0
  ExecWait 'taskkill /F /IM sim-deck.exe' $0

  ReadRegStr $R0 HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Sim Tuner Deck" "UninstallString"
  ReadRegStr $R1 HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Sim Tuner Deck" "InstallLocation"
  ${If} $R0 != ""
  ${AndIf} $R1 != ""
    StrCpy $R2 $R1
    StrCpy $R3 $R2 1
    ${If} $R3 == "$\""
      StrCpy $R2 $R2 "" 1
      StrCpy $R2 $R2 -1
    ${EndIf}
    ExecWait '$R0 /S _?=$R2' $0
  ${EndIf}

  Delete "$DESKTOP\Sim Tuner Deck.lnk"
  Delete "$SMPROGRAMS\Sim Tuner Deck.lnk"
  DeleteRegValue HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "Sim Tuner Deck"
  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Sim Tuner Deck"
  DeleteRegKey HKCU "Software\Sim Tuner\Sim Tuner Deck"
  RMDir /r "$LOCALAPPDATA\Sim Tuner Deck"
!macroend
