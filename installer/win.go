package main

import (
	"bytes"
	"fmt"
	"os"
	"os/exec"
)

func createWindowsShortcut(linkName string, target string, arguments string, directory string, description string, destination string) {
	var scriptTxt bytes.Buffer
	scriptTxt.WriteString("option explicit\n\n")
	scriptTxt.WriteString("sub CreateShortCut()\n")
	scriptTxt.WriteString("dim objShell, strDesktopPath, objLink\n")
	scriptTxt.WriteString("set objShell = CreateObject(\"WScript.Shell\")\n")
	scriptTxt.WriteString("strDesktopPath = objShell.SpecialFolders(\"")
	scriptTxt.WriteString(destination)
	scriptTxt.WriteString("\")\n")
	scriptTxt.WriteString("set objLink = objShell.CreateShortcut(strDesktopPath & \"\\")
	scriptTxt.WriteString(linkName)
	scriptTxt.WriteString(".lnk\")\n")
	scriptTxt.WriteString("objLink.Arguments = \"")
	scriptTxt.WriteString(arguments)
	scriptTxt.WriteString("\"\n")
	scriptTxt.WriteString("objLink.Description = \"")
	scriptTxt.WriteString(description)
	scriptTxt.WriteString("\"\n")
	scriptTxt.WriteString("objLink.TargetPath = \"")
	scriptTxt.WriteString(target)
	scriptTxt.WriteString("\"\n")
	scriptTxt.WriteString("objLink.WindowStyle = 1\n")
	scriptTxt.WriteString("objLink.WorkingDirectory = \"")
	scriptTxt.WriteString(directory)
	scriptTxt.WriteString("\"\n")
	scriptTxt.WriteString("objLink.Save\nend sub\n\n")
	scriptTxt.WriteString("call CreateShortCut()")
	fmt.Print(scriptTxt.String())

	filename := fmt.Sprintf("lnkTo%s.vbs", destination)
	os.WriteFile(filename, scriptTxt.Bytes(), 0777)
	cmd := exec.Command("wscript", filename)
	err := cmd.Run()
	if err != nil {
		fmt.Println("Error creating shortcut")
		fmt.Println(err)
	}
	cmd.Wait()
	os.Remove(filename)
	return
}
