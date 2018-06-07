package deno

import (
	"io/ioutil"
	"path/filepath"
)

var (
	_ = ioutil.Discard
)

const (
	distPath = "dist/"
	denoPath = ""
)

func loadAsset(path string) ([]byte, error) {
	path = filepath.Join(distPath, path)
	data, err := ioutil.ReadFile(path)
	check(err)
	return data, err
}

func loadAssetString(path string) (string, error) {
	data, err := loadAsset(path)
	check(err)
	return string(data), nil
}

func loadAssetModule(path string) ([]byte, error) {
	return loadAsset(path)
}
