#NoEnv
SendMode Input
SetWorkingDir %A_ScriptDir%

Gui +AlwaysOnTop
Gui, Color, 282828, 707070
Gui, Font, s12 cWhite, Verdana ; Set the font size to 12 and color to white, and the font to Verdana

Gui, Add, Text, x10 y10 cFFFFFF, Enabled
Gui, Add, CheckBox, x80 y10 vPasteMode cFFFFFF Checked

Gui, Add, Button, x200 y10 w80 h20 gSaveInputs, Save

Gui, Add, Text, x10 y40 cFFFFFF, Alt 1:
Gui, Add, Edit, x80 y40 vKey1Input w200 cFFFFFF
Gui, Add, Text, x10 y75 cFFFFFF, Alt 2:
Gui, Add, Edit, x80 y75 vKey2Input w200 cFFFFFF
Gui, Add, Text, x10 y110 cFFFFFF, Alt 3:
Gui, Add, Edit, x80 y110 vKey3Input w200 cFFFFFF
Gui, Add, Text, x10 y145 cFFFFFF, Alt 4:
Gui, Add, Edit, x80 y145 vKey4Input w200 cFFFFFF
Gui, Add, Text, x10 y180 cFFFFFF, Alt 5:
Gui, Add, Edit, x80 y180 vKey5Input w200 cFFFFFF
Gui, Add, Text, x10 y215 cFFFFFF, Alt Q:
Gui, Add, Edit, x80 y215 vKey6Input w200 cFFFFFF
Gui, Add, Text, x10 y250 cFFFFFF, Alt W:
Gui, Add, Edit, x80 y250 vKey7Input w200 cFFFFFF
Gui, Add, Text, x10 y285 cFFFFFF, Alt E:
Gui, Add, Edit, x80 y285 vKey8Input w200 cFFFFFF
Gui, Add, Text, x10 y320 cFFFFFF, Alt R:
Gui, Add, Edit, x80 y320 vKey9Input w200 cFFFFFF
Gui, Add, Text, x10 y355 cFFFFFF, Alt T:
Gui, Add, Edit, x80 y355 vKey10Input w200 cFFFFFF

Gui, Show,, Paste Haste
LoadKeyInputs()
FileRead, accounts, accounts.txt
Return
SetKeyDelay, 50, 50

!1::
    if (PasteModeChecked())
        KeyWait, Alt, 1
    {
        Gui, Submit, NoHide
        SendInput %Key1Input%
    }
Return

!2::
    if (PasteModeChecked())
    {
        KeyWait, Alt, 2
        Gui, Submit, NoHide
        SendInput %Key2Input%
    }
Return

!3::
    if (PasteModeChecked())
    {
        KeyWait, Alt, 3
        Gui, Submit, NoHide
        SendInput %Key3Input%
    }
Return

!4::
    if (PasteModeChecked())
    {
        KeyWait, Alt, 4
        Gui, Submit, NoHide
        SendInput %Key4Input%
    }
Return

!5::
    if (PasteModeChecked())
    {
        KeyWait, Alt, 5
        Gui, Submit, NoHide
        SendInput %Key5Input%
    }
Return

^!1::
    CopyToClipboardAndSetGuiControl("Key1Input")
Return

^!2::
    CopyToClipboardAndSetGuiControl("Key2Input")
Return

^!3::
    CopyToClipboardAndSetGuiControl("Key3Input")
Return

^!4::
    CopyToClipboardAndSetGuiControl("Key4Input")
Return

^!5::
    CopyToClipboardAndSetGuiControl("Key5Input")
Return

; Paste Mode
; !q::
;     if (PasteModeChecked())
;     {
;         KeyWait, Alt, q
;         Gui, Submit, NoHide
;         SendInput %Key6Input%
;     }
; Return
; !w::
;     if (PasteModeChecked())
;     {
;         KeyWait, Alt,
;         Gui, Submit, NoHide
;         SendInput %Key7Input%
;     }
; Return
; !e::
;     if (PasteModeChecked())
;     {
;         KeyWait, Alt, e
;         Gui, Submit, NoHide
;         SendInput %Key8Input%
;     }
; Return
; !r::
;     if (PasteModeChecked())
;     {
;         KeyWait, Alt, r
;         Gui, Submit, NoHide
;         SendInput %Key9Input%
;     }
; Return
; !t::
;     if (PasteModeChecked())
;     {
;         KeyWait, Alt, t
;         Gui, Submit, NoHide
;         SendInput %Key10Input%
;     }
; Return
; ^!q::
;     CopyToClipboardAndSetGuiControl("Key6Input")
; Return
; ^!w::
;     CopyToClipboardAndSetGuiControl("Key7Input")
; Return
; ^!e::
;     CopyToClipboardAndSetGuiControl("Key8Input")
; Return
; ^!r::
;     CopyToClipboardAndSetGuiControl("Key9Input")
; Return
; ^!t::
;     CopyToClipboardAndSetGuiControl("Key10Input")
; Return

