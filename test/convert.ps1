pushd
cd $PSScriptRoot

foreach($e in (ls *.ipynb)){
	../target/debug/jupyter2typst $e.FullName
	typst c "$($e.DirectoryName)/$($e.BaseName).typ"
}

popd