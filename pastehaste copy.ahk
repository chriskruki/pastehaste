#Requires AutoHotkey v2.0
#SingleInstance Force

SendMode "Input"
SetWorkingDir A_ScriptDir
SetKeyDelay 50, 50

accounts := ""

myGui := Gui("+AlwaysOnTop")
myGui.BackColor := "282828"
myGui.SetFont("s12 cWhite", "Verdana")

myGui.Add("Text", "x10 y10 cFFFFFF", "Enabled")
myGui.Add("CheckBox", "x80 y10 vPasteMode cFFFFFF Checked")

myGui.Add("Button", "x200 y10 w80 h20", "Save").OnEvent("Click", SaveInputs)

myGui.Add("Text", "x10 y40 cFFFFFF", "Alt 1:")
myGui.Add("Edit", "x80 y40 vKey1Input w200 cFFFFFF")
myGui.Add("Text", "x10 y75 cFFFFFF", "Alt 2:")
myGui.Add("Edit", "x80 y75 vKey2Input w200 cFFFFFF")
myGui.Add("Text", "x10 y110 cFFFFFF", "Alt 3:")
myGui.Add("Edit", "x80 y110 vKey3Input w200 cFFFFFF")
myGui.Add("Text", "x10 y145 cFFFFFF", "Alt 4:")
myGui.Add("Edit", "x80 y145 vKey4Input w200 cFFFFFF")
myGui.Add("Text", "x10 y180 cFFFFFF", "Alt 5:")
myGui.Add("Edit", "x80 y180 vKey5Input w200 cFFFFFF")
myGui.Add("Text", "x10 y215 cFFFFFF", "Alt Q:")
myGui.Add("Edit", "x80 y215 vKey6Input w200 cFFFFFF")
myGui.Add("Text", "x10 y250 cFFFFFF", "Alt W:")
myGui.Add("Edit", "x80 y250 vKey7Input w200 cFFFFFF")
myGui.Add("Text", "x10 y285 cFFFFFF", "Alt E:")
myGui.Add("Edit", "x80 y285 vKey8Input w200 cFFFFFF")
myGui.Add("Text", "x10 y320 cFFFFFF", "Alt R:")
myGui.Add("Edit", "x80 y320 vKey9Input w200 cFFFFFF")
myGui.Add("Text", "x10 y355 cFFFFFF", "Alt T:")
myGui.Add("Edit", "x80 y355 vKey10Input w200 cFFFFFF")

myGui.OnEvent("Close", (*) => ExitApp())
myGui.Show()
LoadKeyInputs()
if FileExist("accounts.txt")
    accounts := FileRead("accounts.txt")

!1:: {
    if PasteModeChecked() {
        KeyWait "Alt"
        saved := myGui.Submit(false)
        SendInput saved.Key1Input
    }
}

!2:: {
    if PasteModeChecked() {
        KeyWait "Alt"
        saved := myGui.Submit(false)
        SendInput saved.Key2Input
    }
}

!3:: {
    if PasteModeChecked() {
        KeyWait "Alt"
        saved := myGui.Submit(false)
        SendInput saved.Key3Input
    }
}

!4:: {
    if PasteModeChecked() {
        KeyWait "Alt"
        saved := myGui.Submit(false)
        SendInput saved.Key4Input
    }
}

!5:: {
    if PasteModeChecked() {
        KeyWait "Alt"
        saved := myGui.Submit(false)
        SendInput saved.Key5Input
    }
}

^!1:: {
    CopyToClipboardAndSetGuiControl("Key1Input")
}

^!2:: {
    CopyToClipboardAndSetGuiControl("Key2Input")
}

^!3:: {
    CopyToClipboardAndSetGuiControl("Key3Input")
}

^!4:: {
    CopyToClipboardAndSetGuiControl("Key4Input")
}

^!5:: {
    CopyToClipboardAndSetGuiControl("Key5Input")
}

; Paste Mode
; !q:: {
;     if PasteModeChecked() {
;         KeyWait "Alt"
;         saved := myGui.Submit(false)
;         SendInput saved.Key6Input
;     }
; }
; !w:: {
;     if PasteModeChecked() {
;         KeyWait "Alt"
;         saved := myGui.Submit(false)
;         SendInput saved.Key7Input
;     }
; }
; !e:: {
;     if PasteModeChecked() {
;         KeyWait "Alt"
;         saved := myGui.Submit(false)
;         SendInput saved.Key8Input
;     }
; }
; !r:: {
;     if PasteModeChecked() {
;         KeyWait "Alt"
;         saved := myGui.Submit(false)
;         SendInput saved.Key9Input
;     }
; }
; !t:: {
;     if PasteModeChecked() {
;         KeyWait "Alt"
;         saved := myGui.Submit(false)
;         SendInput saved.Key10Input
;     }
; }
; ^!q:: {
;     CopyToClipboardAndSetGuiControl("Key6Input")
; }
; ^!w:: {
;     CopyToClipboardAndSetGuiControl("Key7Input")
; }
; ^!e:: {
;     CopyToClipboardAndSetGuiControl("Key8Input")
; }
; ^!r:: {
;     CopyToClipboardAndSetGuiControl("Key9Input")
; }
; ^!t:: {
;     CopyToClipboardAndSetGuiControl("Key10Input")
; }

