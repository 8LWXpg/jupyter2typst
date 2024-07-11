pushd
cd $PSScriptRoot

ls *.ipynb | % {
	cargo run -- $_.FullName
	typst c "$($_.DirectoryName)/$($_.BaseName).typ"
}

popd