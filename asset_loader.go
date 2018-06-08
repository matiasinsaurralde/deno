package deno

import (
	"io/ioutil"
	"path/filepath"
)

const (
	distPath = "dist/"
	denoPath = ""
)

func loadAsset(path string) ([]byte, error) {
	path = filepath.Join(distPath, path)
	data, err := ioutil.ReadFile(path)
	return data, err
}

func loadAssetString(path string) (string, error) {
	data, err := loadAsset(path)
	// check(err)
	return string(data), err
}

func loadAssetModule(path string) ([]byte, error) {
	return loadAsset(path)
}