!w:: {
    KeyWait "Alt"
    if PasteModeChecked() {
        emailMatch := FetchWindowTitleEmail()
        if emailMatch
            SendInput emailMatch
    }
}

!e:: {
    if PasteModeChecked() {
        KeyWait "Alt"
        emailMatch := FetchWindowTitleEmail()

        if !emailMatch
            return

        Loop Parse, accounts, "`n"
        {
            account := StrSplit(A_LoopField, ",")
            username := account[1]
            password := account[2]

            if (username = emailMatch) {
                SendInput password
                return
            }
        }
    }
}

!s:: {
    if PasteModeChecked() {
        KeyWait "Alt"
        emailMatch := FetchWindowTitleEmail()

        if !emailMatch {
            MsgBox "No email found in window title"
            return
        }

        Loop Parse, accounts, "`n"
        {
            account := StrSplit(A_LoopField, ",")
            username := account[1]
            username := StrReplace(username, "`n", "")
            username := StrReplace(username, "`r", "")
            pw := account[2]
            pw := StrReplace(pw, "`n", "")
            pw := StrReplace(pw, "`r", "")

            if (username = emailMatch) {
                A_Clipboard := ""
                Send "^c"
                if !ClipWait(2) {
                    MsgBox "The attempt to copy text onto the clipboard failed."
                    return
                }
                copyText := A_Clipboard
                A_Clipboard := username . " " . pw . "`n" . copyText . "`n"
                break
            }
        }
    }
}

CopyToClipboardAndSetGuiControl(controlName) {
    KeyWait "Alt"
    KeyWait "Ctrl"
    Send "^c"
    Sleep 100
    if ClipWait(3) {
        newClip := A_Clipboard
        myGui[controlName].Value := newClip
    }
}

FetchWindowTitleEmail() {
    activeTitle := WinGetTitle("A")
    if RegExMatch(activeTitle, "^[^\s|]+", &emailMatch)
        return emailMatch[0]
    return ""
}

PasteModeChecked() {
    return myGui["PasteMode"].Value
}

TopRowChecked() {
    return myGui["TopPaste"].Value
}

SaveInputs(*) {
    saved := myGui.Submit(false)
    if FileExist("key_inputs.txt") {
        try FileDelete(A_ScriptDir "\key_inputs.txt")
        catch {
            MsgBox "Failed to delete key inputs file"
            return
        }
    }

    try FileAppend(
        "Key 1: " saved.Key1Input "`nKey 2: " saved.Key2Input "`nKey 3: " saved.Key3Input "`nKey 4: " saved.Key4Input "`nKey 5: " saved.Key5Input "`n",
        A_ScriptDir "\key_inputs.txt"
    )
    catch {
        MsgBox "Failed to save key inputs to file"
    }
}

LoadKeyInputs() {
    if FileExist("key_inputs.txt") {
        Key1Input := ""
        Key2Input := ""
        Key3Input := ""
        Key4Input := ""
        Key5Input := ""

        Loop Read, "key_inputs.txt"
        {
            if InStr(A_LoopReadLine, "Key 1:")
                Key1Input := SubStr(A_LoopReadLine, 8)
            if InStr(A_LoopReadLine, "Key 2:")
                Key2Input := SubStr(A_LoopReadLine, 8)
            if InStr(A_LoopReadLine, "Key 3:")
                Key3Input := SubStr(A_LoopReadLine, 8)
            if InStr(A_LoopReadLine, "Key 4:")
                Key4Input := SubStr(A_LoopReadLine, 8)
            if InStr(A_LoopReadLine, "Key 5:")
                Key5Input := SubStr(A_LoopReadLine, 8)
        }
        myGui["Key1Input"].Value := Key1Input
        myGui["Key2Input"].Value := Key2Input
        myGui["Key3Input"].Value := Key3Input
        myGui["Key4Input"].Value := Key4Input
        myGui["Key5Input"].Value := Key5Input
    }
}
