pushd
cd $PSScriptRoot

(cat ../template/template.typ -raw) -replace [regex]::Escape('#let template(body) = {'),
	"#let template(body) = {`n  set document(date: none)" `
	> ./template.typ

ls *.ipynb | % {
	cargo run -- $_.FullName
	typst c "$($_.DirectoryName)/$($_.BaseName).typ"
}

popd