//go:build !static && !pkgconfig

package monosecret

import (
	"runtime"
	"testing"
)

func TestLibraryNamesPreferLibmonosecretAndRetainPre020Fallback(t *testing.T) {
	names := libNames()
	want := []string{"libmonosecret_ffi.so", "libmonosecret_ffi.so"}
	switch runtime.GOOS {
	case "darwin":
		want = []string{"libmonosecret_ffi.dylib", "libmonosecret_ffi.dylib"}
	case "windows":
		// Cargo emits monosecret_ffi.dll into target/, while packaged assets use
		// libmonosecret_ffi.dll. The final name preserves pre-0.20 compatibility.
		want = []string{"libmonosecret_ffi.dll", "monosecret_ffi.dll", "monosecret_ffi.dll"}
	}
	if len(names) != len(want) {
		t.Fatalf("library names = %v, want %v", names, want)
	}
	for i, name := range want {
		if names[i] != name {
			t.Fatalf("library names = %v, want %v", names, want)
		}
	}
}
