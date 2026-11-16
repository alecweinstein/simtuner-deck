; Always create the desktop shortcut. The finish page checkbox is optional and unchecked by default.
!macro NSIS_HOOK_POSTINSTALL
  Call CreateOrUpdateDesktopShortcut
!macroend
