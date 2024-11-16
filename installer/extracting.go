package main

import (
	"archive/tar"
	"archive/zip"
	"compress/gzip"
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
)

// DownloadAndExtractTarGz downloads a .tar.gz file from the URL and extracts it to the given directory.
func DownloadAndExtractTarGz(url string, targetDir string) error {
	// Step 1: Download the .tar.gz file
	resp, err := http.Get(url)
	if err != nil {
		return fmt.Errorf("failed to download file: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("failed to download file: server returned status %d", resp.StatusCode)
	}

	// Create a temporary file to store the downloaded .tar.gz
	tmpFile, err := os.CreateTemp("", "downloaded-*.tar.gz")
	if err != nil {
		return fmt.Errorf("failed to create temporary file: %w", err)
	}
	defer os.Remove(tmpFile.Name()) // Cleanup the temp file after extraction
	defer tmpFile.Close()

	// Copy the downloaded content into the temp file
	_, err = io.Copy(tmpFile, resp.Body)
	if err != nil {
		return fmt.Errorf("failed to write downloaded content to temp file: %w", err)
	}

	// Step 2: Open the .tar.gz file
	tmpFile.Seek(0, io.SeekStart) // Reset file pointer to the beginning

	gzipReader, err := gzip.NewReader(tmpFile)
	if err != nil {
		return fmt.Errorf("failed to open gzip reader: %w", err)
	}
	defer gzipReader.Close()

	// Step 3: Extract the contents of the .tar.gz file
	tarReader := tar.NewReader(gzipReader)

	// Ensure target directory exists
	if err := os.MkdirAll(targetDir, 0755); err != nil {
		return fmt.Errorf("failed to create target directory: %w", err)
	}

	for {
		header, err := tarReader.Next()
		if err == io.EOF {
			// End of archive
			break
		}
		if err != nil {
			return fmt.Errorf("failed to read tar archive: %w", err)
		}

		// Construct the full file path in the target directory
		extractedFilePath := filepath.Join(targetDir, header.Name)

		// Handle directory entries in the tarball
		if header.Typeflag == tar.TypeDir {
			if err := os.MkdirAll(extractedFilePath, os.FileMode(header.Mode)); err != nil {
				return fmt.Errorf("failed to create directory: %w", err)
			}
			continue
		}

		// Handle regular file entries in the tarball
		if err := extractFileFromTar(tarReader, extractedFilePath, header.Mode); err != nil {
			return fmt.Errorf("failed to extract file %s: %w", header.Name, err)
		}
	}

	fmt.Println("Extraction completed successfully.")
	return nil
}

// extractFileFromTar writes the file content from tarReader to the specified file path
func extractFileFromTar(tarReader *tar.Reader, filePath string, mode int64) error {
	// Create the file
	dir := filepath.Dir(filePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create directory %s: %w", dir, err)
	}

	file, err := os.Create(filePath)
	if err != nil {
		return fmt.Errorf("failed to create file: %w", err)
	}
	defer file.Close()

	// Set the file's permissions
	if err := os.Chmod(filePath, os.FileMode(mode)); err != nil {
		return fmt.Errorf("failed to set file permissions: %w", err)
	}

	// Copy the file content from the tarReader to the file
	_, err = io.Copy(file, tarReader)
	if err != nil {
		return fmt.Errorf("failed to copy file content: %w", err)
	}

	return nil
}

// DownloadAndExtractZip downloads a .zip file from the URL and extracts it to the given directory.
func DownloadAndExtractZip(url, targetDir string) error {
	// Step 1: Download the .zip file
	resp, err := http.Get(url)
	if err != nil {
		printClearError("Erreur de téléchargement du fichier")
		return fmt.Errorf("failed to download file: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("failed to download file: server returned status %d", resp.StatusCode)
	}

	// Create a temporary file to store the downloaded .zip
	tmpFile, err := os.CreateTemp("", "downloaded-*.zip")
	if err != nil {
		return fmt.Errorf("failed to create temporary file: %w", err)
	}
	defer os.Remove(tmpFile.Name()) // Cleanup the temp file after extraction
	defer tmpFile.Close()

	// Copy the downloaded content into the temp file
	_, err = io.Copy(tmpFile, resp.Body)
	if err != nil {
		return fmt.Errorf("failed to write downloaded content to temp file: %w", err)
	}

	// Step 2: Open the .zip file
	tmpFile.Seek(0, io.SeekStart) // Reset file pointer to the beginning

	zipReader, err := zip.OpenReader(tmpFile.Name())
	if err != nil {
		return fmt.Errorf("failed to open zip file: %w", err)
	}
	defer zipReader.Close()

	// Ensure target directory exists
	if err := os.MkdirAll(targetDir, 0755); err != nil {
		return fmt.Errorf("failed to create target directory: %w", err)
	}

	// Step 3: Extract the contents of the .zip file
	for _, file := range zipReader.File {
		// Construct the full file path in the target directory
		extractedFilePath := filepath.Join(targetDir, file.Name)

		// Ensure the directory exists for the file
		if file.FileInfo().IsDir() {
			// Create directory if it's a directory entry in the zip
			if err := os.MkdirAll(extractedFilePath, os.FileMode(file.Mode())); err != nil {
				return fmt.Errorf("failed to create directory %s: %w", extractedFilePath, err)
			}
			continue
		}

		// Handle regular file entries in the zip archive
		if err := extractZipFile(file, extractedFilePath); err != nil {
			return fmt.Errorf("failed to extract file %s: %w", file.Name, err)
		}
	}

	fmt.Println("Extraction completed successfully.")
	return nil
}

// extractZipFile extracts a single file from the zip archive.
func extractZipFile(file *zip.File, filePath string) error {
	// Ensure the directory exists for the file
	dir := filepath.Dir(filePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create directory %s: %w", dir, err)
	}

	// Open the file inside the zip archive
	zipFileReader, err := file.Open()
	if err != nil {
		return fmt.Errorf("failed to open zip file content %s: %w", file.Name, err)
	}
	defer zipFileReader.Close()

	// Create the extracted file
	extractedFile, err := os.Create(filePath)
	if err != nil {
		return fmt.Errorf("failed to create file %s: %w", filePath, err)
	}
	defer extractedFile.Close()

	// Copy the file content from the zip reader to the new file
	_, err = io.Copy(extractedFile, zipFileReader)
	if err != nil {
		return fmt.Errorf("failed to copy file content to %s: %w", filePath, err)
	}

	// Set the file's permissions (optional, can use original permissions from the zip entry)
	if err := os.Chmod(filePath, os.FileMode(file.Mode())); err != nil {
		return fmt.Errorf("failed to set file permissions for %s: %w", filePath, err)
	}

	return nil
}
