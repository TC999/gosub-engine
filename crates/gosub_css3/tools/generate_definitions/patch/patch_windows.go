package patch

import (
	"fmt"
	"generate_definitions/utils"
	"os"
	"os/exec"
)

// PatchFile applies a patch file to the target file on Windows
// This implementation uses Git's patch functionality which is available in Git for Windows
func PatchFile(path string, patchPath string) (string, error) {
	// Try using git apply first, which is more portable on Windows
	cmd := exec.Command("git", "apply", "--ignore-whitespace", patchPath)
	cmd.Dir = "."
	
	output, err := cmd.CombinedOutput()
	if err != nil {
		// If git apply fails, try traditional patch command (available in Git Bash/MSYS2)
		cmd = exec.Command("patch", path, patchPath)
		output, err = cmd.CombinedOutput()
		if err != nil {
			return "", fmt.Errorf("Failed to apply patch %v: %v\nOUTPUT:\n%v", path, err, string(output))
		}
	}

	// Check if the file was deleted (common in patch operations)
	if _, err := os.Stat(path); os.IsNotExist(err) {
		if os.WriteFile(path, []byte("<deleted>"), 0644) != nil {
			return "", fmt.Errorf("Failed to create del file %v: %v", path, err)
		}
		return "<deleted>", nil
	}

	return utils.ComputeGitBlobSHA1(path)
}