!w::
    KeyWait, Alt, w
    if (PasteModeChecked()) {
        KeyWait, Alt, w
        emailMatch := FetchWindowTitleEmail()
        if (emailMatch)
        {
            SendInput %emailMatch%
        }
    }

Return

!e::
    if (PasteModeChecked()) {
        KeyWait, Alt, e
        emailMatch := FetchWindowTitleEmail()

        if (!emailMatch) {
            Return
        }

        Loop, Parse, accounts, `n
        {
            account := StrSplit(A_LoopField, ",")
            username := account[1]
            password := account[2]

            if (username = emailMatch)
            {
                SendInput %password%
                Return
            }
        }
    }
Return

!s::
    if (PasteModeChecked()) {
        KeyWait, Alt, s
        emailMatch := FetchWindowTitleEmail()

        if (!emailMatch) {
            MsgBox, No email found in window title
            return
        }

        Loop, Parse, accounts, `n
        {
            account := StrSplit(A_LoopField, ",")
            username := account[1]
            username := StrReplace(username, "`n", "")
            username := StrReplace(username, "`r", "")
            pw := account[2]
            pw := StrReplace(pw, "`n", "")
            pw := StrReplace(pw, "`r", "")

            if (username = emailMatch)
            {
                ClipBoard := ""
                Send, ^c
                ClipWait, 2
                if ErrorLevel {
                    MsgBox, The attempt to copy text onto the clipboard failed.
                    return
                }
                copyText := Clipboard
                ; copyText := StrReplace(copyText, "`n", "")
                ; copyText := StrReplace(copyText, "`r", "")
                Clipboard := username . " " . pw . "`n" . copyText . "`n"
                Break
            } else {
                ; MsgBox, No account found for email: %emailMatch%
            }
        }
    }
Return

CopyToClipboardAndSetGuiControl(controlName)
{
    KeyWait, Alt, Ctrl
    Send, ^c
    Sleep, 100
    ClipWait, 3
    newClip := Clipboard
    if (!ErrorLevel) ; If text was successfully copied to the clipboard
    {
        GuiControl,, %controlName%, %newClip%
    }
}

FetchWindowTitleEmail()
{
    WinGetTitle, activeTitle, A
    ; RegExMatch(activeTitle, "i)\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}\b", emailMatch)
    RegExMatch(activeTitle, "^[^\s|]+", emailMatch)
    return emailMatch
}

PasteModeChecked()
{
    GuiControlGet, PasteMode
    Return PasteMode
}

TopRowChecked()
{
    GuiControlGet, TopPaste
    Return TopPaste
}

SaveInputs()
{
    global
    Gui, Submit, NoHide
    if (FileExist("key_inputs.txt")) {
        FileDelete, %A_ScriptDir%\key_inputs.txt
        if (ErrorLevel) {
            MsgBox, Failed to delete key inputs file
            Return
        }
    }

    FileAppend, Key 1: %Key1Input%`nKey 2: %Key2Input%`nKey 3: %Key3Input%`nKey 4: %Key4Input%`nKey 5: %Key5Input%`n, %A_ScriptDir%\key_inputs.txt
    if (ErrorLevel) {
        MsgBox, Failed to save key inputs to file
        Return
    }
}

LoadKeyInputs()
{
    global
    if FileExist("key_inputs.txt")
    {
        Loop, Read, key_inputs.txt
        {
            IfInString, A_LoopReadLine, Key 1:
                Key1Input := SubStr(A_LoopReadLine, 8)
            IfInString, A_LoopReadLine, Key 2:
                Key2Input := SubStr(A_LoopReadLine, 8)
            IfInString, A_LoopReadLine, Key 3:
                Key3Input := SubStr(A_LoopReadLine, 8)
            IfInString, A_LoopReadLine, Key 4:
                Key4Input := SubStr(A_LoopReadLine, 8)
            IfInString, A_LoopReadLine, Key 5:
                Key5Input := SubStr(A_LoopReadLine, 8)
        }
        GuiControl,, Key1Input, %Key1Input%
        GuiControl,, Key2Input, %Key2Input%
        GuiControl,, Key3Input, %Key3Input%
        GuiControl,, Key4Input, %Key4Input%
        GuiControl,, Key5Input, %Key5Input%
    }
}

GuiClose:
ExitApp