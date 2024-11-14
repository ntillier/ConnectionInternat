package main

import (
	"fmt"
	"io"
	"os"
	"path/filepath"
)

// MoveFolder moves a folder (or its contents) to a new location, overwriting the target if it exists.
func MoveFolder(src, dst string, overwrite bool) error {
	// Check if the source directory exists
	srcInfo, err := os.Stat(src)
	if err != nil {
		return fmt.Errorf("source directory does not exist: %w", err)
	}
	if !srcInfo.IsDir() {
		return fmt.Errorf("source is not a directory")
	}

	// If the destination directory exists and overwrite is true, remove the destination directory
	if _, err := os.Stat(dst); err == nil {
		if overwrite {
			if err := os.RemoveAll(dst); err != nil {
				return fmt.Errorf("failed to remove destination directory: %w", err)
			}
		} else {
			return fmt.Errorf("destination directory already exists and overwrite is false")
		}
	}

	// Try to rename the source folder to the destination (this is the simplest move)
	err = os.Rename(src, dst)
	if err != nil {
		fmt.Printf("Renaming folder didn't work, copying contents...\n")
		// If Rename fails, try copying the contents (e.g., because they're on different filesystems)
		return moveContents(src, dst)
	}

	return nil
}

// moveContents copies the contents of the source directory to the target directory.
func moveContents(src, dst string) error {
	// Open the source directory
	entries, err := os.ReadDir(src)
	if err != nil {
		return fmt.Errorf("failed to read source directory: %w", err)
	}

	// Create the destination directory
	if err := os.MkdirAll(dst, 0755); err != nil {
		return fmt.Errorf("failed to create destination directory: %w", err)
	}

	// Copy all files and subdirectories from source to destination
	for _, entry := range entries {
		srcPath := filepath.Join(src, entry.Name())
		dstPath := filepath.Join(dst, entry.Name())

		if entry.IsDir() {
			// Recursively move subdirectories
			if err := MoveFolder(srcPath, dstPath, true); err != nil {
				return fmt.Errorf("failed to move subdirectory %s: %w", entry.Name(), err)
			}
		} else {
			// Move files
			if err := moveFile(srcPath, dstPath); err != nil {
				return fmt.Errorf("failed to move file %s: %w", entry.Name(), err)
			}
		}
	}

	// Once everything is moved, remove the source directory
	return os.RemoveAll(src)
}

// moveFile moves a single file from src to dst
func moveFile(src, dst string) error {
	// Open the source file
	srcFile, err := os.Open(src)
	if err != nil {
		return fmt.Errorf("failed to open source file %s: %w", src, err)
	}
	defer srcFile.Close()

	// Create the destination file
	dstFile, err := os.Create(dst)
	if err != nil {
		return fmt.Errorf("failed to create destination file %s: %w", dst, err)
	}
	defer dstFile.Close()

	// Copy the contents of the source file to the destination file
	_, err = io.Copy(dstFile, srcFile)
	if err != nil {
		return fmt.Errorf("failed to copy file content from %s to %s: %w", src, dst, err)
	}

	srcFileInfo, err := srcFile.Stat()
	if err != nil {
		return fmt.Errorf("failed to get source file info: %w", err)
	}

	// Set the same permissions as the source file
	if err := os.Chmod(dst, srcFileInfo.Mode()); err != nil {
		return fmt.Errorf("failed to set file permissions for %s: %w", dst, err)
	}

	// Remove the source file
	if err := os.Remove(src); err != nil {
		return fmt.Errorf("failed to remove source file %s: %w", src, err)
	}

	return nil
}
