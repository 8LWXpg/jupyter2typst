pushd
cd $PSScriptRoot

ls *.typ -e base.typ | % {
    "#import `"$($_.Name)`": *", (cat ./base.typ) | typst c - "./out/$($_.BaseName).png"
}
oxipng ./out/*.png

popd